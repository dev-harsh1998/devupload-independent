extern crate ftp;
use ftp::FtpStream;
use ftp::types::FileType;
use std::path::Path;
use std::fs::File;
use std::fs::metadata;
mod credential_setup;

pub fn upload_file(_choice: u8, _file_path: String) {
    let mut _url: String = String::new();
    if _choice == 1 {
        _url = "basketbuild.com:21".to_string();
    } else {
        _url = "uploads.androidfilehost.com:21".to_string();
    }
    println!("\nType in your credentials for {}", _url);
    let _credentials: (String, String) = credential_setup::get_credentials();
    if _credentials.0 == "-1" || _credentials.1 == "-1"{
        // We shouldn't be here.
        panic!("[CRD-DBG]This isn't supposed to happen\n");
    }
    //
    // File handling
    //
    let _file_name = Path::new(& _file_path).file_name()
        .unwrap().to_str().unwrap();
    let mut file_stream = File::open(&_file_path).unwrap();
    // Compute File Size
    let mut _file_size = metadata(& _file_path);
    let _file_size:f64 =  (_file_size.unwrap().len()) as f64 / 1000000 as f64;
    //
    // FTP begin
    //
    let mut _ftp_stream = FtpStream::connect(_url.as_str())
        .unwrap_or_else(|err|panic!("Couldn't connect to remote server err log: {}", err));
    let _ = _ftp_stream
        .login(_credentials.0.as_str(), _credentials.1.as_str())
        .unwrap_or_else(|err|panic!("Can't login here's the response: {}", err));
    _ftp_stream.transfer_type(FileType::Binary)
        .expect("Can't set to binary upload mode");
    // Display File size
    println!("\nStarting to upload file: {} , with size: {} Megabytes", _file_name, _file_size);
    let _success = _ftp_stream.put(_file_name, &mut file_stream);
    let _success = match _success {
        Ok(_) => {
            println!("File has been uploaded successfully!");
        },
        Err(e) => {
            println!("There was an error uploading the file {}", e);
        },
    };
    let _ = _ftp_stream.quit();
}