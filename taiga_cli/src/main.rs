use std::io;

use cli_storage::{LocalStorage, Session};
use taiga_lib::lib_routes::{self, TaigaRoute, AuthType};

mod cli_storage;
use rpassword::read_password;
use std::io::Write;

use log::debug;
    

const BASE_URL:&str = "https://api.taiga.io";

pub fn main(){

    initialize();

}

/**
Initializing the program. It will try to read config and session data from files.
If they are not present, it will go trough a "wizard" where the login is asked and which project should be used.
*/
fn initialize() -> Option<()>{

    let session = match  cli_storage::Session::load(){
        Ok(session) => {
            let mut session = session;
        }
        Err(e) => {
    
            println!("{e}");
            login();
        }
    };

    println!("### test ");

    Some(()) 
}

/// This function will prompt the user to login. 
fn login() -> Session{

    let mut username=  String::new();
    let mut password=  String::new();

    println!("Provide TAIGA username:");
    
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Provide TAIGA password:");

    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    println!("username:  {}",username);
    println!("password:  {}",password);

    let auth_type = lib_routes::AuthType::Taiga{
        username: username,
        password: password,
    }; 

    if let AuthType::Taiga { username, password } = &auth_type {
        println!("username:  {}",username);
        println!("password:  {}",password);
    }
    let route = lib_routes::Authentificate{auth_type:&auth_type};
    let data = route.request(&BASE_URL.to_owned(), &None).expect("Could not login: ");

    Session { auth_key: data["auth_key"].to_string().to_owned() }

    }

#[cfg(test)]
mod tests{
    use crate::cli_storage::{self, LocalStorage};

    use crate::*;
    #[test]
    fn test_login(){
        let session = login();
        println!("Session: {}",session.auth_key)
    }

    
}