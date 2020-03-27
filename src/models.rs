use crate::schema::products;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use crate::db::establish_connection;
use crate::schema::products::dsl;
use crate::diesel::ExpressionMethods;

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub category: Option<String>,
}

impl Product {
    pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();

        products::table.find(id).first(&connection)
    }

    pub fn delete(product_id: &i32) -> Result<Product, diesel::result::Error> {

        let connection = establish_connection();
        let deleted_product = Product::find(&product_id)?;
        let result = diesel::delete(dsl::products)
            .filter(dsl::id.eq(product_id))
            .execute(&connection)?;
        Ok(deleted_product)
    }

    pub fn update(id: &i32, new_product: &NewProduct) -> Result<Product, diesel::result::Error> {
        let connection = establish_connection();
        diesel::update(dsl::products.find(id))
            .set(new_product)
            .execute(&connection)?;
        let updated_product = Product::find(&id)?;
        Ok(updated_product)
    }
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
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
pub struct ProductList {
    pub products: Vec<Product>,
    pub count: usize,
}

impl ProductList {
    pub fn list(query: ListQuery) -> Self {
        use crate::schema::products::dsl::*;

        let connection = establish_connection();

        let count = 
            products
                .load::<Product>(&connection)
                .expect("Error counting products").len();

        let result = 
            products
                .order(id)
                .limit(query.limit.unwrap_or(std::i64::MAX))
                .offset(query.offset.unwrap_or(0))
                .load::<Product>(&connection)
                .expect("Error loading products");
        ProductList{products: result, count}
    }
}