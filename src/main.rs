extern crate clap;
use std::io;
use clap::{Arg, App};

struct Credentials <'a> {
    username: &'a str,
    pass: &'a str
}

fn get_credentials() {
    let mut u_details: Credentials = Credentials {username: "PLACEHOLDER", pass: "PLACEHOLDER"};
    println!("Enter your username below: ");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("No Input!");
    temp = temp.trim().to_string();
    u_details.username = &temp;
    println!("Enter your username below: ");
    let mut pass: String = String::new();
    io::stdin().read_line(&mut pass).expect("No Input!");
    pass = pass.trim().to_string();
    u_details.pass = &pass;
}

fn main() {
    let _init = App::new("Dev-Upload!")
    .version("0.1.0")
    .author("dev-harsh1998 <dev-harsh1998@hotmail.com>")
    .about("A Handy tool for Android developer community to ease file upload to ftp clients.")
    .arg(Arg::with_name("basketbuild")
    .short("b")
    .long("basketbuild")
    .required(false)
    .takes_value(true))
    .arg(Arg::with_name("androidfilehost")
    .short("a")
    .long("androidfilehost")
    .required(false)
    .takes_value(true))
    .get_matches();
}
