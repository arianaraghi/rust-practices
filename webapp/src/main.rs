#[macro_use] extern crate nickel;

use nickel::Nickel;
use std::io::prelude::*;
use std::fs::File;
use std::io;




fn main() {
    // pt1();
    // pt2a();
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
    match log_something("log.txt", b"ITS ALIVE!!!") {
        Ok(_) => (println!("File created!")),
        Err(_) => ()
    }
}
fn log_something(filename: &str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = File::create(&filename).expect("Failure to create file!");
    let _ = f.write( string);
    Ok(())
}






