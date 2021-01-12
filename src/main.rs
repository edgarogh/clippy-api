#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::Json;

lazy_static::lazy_static! {
    static ref QUOTES: Box<[&'static str]> = include_str!("../clippy.txt").lines().collect();
}

#[get("/v1/quote.txt")]
fn v1_quote() -> &'static str {
    use rand::seq::SliceRandom;

    QUOTES.choose(&mut rand::thread_rng()).unwrap()
}

fn main() {
    rocket::ignite().mount("/clippy", routes![v1_quote]).launch();
}
