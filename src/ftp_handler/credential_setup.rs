use std::io;

pub fn get_credentials() -> (String, String) {
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