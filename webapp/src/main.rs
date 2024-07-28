#[macro_use] extern crate nickel;
extern crate chrono;

use nickel::Nickel;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;




fn main() {
    // pt1();
    // pt2a();
    // pt2b();
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
// A
fn pt2a() {
    match log_something("log.txt", "ITS ALIVE!!!") {
        Ok(_) => (println!("File created!")),
        Err(_) => ()
    }
}
fn log_something(filename: &str, string: &str) -> io::Result<()> {
    let mut f = File::create(&filename).expect("Failure to create file!");
    let _ = f.write(string.to_string().as_bytes());
    Ok(())
}
// B
fn pt2b() -> std::io::Result<()>{
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%Y"));
    // let mut st = String::new();
    let mut f = File::options().append(true).create(true).open("log2.txt")?;
    // f.read_to_string(&mut st);
    // st = st + "\n" + &local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    writeln!(f, "{}", &local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string())?;

    // let mut f2 = File::options().append(true).create(true).open("log3.txt")?;
    // writeln!(f2, "{}", &local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string())?;
    Ok(())
}

// Part3



