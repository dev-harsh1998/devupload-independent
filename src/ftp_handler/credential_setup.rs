use rpassword::read_password;
use rpassword::read_password_from_tty;
use std::io;

pub fn get_credentials() -> (String, String) {
    println!("Enter your username below: ");
    let mut temp: String = String::new();
    io::stdin().read_line(&mut temp).expect("No Input!");
    temp = temp.trim().to_string();
    if cfg!(windows) {
        println!("Enter your password: ");
        let pass = read_password().unwrap();
        (temp, pass)
    } else if cfg!(unix) {
        let pass = read_password_from_tty(Some("Password: ")).unwrap();
        (temp, pass)
    } else {
        //control should never reach here.
        ("-1".to_string(), "-1".to_string())
    }
}
