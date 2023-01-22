mod routes;

use routes::AuthType;

// TODO Make this an ENV or Config
const BASE_URL:&str = "https://api.taiga.io";

struct Project{
    id:String,
    structure:ProjectStructure,
}

impl Project {
    pub fn list_owned()-> Vec<String>{
        todo!();
    }
}
struct ProjectStructure{
    // Get the possible statuses of the project. 
    // These should then be presented to the user, so they can use the exact statusus 
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


struct Task{
    info:BasicInfo,
}

struct Issue {
    id: Option<String>,
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
    watchers: Vec<String>,
}

impl TaigaActions for Issue{
    fn get(&mut self, id:String) -> Self {
        todo!()
    }

    fn create(&mut self){
    
        let route = CreateIssue;





    }

    fn update(&mut self, subject:String, description:String) {
        todo!()
    }
}

struct UserStory{
    info:BasicInfo
}
struct BasicInfo{

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
    
    fn create(&mut self);

    fn update(&mut self);

}

// Functions
pub fn authentificate(auth_type:&AuthType) -> Result<String, String>{
   
    //TODO: #92 split these authentification modules into seperate functions based on the Authtype

    let route = routes::Authentificate{auth_type:&auth_type};

    let response_result =  routes::request(&BASE_URL.to_string(), None, &route);
       
    let binding = response_result.unwrap();
    let authkey = binding["auth_token"].as_str().unwrap();

    Ok(authkey.to_owned())

}



#[cfg(test)]
mod tests{
    use crate::routes::*;

    // "Services"
    #[test]
    fn test_functions(){

        // Loads the .env file
        use dotenv::dotenv;
        dotenv().ok();
        
        let auth = AuthType::Taiga { 
            username: std::env::var("taiga_username").expect("Username not found"), 
            password: std::env::var("taiga_password").expect("Password not found")};

        let authkey = crate::authentificate(&auth).unwrap();

        println!("Authentificated");
        
        
         
    }
}