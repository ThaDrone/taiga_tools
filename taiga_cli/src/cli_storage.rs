use std::{io};
use std::fs;
use serde::Deserialize;
use toml;
use serde::Serialize;

// Constants
const CFG_FILENAME:&str = "taigacli_config.toml";
const SES_FILENAME:&str = "taigacli_session.toml";

use toml::{to_string};

#[derive(Serialize, Deserialize)]
struct Session {
    auth_key: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    project_id: String,
}

impl LocalStorage for Session {
    /// Returns the file location of this [`Session`].
    fn file_location(&self) -> String {
        SES_FILENAME.to_string()
    }
}

impl LocalStorage for Config {
    /// Returns the file location of this [`Config`].
    fn file_location(&self) -> String {
        CFG_FILENAME.to_string()
    }
}

trait LocalStorage {
    fn save(&self) -> Result<(), io::Error> where Self: Serialize{
        let toml_str = to_string(self).expect("Error deserialsing the struct");
        let file_location = self.file_location();
        fs::write(file_location, toml_str)?;

        Ok(())
    }

    fn load(&mut self) -> Result<(), io::Error> {
        todo!()
    }

    fn file_location(&self) -> String;
}


#[cfg(test)]
mod tests{
    use crate::{Session, LocalStorage};

    #[test]
    fn test_localstorage(){

        let keys = Session{auth_key:"1234".to_string()};
        keys.save();

    }
}
