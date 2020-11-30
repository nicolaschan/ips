#[macro_use]
extern crate diesel;

use self::models::*;
use self::diesel::prelude::*;

pub mod schema;
pub mod models;

pub fn create_hit(conn: &SqliteConnection, ip_addr: &String, timestamp: std::time::Duration) -> usize {
    use schema::hits;
    let new_hit = NewHit {
        ip_addr: ip_addr.clone(),
        timestamp: timestamp.as_millis().to_string()
    };
    diesel::insert_into(hits::table)
        .values(&new_hit)
        .execute(conn)
        .expect("Error saving new hit")
}
