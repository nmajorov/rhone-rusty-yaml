pub mod rhone_yaml {
    use serde::{Deserialize, Serialize};

    /// yaml repesentation of rhone  project
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Project {
        #[serde(rename = "apiVersion")]
        pub api_version: String,
        pub name: String,
        pub description: String,
        pub version: String,
        pub language: String,
        pub interpreter_version: String,
        pub contributors: Vec<Contributor>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Contributor {
        pub name: String,
        pub email: String,
    }
}
