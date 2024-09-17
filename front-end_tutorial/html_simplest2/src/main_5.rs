#[macro_use] extern crate rocket;

use std::fs::read_to_string;
use rocket::response::content::RawHtml;
//use rocket::response::content;
use std::path::PathBuf;


#[get("/")]
async fn index() -> Html<String> {
    // Path to your index.html
    let path = PathBuf::from("index.html");
    //let response = content::RawHtml;
    // Read the file content into a string
    let html_content = read_to_string(path)
        .unwrap_or_else(|_| "<h1>Error: Could not read index.html</h1>".to_string());
    
    //response(html_content)
    RawHtml(html_content)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}