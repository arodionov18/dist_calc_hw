use crate::schema::products;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::db::establish_connection;
use crate::schema::products::dsl;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub category: Option<String>,
}

impl Product {
    pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();

        products::table.find(id).first(&connection)
    }

    pub fn delete(id: &i32) -> Result<(), diesel::result::Error> {
        let connection = establish_connection();
        diesel::delete(dsl::products.find(id))
            .execute(&connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_product: &NewProduct) -> Result<(), diesel::result::Error> {
        let connection = establish_connection();
        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(&connection)?;
        Ok(())
    }
}

impl NewProduct {
    pub fn create(&self) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();
        diesel::insert_into(products::table)
            .values(self)
            .get_result(&connection)
    }
}

#[derive(Serialize, Deserialize)] 
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        use crate::schema::products::dsl::*;

        let connection = establish_connection();

        let result = 
            products
                .load::<Product>(&connection)
                .expect("Error loading products");

        // We return a value by leaving it without a comma
        ProductList(result)
    }
}