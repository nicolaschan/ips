#[macro_use]
extern crate diesel;

use self::models::*;
use self::diesel::prelude::*;

pub mod schema;
pub mod models;

pub fn create_hit(conn: &PgConnection, ip_addr: &str, timestamp: std::time::SystemTime) -> usize {
    use schema::hits;
    let new_hit = NewHit {
        ip_addr: ip_addr.to_string(),
        timestamp
    };
    diesel::insert_into(hits::table)
        .values(&new_hit)
        .execute(conn)
        .expect("Error saving new hit")
}
