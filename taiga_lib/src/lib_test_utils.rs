use lazy_static::lazy_static;

use crate::lib_routes::{AuthType, Authentificate, TaigaRoute};

/// This module contains some utilities for testing.

lazy_static!{
/// Lazy static macro that will fetch the TestData needed in the functions at runtime. 
/// This prevents that for every test, the .env file is not loaded over and over again, and more importantly 
/// that the Authentification function is not run every time, causing a potential timeout.
    pub static ref env_data:TestData = TestData::get(); 
}

/// Container to hold data of the .env file. 
pub struct TestData{
    pub(crate) base_url:String,
    pub(crate) auth_key:String,
    pub(crate) project_id:String,
}


impl TestData{
    /// Get the the data from the .env file. The env file should be in TOML format and contain the following data:
    /// `taiga_project_id` = project_id of a project to be used for testing.
    /// `taiga_username` = taiga username you want to use to run the test.
    /// `taiga_password` = the password of the account to be used in the test.
    /// `taiga_base_url` = base url where your taiga projects are on. For example: "https://api.taiga.io"
    fn get() -> TestData{

        let project_id =  std::env::var("taiga_project_id").expect("ProjectID not found");
        let base_url =  std::env::var("taiga_base_url").expect("ProjectID not found");


        let auth = AuthType::Taiga { 
            username: std::env::var("taiga_username").expect("Username not found"), 
            password: std::env::var("taiga_password").expect("Password not found")};

        let route = Authentificate{
        auth_type:&auth                        
        };
        // Make request for the Authkeys
        let response_json = route.request(&base_url,None).expect("Request failed");
        
        let auth_key = response_json["auth_token"].as_str().unwrap().to_string();
        
        TestData { base_url, auth_key, project_id}
        }
}
