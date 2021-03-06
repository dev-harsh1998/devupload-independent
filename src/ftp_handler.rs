extern crate ftp;
use ftp::types::FileType;
use ftp::FtpStream;
use std::fs::metadata;
use std::fs::File;
use std::path::Path;
mod credential_setup;

pub fn upload_file(_choice: u8, _file_path: String) {
    let _url = if _choice == 1 {
        "basketbuild.com:21".to_string()
    } else {
        "uploads.androidfilehost.com:21".to_string()
    };
    println!("\nType in your credentials for {}", _url);
    let _credentials: (String, String) = credential_setup::get_credentials();
    if _credentials.0 == "-1" || _credentials.1 == "-1" {
        // We shouldn't be here.
        panic!("[CRD-DBG]This isn't supposed to happen\n");
    }
    //
    // File handling
    //
    let _file_name = Path::new(&_file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let mut file_stream = File::open(&_file_path).unwrap();
    // Compute File Size
    let mut _file_size = metadata(&_file_path);
    let _file_size: f64 = (_file_size.unwrap().len()) as f64 / 1_000_000_f64;
    //
    // FTP begin
    //
    let mut _ftp_stream = FtpStream::connect(_url.as_str())
        .unwrap_or_else(|err| panic!("Couldn't connect to remote server err log: {}", err));
    _ftp_stream
        .login(_credentials.0.as_str(), _credentials.1.as_str())
        .unwrap_or_else(|err| panic!("Can't login here's the response: {}", err));
    _ftp_stream
        .transfer_type(FileType::Binary)
        .expect("Can't set to binary upload mode");
    // Display File size
    println!(
        "\nStarting to upload file: {} , with size: {} Megabytes",
        _file_name, _file_size
    );
    let _success = _ftp_stream.put(_file_name, &mut file_stream);
    match _success {
        Ok(_) => {
            println!("File has been uploaded successfully!");
        }
        Err(e) => {
            println!("There was an error uploading the file {}", e);
        }
    };
    let _ = _ftp_stream.quit();
}
