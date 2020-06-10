use chrono::NaiveDateTime;
use crate::schema::users;
use crate::schema::session;
use crate::jwt;
use crate::schema::session::dsl;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    //#[serde(skip)] // don't show id
    pub id: i32,
    pub email: String,
    #[serde(skip)] // don't show password
    pub password: String,
    pub created_at: NaiveDateTime,
    pub confirmed: i32,
    pub role: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Associations, AsChangeset, Identifiable)]
#[belongs_to(parent="User", foreign_key="user_id")]
#[table_name = "session"]
pub struct Session {
    pub id: i32,
    pub refresh_token: String,
    pub refresh_token_expire_at: NaiveDateTime,
    pub user_id: i32
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "session"]
pub struct NewSession {
    pub refresh_token: String,
    pub refresh_token_expire_at: NaiveDateTime,
    pub user_id: i32
}

use bcrypt::{hash, DEFAULT_COST};
use crate::db::establish_connection;
use chrono::{Local, Duration};
use crate::errors::MyStoreError;

impl User {
    pub fn create(register_user: RegisterUser) ->
     Result<User, MyStoreError> {
        use diesel::RunQueryDsl;

        let connection = establish_connection();

        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local()
            })
            .get_result(&connection)?)
    }

    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }

    pub fn make_confirmation(info: jwt::ConfirmInfo) -> Result<(), MyStoreError> {
        use diesel::{QueryDsl, RunQueryDsl};

        let connection = establish_connection();

        let mut user = users::table.find(info.user_id).first::<User>(&connection)?;
        
        user.confirmed = 1;

        diesel::update(users::table.find(info.user_id)).set(user).execute(&connection)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    pub fn validates(self) -> Result<RegisterUser, MyStoreError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(
                MyStoreError::PasswordNotMatch(
                    "Passwords does not match".to_string())
            )
        }
    }
}

impl Session {
    pub fn create(new_session: NewSession) -> Result<Session, MyStoreError> {
        use diesel::RunQueryDsl;

        let connection = establish_connection();

        Ok(diesel::insert_into(session::table).values(new_session).get_result(&connection)?)
    }
}


#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct Tokens {
    pub access: Option<String>,
    pub refresh: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct AccessToken {
    pub access: String
}

impl From<Tokens> for AccessToken {
    fn from(tokens: Tokens) -> Self {
        AccessToken {
            access: tokens.access.expect("access")
        }
    }
}

impl Tokens {
    pub fn refresh(old_tokens: Tokens) -> Result<Tokens, MyStoreError> {
        use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
        use crate::schema::session::dsl::refresh_token;

        if old_tokens.refresh.is_none() {
            return Err(MyStoreError::CustomError("Wrong request".to_string()));
        }

        let connection = establish_connection();

        let mut session_records = session::table
                        .filter(refresh_token.eq(&old_tokens.refresh.expect("refresh")))
                        .load::<Session>(&connection)?;
        let session = session_records.pop().ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;

        if session.refresh_token_expire_at >= Local::now().naive_local() {
            let refresh = libreauth::key::KeyBuilder::new().generate().as_base32();

            let user = users::table
                                .find(session.user_id).first::<User>(&connection)?;
            let access = jwt::create_token(&user.email, session.id)?;

            let new_session = NewSession{
                refresh_token: refresh.clone(),
                refresh_token_expire_at: (Local::now().naive_local() + Duration::hours(1)),
                user_id: user.id
            };

            diesel::update(dsl::session.find(session.id))
                .set(new_session)
                .execute(&connection)?;
            Ok(Tokens{
                access: serde::export::Some(access),
                refresh: serde::export::Some(refresh)})
        } else {
            Err(MyStoreError::TokenError("Refresh token expired".to_string()))
        }
    }

    pub fn validate(&self) -> Result<i32, MyStoreError> {
        use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
        use crate::schema::session::dsl::refresh_token;
        
        println!("all ok");

        if self.access.is_none() {
            return Err(MyStoreError::CustomError("Wrong request".to_string()));
        }

        println!("poprworwp");

        let decoded_claims = jwt::decode_token(self.access.as_ref().expect("access"))?;
        if decoded_claims.exp < Local::now().timestamp() {
            return Err(MyStoreError::TokenExpired("Access token expired".to_string()));
        }
        println!("Still ok");

        let connection = establish_connection();
        let mut session_records = session::table
                        .filter(crate::schema::session::columns::id.eq(&decoded_claims.session_id))
                        .load::<Session>(&connection)?;
        let session = session_records.pop().ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        println!("Still ok2");
        let user = users::table
                        .find(session.user_id).first::<User>(&connection)?;
        Ok(user.role)
    }
}

pub fn set_role(email: &str) -> Result<(), MyStoreError> {
    use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
    use crate::schema::users::dsl;

    let connection = establish_connection();

    let mut records = users::table
                        .filter(dsl::email.eq(&email))
                        .load::<User>(&connection)?;
    
    let mut user = records
                    .pop()
                    .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        
    user.role = 1;

    diesel::update(users::table.find(user.id)).set(user).execute(&connection)?;

    Ok(())
}

impl AuthUser {
    pub fn login(&self) -> Result<Tokens, MyStoreError> {
        use bcrypt::verify;
        use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
        use crate::schema::users::dsl::email;

        let connection = establish_connection();

        let mut records = users::table
                            .filter(email.eq(&self.email))
                            .load::<User>(&connection)?;

        let user = records
                        .pop()
                        .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        
        if user.confirmed == 0 {
            return Err(MyStoreError::NotConfirmed(user.email));
        }
        
        let verify_password = verify(&self.password, &user.password)
                                .map_err( |error| {
                                    MyStoreError::WrongPassword(
                                        "Wrong password".to_string()
                                    )
                                })?;
        
        
        let refresh = libreauth::key::KeyBuilder::new().generate().as_base32();
        let session = Session::create(NewSession {
            refresh_token: refresh.clone(),
            refresh_token_expire_at: (Local::now().naive_local() + Duration::hours(1)),
            user_id: user.id
        })?;
        let access = jwt::create_token(&user.email, session.id)?;
        if verify_password {
            Ok(Tokens{access: serde::export::Some(access), refresh: serde::export::Some(refresh)})
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password".to_string()
            ))
        }
    }
}