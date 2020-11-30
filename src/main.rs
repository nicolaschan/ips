#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use crate::rocket_contrib::databases::diesel::RunQueryDsl;
use rocket_contrib::databases::diesel;

use std::time::{SystemTime, UNIX_EPOCH};

use ips::create_hit;
use ips::models::Score;

struct XRealIP(String);

// Source: https://stackoverflow.com/a/64829582
impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for XRealIP {
    type Error = std::convert::Infallible;
    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let ip_addr = request.client_ip();
        match ip_addr {
            Some(ip_addr) => rocket::Outcome::Success(XRealIP(ip_addr.to_string())),
            None => rocket::Outcome::Success(XRealIP("0.0.0.0".to_string()))
        }
    }
}

#[database("sqlite_hits")]
struct HitsDbConn(diesel::SqliteConnection);

fn display_scores(scores: Vec<Score>) -> String {
    scores.into_iter()
        .map(|r| r.ip_addr + ": " + &r.count.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

#[get("/")]
fn index(conn: HitsDbConn, ip_addr: XRealIP) -> String {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    create_hit(&conn, &ip_addr.0, time);
    let all_time: Vec<Score> = diesel::sql_query("
        SELECT ip_addr, COUNT(*) AS count 
        FROM hits 
        GROUP BY ip_addr
        ORDER BY count DESC
        LIMIT 10
    ").load(&conn.0).unwrap();
    let recent: Vec<Score> = diesel::sql_query("
        SELECT ip_addr, COUNT(*) AS count 
        FROM hits 
        GROUP BY ip_addr
        ORDER BY MAX(timestamp) DESC
        LIMIT 5
    ").load(&conn.0).unwrap();
    let past_ten_min: Vec<Score> = diesel::sql_query("
        SELECT ip_addr, COUNT(*) AS count 
        FROM hits 
        WHERE datetime(timestamp/1000, 'unixepoch') >= datetime('now', '-10 minutes')
        GROUP BY ip_addr
        ORDER BY count DESC
        LIMIT 10
    ").load(&conn.0).unwrap();

    let lines = vec![
        "Your IP: ".to_owned() + &ip_addr.0,
        "\nRecent (last 5)".to_owned(),
        display_scores(recent),
        "\nAll Time (top 10)".to_owned(),
        display_scores(all_time),
        "\nPast 10 Minutes (top 10)".to_owned(),
        display_scores(past_ten_min),
    ];
    lines.join("\n")
}

fn main() {
    rocket::ignite()
        .attach(HitsDbConn::fairing())
        .mount("/", routes![index]).launch();
}
