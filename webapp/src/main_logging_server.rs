#[macro_use] extern crate nickel;
extern crate chrono;

use nickel::Nickel;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;
// use std::fmt::Debug;




fn main() {
    // pt1(say_hello, "WORK!");
    // pt1();
    // pt2a();
    // pt2b("log3.txt");
    // pt3("log4.txt");
}

// Part1
fn say_hello(hell: &str) -> &'static str {
    "Hello dear world! {hell}"
}
fn pt1(f: fn(&str)-> &'static str, arg: &'static str) {
    let mut server = Nickel::new();
    println!("YES");

    server.utilize(router! {
        get "**" => |_req, _res| {
            f(arg);
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
fn pt2b(filename: &str) -> std::io::Result<()>{
    
    // let mut st = String::new();
    let mut f = File::options().append(true).create(true).open(filename)?;
    // f.read_to_string(&mut st);
    // st = st + "\n" + &local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    writeln!(f, "{}", format_time())?;

    // let mut f2 = File::options().append(true).create(true).open("log3.txt")?;
    // writeln!(f2, "{}", &local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string())?;
    Ok(())
}
fn format_time() -> String{
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%Y"));
    local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string()
}

// Part3
fn pt3(filename: &'static str){
    pt1(do_log, filename);

}
fn do_log(filename: &str) -> &'static str{
    match pt2b(filename) {
        Ok(..) => "File created!",
        Err(e) => "Error: {e}"
    }
}

// Part4
fn pt4(){
    
}

