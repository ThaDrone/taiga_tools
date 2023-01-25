pub mod lib_routes;

use lib_routes::{AuthType, CreateIssue, TaigaRoute};

// TODO Make this an ENV or Config
const BASE_URL:&str = "https://api.taiga.io";

pub struct Project{
    id:String,
    structure:ProjectStructure,
}

impl Project {
    pub fn list_owned()-> Vec<String>{
        todo!();
    }
}
/// Get the possible statuses of the project. 
/// These should then be presented to the user, so they can use the exact statusus 
pub struct ProjectStructure{
    task_status:Vec<String>,
    
    // Get a vector with the possible states, possibly make this hashmaps? 
    issue_status:Vec<String>,
    issue_priorities:Vec<String>,
    issue_types:Vec<String>,


    // TODO complete this. 
}

impl ProjectStructure{

    // TODO Get all project structs
    pub fn new(id:String) -> ProjectStructure {
        todo!() // Make request so we can fill out the struct 

        // TODO This is a place where we could use ASYNC
        //https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html
    }

    fn get_info(){

    }

}
// Struct or enum?


pub struct Task{
    info:BasicInfo,
}

pub struct Issue {
    id: Option<String>,
    number: Option<String>,
    subject: String,
    project: String,
    description: Option<String>,
    assigned_to: Option<String>,
    blocked_note: Option<String>,
    is_blocked: Option<bool>,
    is_closed: Option<bool>,
    milestone: Option<String>,
    status: Option<String>,
    severity: Option<String>,
    priority: Option<String>,
    typeid: Option<String>,
    tags: Option<String>,
    watchers: Option<Vec<String>>,
}

impl TaigaActions for Issue{
    fn get(&mut self, id:String) -> Self {
        todo!()
    }

    fn create(&mut self, auth_key:&Option<String>) -> Result<String,String> {
        
        let route = CreateIssue{issue:&self};
        
        let response = route.request(&BASE_URL.to_string(), &auth_key)?; //TODO key is copied here :(

        let id = response["id"].as_str();
        let number = response["ref"].as_str();
        
        if let Some(id) = id{
           self.id = Some(id.to_owned());
        }
        
        if let Some(number) = number{
           self.number = Some(number.to_owned());
        }

        Ok("Succesfully created".to_string())

    }

    fn update(&mut self) {
        todo!()
    }

 
}

pub struct UserStory{
    info:BasicInfo
}
pub struct BasicInfo{

    id:String,
    project:String,
    subject:String,
    decscription:String,
    status:String,
 
}

trait TaigaActions {
    // TODO implement these in the Task / Usestories / Issues
    // TODO find better name for this trait
    fn get(&mut self, id:String) -> Self;
    
    fn create(&mut self, auth_key:&Option<String>)-> Result<String, String>;

    fn update(&mut self);

}

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