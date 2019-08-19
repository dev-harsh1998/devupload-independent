extern crate clap;
use clap::{App, Arg};
mod ftp_handler;

fn main() {
    let _init = App::new("Dev-Upload!")
        .version("1.1.0")
        .author("dev-harsh1998 <dev-harsh1998@hotmail.com>")
        .about("A Handy tool for Android developer community to ease file upload to ftp clients.")
        .arg(
            Arg::with_name("File Path")
                .short("f")
                .long("file-path")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .help("Specify file path like `devupload -a -f path/to/your/file.extention` to trigger afh upload"),
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
    if _init.is_present("basketbuild") {
        ftp_handler::upload_file(1, _init.value_of("File Path").unwrap().to_string());
    } else if _init.is_present("androidfilehost") {
        ftp_handler::upload_file(2, _init.value_of("File Path").unwrap().to_string());
    } else {
        panic!("WTH are you trying to do?");
    }
}
