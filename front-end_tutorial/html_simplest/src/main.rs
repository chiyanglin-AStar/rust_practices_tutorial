// main.rs

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<!DOCTYPE html>
    <html>
    <head>
        <title>Simple HTML</title>
    </head>
    <body>
        <h1>Hello, world!</h1>
        <p>This is a simple HTML rendering example using Rocket in Rust.</p>
    </body>
    </html>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}