use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)] // don't show id
    pub id: i32,
    pub email: String,
    pub company: String,
    #[serde(skip)] // don't show password
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

use bcrypt::{hash, DEFAULT_COST};
use crate::db::establish_connection;
use chrono::Local;
use crate::errors::MyStoreError;

impl User {
    pub fn create(register_user: RegisterUser) ->
     Result<User, MyStoreError> {
        use diesel::RunQueryDsl;

        let connection = establish_connection();

        Ok(diesel.inser_into(users::table)
            .values(NewUser {
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local()
            })
            .get_result(connection)?)
    }

    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {
    pub fn login(&self) -> Result<User, MyStoreError> {
        use bcrypt::verify;
        use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
        use crate::schema::users::dsl::email;

        let connection = establish_connection();

        let mut records = users::table
                            .filter(email.eq(&self.email))
                            .load<User>(connection)?;

        let user = records
                        .pop()
                        .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        
        let verify_password = verify(&self.password, &user.password)
                                .map_err( |error| {
                                    MyStoreError::WrongPassword(
                                        "Wrong password".to_string()
                                    )
                                })?;
        
        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password".to_string()
            ))
        }
    }
}

use actix_web::{ FromRequest, HttpRequest, dev };
use actix_web::middleware::identity::Identity;
use crate::jwt::{ decode_token, SlimUser };
pub type LoggedUser = SlimUser;

use hex;
use csrf_token::CsrfTokenGenerator;

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Config = ();
    type Future = Result<Self, HttpResponse>;

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let generator = 
            req.app_data::<CsrfTokenGenerator>()
            .ok_or(HttpResponse::InternalServerError())?;

        let csrf_token =
            req
                .headers()
                .get("x-csrf-token")
                .ok_or(HttpResponse::Unauthorized())?;

        let decoded_token =
            hex::decode(&csrf_token)
                .map_err(|error| HttpResponse::InternalServerError().json(error.to_string()))?;

        generator
            .verify(&decoded_token)
            .map_err(|_| HttpResponse::Unauthorized())?;

        if let Some(identity) = Identity::from_request(req, payload)?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }  
        Err(HttpResponse::Unauthorized().into())
    }
}