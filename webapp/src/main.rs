#[macro_use] extern crate nickel;

use nickel::Nickel;
use std::fs::File;



fn main() {
    // pt1();
    pt2a();
}

// Part1
fn say_hello() -> &'static str {
    "Hello dear world!"
}
fn pt1() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767");
}

// Part2
fn pt2a() {
    match File::create("foo.txt") {
        Ok(_) => println!("File created!"),
        Err(_) => println!("Error: could not create file.")
    }
}






