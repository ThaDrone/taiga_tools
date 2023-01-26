use crate::lib_routes::{CreateIssue, TaigaRoute};



/// Taiga "ORM". 
/// Use this library to interact with taiga.
/// The models use the [`lib_routes`].

pub struct Task{
    info:BasicInfo,
}

pub struct Issue {
    pub id: Option<String>,
    pub number: Option<String>,
    pub subject: String,
    pub project: String,
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub blocked_note: Option<String>,
    pub is_blocked: Option<bool>,
    pub is_closed: Option<bool>,
    pub milestone: Option<String>,
    pub status: Option<String>,
    pub severity: Option<String>,
    pub priority: Option<String>,
    pub typeid: Option<String>,
    pub tags: Option<String>,
    pub watchers: Option<Vec<String>>,
}

impl TaigaActions for Issue{

    fn get(&mut self, id:String) -> Self {
        todo!()
    }

    fn create(&mut self, auth_key:&Option<String>) -> Result<String,String> {
        
        let route = CreateIssue{issue:&self};
        
        let response = route.request(&crate::BASE_URL.to_string(), &auth_key)?; //TODO key is copied here :(

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

pub trait TaigaActions {
    // TODO implement these in the Task / Usestories / Issues
    // TODO find better name for this trait
    fn get(&mut self, id:String) -> Self;
    
    fn create(&mut self, auth_key:&Option<String>)-> Result<String, String>;

    fn update(&mut self);

}


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