#   crate example

ref : https://medium.com/learning-rust/rust-lets-get-it-started-bdd8de58178d

##  Think we run

cargo new --bin phrases

cargo new phrases/greetings

## put crate in github example

// -- Cargo.toml --
[dependencies]

// 01. Get the latest commit on the master branch
rocket = { git = "https://github.com/SergioBenitez/Rocket" }

// 02. Get the latest commit of a specific branch
rocket = { git = "https://github.com/SergioBenitez/Rocket", branch = "v0.3" }

// 03. Get a specific tag
rocket = { git = "https://github.com/SergioBenitez/Rocket", tag = "v0.3.2" }

// 04. Get a specific revision (on master or any branch, according to rev)
rocket = { git = "https://github.com/SergioBenitez/Rocket", rev = "8183f636305cef4adaa9525506c33cbea72d1745" }


