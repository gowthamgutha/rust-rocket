#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/sum/<a>/<b>")]
fn sum(a: i32, b: i32) -> String {
    format!("{} + {} = {}", a, b, a+b)
}


fn main() {
    rocket::ignite().mount("/", routes![hello, sum])
    .launch();
}