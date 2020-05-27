#[cfg(test)]
mod tests {

    use rust_yaml::rhone_yaml::{Contributor, Project, Scripts};
    use serde_yaml;

    #[test]
    fn it_works() -> Result<(), serde_yaml::Error> {
        let contr1 = Contributor {
            name: "Nikolaj Majorov".to_string(),
            email: "nikolaj@majorov.biz".to_string(),
        };

        let script = Scripts {
            preBuild: String::from("run-prebuild.zsh"),
            build: String::from("build"),
            postBuild: String::from("run-build.sh"),
            preSCM: String::from("run-preSCM.zsh"),
        };
        let project = Project {
            api_version: String::from("build.rhone.io/v1"),
            name: String::from("rust-yaml"),
            description: String::from("simple rest project"),
            language: String::from("rust"),
            version: String::from("1.0.1"),
            interpreter_version: String::from("1.39.0"),
            contributors: vec![contr1],
            scripts: script,
            build_trigger: String::from("every 3 hours"),
        };

        let s = serde_yaml::to_string(&project)?;
        println!("yaml: \n {}", s);
        Ok(())
    }
}
