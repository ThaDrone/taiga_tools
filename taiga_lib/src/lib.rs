pub mod lib_routes;
pub mod lib_models;
pub mod lib_test_utils;


// TODO Make this an ENV or Config
const BASE_URL:&str = "https://api.taiga.io";

#[cfg(test)]
mod tests{
    use crate::{lib_routes::*, BASE_URL};
    use crate::lib_models::{Issue, TODOActions};

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

        let response_json = route.request(BASE_URL,None).expect("Request failed");
        
        let auth_key:Option<&str>= Some(response_json["auth_token"].as_str().unwrap()); // TODO unwrap!

        println!("Authentificated");
            
        let mut issue = Issue{
            subject: Some(String::from("Test!")),
            project: Some(std::env::var("taiga_project_id").expect("No project found in .env")),
            description:Some(String::from("TEST")),
            ..Issue::default()
        };

        let response = issue.create(auth_key);

        println!("Created my issue! {}", response.unwrap_err())

    }
}