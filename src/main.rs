extern crate clap;
use std::io;
use clap::{Arg, App};

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
