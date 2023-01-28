use std::{collections::HashMap};
use reqwest::{header::{HeaderMap, HeaderValue}, blocking::{Client}, StatusCode, Method };
use serde_json;
use log::debug;

use crate::lib_models::Issue;




/// This is supposed to be an abstraction for the api.
/// Try however to not use these functions directly. 
/// Instead use the items in [`lib_models`]. This is a kind of "ORM". 
/// Here will be for example a Struct called `Issue`, then you can either fill out the structs and create it on Taiga using 
/// 'let my_issue = Issue{.<your data>.};
/// my_issue.create();'
pub trait TaigaRoute {
    fn url(&self) -> String;

    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    fn form(&self) -> HashMap<&str, String> {
        HashMap::new()
    }

    fn needs_authkey() -> bool{
        true
    }

    fn method(&self) -> Method{
        Method::GET
    }
    
    /// Request method for structs implementing the `Taigaroute` trait
    /// Takes the base url (this should be fetched from a config file)
    /// TODO consider changing opt_auth_key parameter to Session
    fn request(&self,base_url:&str, opt_auth_key:Option<&str>) -> Result<serde_json::Value, String>{


        let mut url = base_url.to_owned();
        url.push_str(&self.url());

        // Get the headers, insert the default headers as well
        let mut headers = self.headers();
        headers.insert("Content-Type",HeaderValue::from_static("application/json"));

        let method = self.method();
        
        let form = self.form();

        // let mut builder = Client::new().request(method, &url);
        let mut builder = Client::new()
            .request(method, &url)
            .headers(headers)
            .form(&form);
            
        /*
        Check if an authkey was given. If not, it will check if the Authentificate route was given.
        If it was not, it will return an error.
        We cannot pass a empty string in the bearer_auth, then the Taiga server will panic. 
        */
        if let (Some(key), true) = (opt_auth_key, Self::needs_authkey()) { 
            // Ideally this checked at the start of the function,
            // but I dont know if I can make the request builder for only the bearer auth, and later merge it into the rest. 
            builder = builder.bearer_auth(key);
            
        }
        
        debug!("URL {} ", url);

        let response = builder.send();

        // Check if the status is correct
        let response = match response {
            Ok(resp) =>  resp,
            Err(e) => return Err(e.to_string()),
        };

        // Check if the status is correct
        match response.status(){
            StatusCode::OK => (),
            code => return Err(format!("Statuscode: {}, URL: {} ", code, url)),
        };

        // Check if the body is correct
        let body = match response.text() {
            Ok(body) => body,
            Err(e) => return Err(format!("Could not read the body of the response: {} ",e.to_string())),
        };

        match serde_json::from_str(&body){
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Could not convert the response to JSON: {}",e.to_string())),
        }


    } 
    
}
/// Enum that defines the options that can be used to authentificate to Taiga.
pub enum AuthType {
    Taiga{username:String,password:String}, 
    Github{username:String, password:String}, //TODO: #1 add github username + password login
    GithubToken(String), //TODO: #2 add github personal api token login
    // TODO #3 Add more auth methods
}

pub struct Authentificate<'a>{
    pub auth_type:&'a AuthType
}

impl<'a> TaigaRoute for Authentificate<'a>{
    fn url(&self) -> String {
        String::from("/api/v1/auth")
    }

    fn form(&self) -> HashMap<&str, String> {
        
        let mut form: HashMap<&str, String> = HashMap::new();

        match self.auth_type {
            AuthType::Taiga{username, password} => {
                form.insert("type","normal".to_string());
                form.insert("username",username.to_owned());
                form.insert("password",password.to_owned());
            } ,
            AuthType::Github {username, password } => todo!(),
            AuthType::GithubToken(_) => todo!(),        
        };

        form
    }

    fn needs_authkey() -> bool {
        false
    }

    fn method(&self) -> Method{
        Method::POST
    }
}

pub struct UserInfoMeRequest<'a>{
    pub(crate) id:&'a MemberID<'a>,
}

pub enum MemberID<'a>{
    Me,
    ID(&'a String)
}

impl<'a> TaigaRoute for UserInfoMeRequest<'a>{
    
    
    fn url(&self) -> String {   
        
        let url = String::from("api/v1/users/{id}");

        match self.id {
            MemberID::ID(id) => url.replace("{id}", &id),
            MemberID::Me => url.replace("{id}", &"me"),
        };

        url
    }


}

pub struct CreateIssue<'a>{
    pub(crate) issue:&'a Issue,
}

impl<'a> TaigaRoute for CreateIssue<'a> {
    fn url(&self) -> String {
        String::from("/api/v1/issues")
    }

    fn form(&self) -> HashMap<&str, String> {
        let mut form: HashMap<&str, String>  = HashMap::new();

        // TODO check if this can be more efficient with borrows. Might get really messy though
        form.insert("subject",self.issue.subject.to_owned());
        form.insert("project", self.issue.project.to_owned());

        self.issue.assigned_to.to_owned().and_then(|v| {form.insert("assigned_to",v)});
        self.issue.blocked_note.to_owned().and_then(|v| {form.insert("blocked_note",v)});
        self.issue.description.to_owned().and_then(|v| {form.insert("description",v)});
        self.issue.is_blocked.to_owned().and_then(|v| {form.insert("is_blocked",v.to_string())});
        self.issue.is_closed.to_owned().and_then(|v| {form.insert("is_closed",v.to_string())});
        self.issue.milestone.to_owned().and_then(|v| {form.insert("milestone",v)});
        self.issue.status.to_owned().and_then(|v| {form.insert("status",v)});
        self.issue.severity.to_owned().and_then(|v| {form.insert("severity",v)});
        self.issue.priority.to_owned().and_then(|v| {form.insert("priority",v)});
        self.issue.typeid.to_owned().and_then(|v| {form.insert("type",v)});
        self.issue.tags.to_owned().and_then(|v| {form.insert("tags",v)});

        form
    }

    fn method(&self) -> Method{
        Method::POST
    }
    // TODO implement Error in Request Error
  
}
enum RequestError{
    Reqwest(reqwest::Error),
    NoAuthKey,
}





#[cfg(test)]
mod tests{

    use crate::{lib_routes::*, BASE_URL};

    #[test]
    fn test_routes(){

        use dotenv::dotenv;
        dotenv().ok();
        
        let auth = AuthType::Taiga { 
            username: std::env::var("taiga_username").expect("Username not found"), 
            password: std::env::var("taiga_password").expect("Password not found")};

        let route = Authentificate{
           auth_type:&auth                        
        };

        let response_json = route.request(BASE_URL,None).expect("Request failed");
        
        let auth_key = response_json["auth_token"].as_str().unwrap().to_string();
        
        println!("Response to our request: {}", response_json);

        let route = UserInfoMeRequest{id:&MemberID::Me};

        let response = route.request(BASE_URL, Some(&auth_key));

    }

  

}