pub mod rhone_yaml {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Project {
        pub api_version: String,
        pub name: String,
        pub description: String,
        pub version: String,
        pub language: String,
        pub interpreter_version: String,
    }

  
}
