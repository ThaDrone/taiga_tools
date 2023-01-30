use std::io::{Write, self};

use rpassword::read_password;
use taiga_lib::lib_routes::{self, TaigaRoute, RouteError,MemberID};

use crate::{cli_storage::{Session, Config, LocalStorage}, BASE_URL};


/**
Initializing the program. It will try to read config and session data from files.
If they are not present, it will go trough a "wizard" where the login is asked and which project should be used.
*/
pub(crate) fn initialize() -> (Option<Session>, Option<Config>){

    // Try to load the config file with preferences, if not available, create one. 
    let config = match Config::load(){
        Ok(config) => config,
        Err(e) => {
            println!("{e}");
            config()
        },
    };

    // Try to load the session config file, if not found, login.
    let mut session = match  Session::load(){
        Ok(session) => check_session(session, &config),
        Err(e) => {
            println!("{e}");
            login()
        }
    };

    (Some(session),Some(config)) 
}

/// Makes a request using the session and the config. 
/// If the request fails, login again.
/// Return either the original [`Session`], or the one created by the fuction [`login()`]
fn check_session(session:Session,config:&Config) -> Session{

    // Make a dummy request.
    let response = lib_routes::GetUserInfo{id:&MemberID::Me};

    let status = response.request(&config.base_url,&Some(session.auth_key.to_string()));

    // Check if the  request was ok. If not, login.
    return if let Err(e)= status {
        
        // When there is a auth related issue, prompt the user to login.
        // If not, panic and print out the reason.
        match e {
            RouteError::NoAuthKey | RouteError::AuthentificationError => login(),
            e => panic!("Something unexpected went wrong making the test request {}", e)
        }
    } else {
        session
    }
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
    let data = route.request(BASE_URL, &None).expect("Could not login: ");

    let auth_key = data["auth_key"].to_string();

    let session = Session {auth_key};
    session.save();

    session

    }

fn config() -> Config{

    
    let config = Config::load();

    if let Ok(config) = config {
        return config;
    };

    todo!("No CFG found."); // Create a wizard for creating config files

    // TODO #5 
    // Get all projects for the user
    // display the projects
    // Ask the user which project to use
    // Save the config file 
    // Return the Config struct

}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_login(){
        let session = login();
        println!("Session: {}",session.auth_key)
    }
}