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
        pub build_trigger: String,
        pub scripts: Scripts,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Contributor {
        pub name: String,
        pub email: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Scripts {
        pub preBuild: String,
        pub build: String,
        pub postBuild: String,
        pub preSCM: String,
    }
}
