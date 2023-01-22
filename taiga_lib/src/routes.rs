use std::{collections::HashMap};
use reqwest::{header::{HeaderMap, HeaderValue}, blocking::{Client, RequestBuilder}, StatusCode, Method};
use serde_json;
use log::debug;

use crate::Issue;


pub trait TaigaRoute {
    fn url(&self) -> String;

    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    fn form(&self) -> HashMap<&str, String> {
        HashMap::new()
    }

    fn needs_authkey(&self) -> bool{
        true
    }

    fn method(&self) -> Method{
        Method::GET
    }
}
pub enum AuthType {
    Taiga{username:String,password:String}, 
    Github{username:String, password:String}, //TODO: #94 add github username + password login
    GithubToken(String), //TODO: #97 add github personal api token login
    //TODO: #98 add missing authtypes
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

    fn needs_authkey(&self) -> bool {
        false
    }

    fn method(&self) -> Method{
        Method::POST
    }
}

struct UserInfoMeRequest<'a>{
    id:&'a MemberID<'a>,
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
    issue:&'a Issue,
}

impl<'a> TaigaRoute for CreateIssue<'a> {
    fn url(&self) -> String {
        String::from("/api/v1/issues")
    }

    fn form(&self) -> HashMap<&str, String> {
        let mut form: HashMap<&str, String>  = HashMap::new();

        form.insert(&"subject",self.issue.subject);
        form.insert("project", self.issue.project);

        self.issue.assigned_to.and_then(|v| {form.insert("assigned_to",v)});
        self.issue.blocked_note.and_then(|v| {form.insert("blocked_note",v)});
        self.issue.description.and_then(|v| {form.insert("description",v)});
        self.issue.is_blocked.and_then(|v| {form.insert("is_blocked",v.to_string())});
        self.issue.is_closed.and_then(|v| {form.insert("is_closed",v.to_string())});
        self.issue.milestone.and_then(|v| {form.insert("milestone",v)});
        self.issue.status.and_then(|v| {form.insert("status",v)});
        self.issue.severity.and_then(|v| {form.insert("severity",v)});
        self.issue.priority.and_then(|v| {form.insert("priority",v)});
        self.issue.typeid.and_then(|v| {form.insert("type",v)});
        self.issue.tags.and_then(|v| {form.insert("tags",v)});

        form
    }

    fn method(&self) -> Method{
        Method::POST
    }
    
}
enum RequestError{
    Reqwest(reqwest::Error),
    NoAuthKey,
}

// TODO implement Error in Request Error

pub fn request(base_url:&String, opt_authkey:Option<&String>,route:&dyn TaigaRoute) -> Result<serde_json::Value, String>{

    // Check if an authkey was given. If not, it will check if the Authentificate route was given.
    // If it was not, it will return an error.
    let mut auth_key:&String = &String::new();
    
    if route.needs_authkey() {
        auth_key = match opt_authkey {
            Some(key) =>  &key,
            None => return Err("No authkey was found".to_string()),
        }
    }
   
    let method = route.method();

    // Get URL
    let mut url = base_url.to_owned();
    let end_point = route.url();
    
    url.push_str(&end_point);

    // Getting the Headers
    let mut headers = route.headers();
    headers.insert("Content-Type",HeaderValue::from_static("application/json"));

    let form = route.form();

    // Debug
    debug!("Auth key {}", auth_key);
    debug!("URL {} ", url);
    // debug!("Headers {}", headers);
    // debug!("Form {}", &form);

    let client = Client::new();

    let mut builder = client
        .request(method, &url)
        .headers(headers)
        .form(&form);
    
    if let Some(key) = opt_authkey {
        builder = builder.bearer_auth(key);
    }

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



#[cfg(test)]
mod tests{

    use crate::{routes::*, BASE_URL};

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

        let response_json = request(&BASE_URL.to_string(),None, &route).expect("Request failed");
        
        let auth_key = response_json["auth_token"].as_str().unwrap().to_string();
        
        println!("Response to our request: {}", response_json);

        let route = UserInfoMeRequest{id:&MemberID::Me};

        let response = request(&BASE_URL.to_string(), Some(&auth_key), &route);

    }

  

}