extern crate clap;
extern crate ftp;
use clap::{App, Arg};
use ftp::FtpStream;
use ftp::types::FileType;
use std::io;
use std::path::Path;
use std::fs::File;

fn get_credentials() -> (String, String) {
    println!("Enter your username below: ");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("No Input!");
    temp = temp.trim().to_string();
    println!("Enter your password below: ");
    let mut pass: String = String::new();
    io::stdin().read_line(&mut pass).expect("No Input!");
    pass = pass.trim().to_string();
    (temp, pass)
}

fn upload_file(_choice: u8, _file_path: String) {
    let mut _url: String = String::new();
    if _choice == 1 {
        _url = "basketbuild.com:21".to_string();
    } else {
        _url = "uploads.androidfilehost.com:21".to_string();
    }
    println!("\nType in your credentials for {}", _url);
    let _credentials: (String, String) = get_credentials();
    //
    // File handling
    //
    let _file_name = Path::new(& _file_path).file_name()
        .unwrap().to_str().unwrap();
    let mut file_stream = File::open(&_file_path).unwrap();
    //
    // FTP begin
    //
    let mut _ftp_stream = FtpStream::connect(_url.as_str()).unwrap();
    let _ = _ftp_stream
        .login(_credentials.0.as_str(), _credentials.1.as_str())
        .expect("Couldn't login!");
    _ftp_stream.transfer_type(FileType::Binary)
        .expect("Can't set to binary upload mode");
    let _ = _ftp_stream.put(_file_name, &mut file_stream);
    let _ = _ftp_stream.quit();
}

fn main() {
    let _init = App::new("Dev-Upload!")
        .version("0.1.0")
        .author("dev-harsh1998 <dev-harsh1998@hotmail.com>")
        .about("A Handy tool for Android developer community to ease file upload to ftp clients.")
        .arg(
            Arg::with_name("File Path")
                .short("f")
                .long("file-path")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .help("Specify file path like `devupload -f /path/to/your/file.extention`"),
        )
        .arg(
            Arg::with_name("basketbuild")
                .short("b")
                .long("basketbuild")
                .required(false)
                .takes_value(false)
                .help("Specify `-b` flag to trigger uploader to upload to basketbuild"),
        )
        .arg(
            Arg::with_name("androidfilehost")
                .short("a")
                .long("androidfilehost")
                .required(false)
                .takes_value(false)
                .help("Specify `-a` flag to trigger uploader to upload to androidfilehost"),
        )
        .get_matches();
    let mut _dec: u8 = 0;
    if _init.is_present("basketbuild") {
        _dec = 1;
    } else if _init.is_present("androidfilehost") {
        _dec = 2;
    } else {
        _dec = 0;
    }
    upload_file(_dec, _init.value_of("File Path").unwrap().to_string());
}
