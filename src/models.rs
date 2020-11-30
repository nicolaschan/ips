use super::schema::hits;
use diesel::sql_types::*;

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "hits"]
pub struct Hit {
    pub id: i32,
    pub ip_addr: String,
    pub timestamp: String
}

#[derive(Insertable)]
#[table_name="hits"]
pub struct NewHit {
    pub ip_addr: String,
    pub timestamp: String,
}

#[derive(Queryable, QueryableByName, Debug)]
pub struct Score {
    #[sql_type = "Text"]
    pub ip_addr: String,
    #[sql_type = "Integer"]
    pub count: i32,
}


