extern crate clap;
use std::io;
use clap::{Arg, App};

fn get_credentials() -> (String, String) {
    println!("Enter your username below: ");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("No Input!");
    temp = temp.trim().to_string();
    println!("Enter your username below: ");
    let mut pass: String = String::new();
    io::stdin().read_line(&mut pass).expect("No Input!");
    pass = pass.trim().to_string();
    (temp, pass)
}

fn main() {
    let _init = App::new("Dev-Upload!")
    .version("0.1.0")
    .author("dev-harsh1998 <dev-harsh1998@hotmail.com>")
    .about("A Handy tool for Android developer community to ease file upload to ftp clients.")
    .arg(Arg::with_name("File Path")
    .short("f")
    .long("file-path")
    .required(true)
    .multiple(true)
    .takes_value(true)
    .help("Specify file path like `devupload -f /path/to/your/file.extention`"))
    .arg(Arg::with_name("basketbuild")
    .short("b")
    .long("basketbuild")
    .required(false)
    .takes_value(false)
    .help("Specify `-b` flag to trigger uploader to upload to basketbuild"))
    .arg(Arg::with_name("androidfilehost")
    .short("a")
    .long("androidfilehost")
    .required(false)
    .takes_value(false)
    .help("Specify `-a` flag to trigger uploader to upload to androidfilehost"))
    .get_matches();
    let mut _dec: u8 = 0; 
    if _init.is_present("basketbuild"){
        _dec = 1;
    }
    else if _init.is_present("androidfilehost"){
        _dec = 2;
    } else {
        _dec = 0;
    }
}
