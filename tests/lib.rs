#[cfg(test)]
mod tests {

use serde_yaml;
use rust_yaml::rhone_yaml::{Project};

    #[test]
    fn it_works() -> Result<(), serde_yaml::Error> {

        let project = Project {
            api_version: String::from("v1"),
            name: String::from("rust-yaml"),
            description: String::from("simple rest project"),
            language: String::from("rust"),
            version: String::from("1.0.1"),
            interpreter_version: String::from("1.39.0"),
        };

        let s = serde_yaml::to_string(&project)?;
        println!("yaml {}", s);
        Ok(())
    }
}
