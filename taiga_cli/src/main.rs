use std::io;

use cli_storage::{LocalStorage, Session, Config};
use taiga_lib::lib_routes::{self, TaigaRoute, AuthType, Authentificate};

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
fn initialize() -> Option<(Session, Config)>{

    let mut session = match  cli_storage::Session::load(){
        Ok(session) => session,
        Err(e) => {
            println!("{e}");
            login()
        }
    };

    let config = match Config::load(){
        Ok(config) => config,
        Err(e) => {
            println!("{e}");
            config()
        },
    }

    Some((session,config )) 
}

/// This function will prompt the user to login. 
fn login() -> Session{

    // TODO #6 Check for environment variables in the login function


    let mut username=  String::new();

    println!("Provide TAIGA username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Provide TAIGA password:");

    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    let username =  username.trim().to_string();
    let password = password.trim().to_string();

    // TODO Based on some options we should have different types here (Github etc)
    // Parsing the recieved username and password into a Authtype::Taiga route
    let auth_type = lib_routes::AuthType::Taiga{
        username,
        password,
    }; 

    // Creating a route
    let route = lib_routes::Authentificate{auth_type:&auth_type};

    // Running the route
    // TODO handle this properly
    let data = route.request(&BASE_URL.to_owned(), &None).expect("Could not login: ");

    let auth_key = data["auth_key"].to_string();
    let id = data["id"].to_string();

    Session {auth_key,id}

    }

fn config() -> Config{

    
    let config = Config::load();

    if let Ok(config) = config {
        return config;
    };

    todo!(); // Create a wizard for creating config files
    // TODO #5 
    // Get all projects for the user
    // display the projects
    // Ask the user which project to use
    // Save the config file 
    // Return the Config struct

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