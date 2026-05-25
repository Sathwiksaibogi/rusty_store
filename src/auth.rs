use std::io::{self,Write};

pub fn login()->bool{
    println!("rusty_store login");

    println!("enter your username : ");
    io::stdout().flush().unwrap();
    let mut username=String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("enter your password : ");
    io::stdout().flush().unwrap();
    let mut password=String::new();
    io::stdin().read_line(&mut password).unwrap();

    let final_username=username.trim();
    let final_password=password.trim();

    final_username=="admin" && final_password=="admin123"
}