pub mod lib_routes;

use lib_routes::{AuthType, CreateIssue, TaigaRoute};

// TODO Make this an ENV or Config
const BASE_URL:&str = "https://api.taiga.io";

// Struct or enum?



// Functions
// pub fn authentificate(auth_type:&AuthType) -> Result<String, String>{
   
//     //TODO: #92 split these authentification modules into seperate functions based on the Authtype

//     let route = lib_routes::Authentificate{auth_type:&auth_type};

//     let response_result =  lib_routes::request(&BASE_URL.to_string(), &None, &route);
       
//     let binding = response_result.unwrap();
//     let authkey = binding["auth_token"].as_str().unwrap();

//     Ok(authkey.to_owned())

// }



#[cfg(test)]
mod tests{
    use crate::{lib_routes::*, Issue, BASE_URL, TaigaActions};

    // "Services"
    #[test]
    fn test_functions(){

        use dotenv::dotenv;
        dotenv().ok();
        
        let auth = AuthType::Taiga { 
            username: std::env::var("taiga_username").expect("Username not found"), 
            password: std::env::var("taiga_password").expect("Password not found")};

        let route = Authentificate{
           auth_type:&auth                        
        };

        let response_json = route.request(&BASE_URL.to_string(),&None).expect("Request failed");
        
        let auth_key:Option<String>= Some(response_json["auth_token"].as_str().unwrap().to_string());

        println!("Authentificated");
            
        let mut issue = Issue{
            id: None,
            number:None, 
            subject: String::from("Test!"),
            project: std::env::var("taiga_project_id").expect("No project found in .env"),
            description:Some(String::from("TEST")),
            assigned_to: None,
            blocked_note: None,
            is_blocked: None,
            is_closed: None,
            milestone: None,
            status: None,
            severity: None,
            priority: None,
            typeid: None,
            tags: None,
            watchers: None,
        };

        let response = issue.create(&auth_key);

        println!("Created my issue! {}", response.unwrap_err())

        
        
         
    }
}