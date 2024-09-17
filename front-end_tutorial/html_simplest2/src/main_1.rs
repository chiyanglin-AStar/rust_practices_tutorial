use rocket::response::content;

fn main() {
let response = content::RawHtml("<h1>Hello, world!</h1>");
}