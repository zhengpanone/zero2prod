use std::{fs::File, io::Read};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub application: Application,
    pub test: TestConf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: Server::default(),
            application: Application::default(),
            test: TestConf::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: i32,
}

impl Default for Server {
    fn default() -> Self {
        Self { port: 8090 }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
}

impl Default for Application {
    fn default() -> Self {
        Self { name: String::from("Axum App") }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConf {
    pub debug: bool,
}

impl Default for TestConf {
    fn default() -> Self {
        Self { debug: false }
    }
}

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = {
        let mut file = File::open("config.yaml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: AppConfig = serde_yaml::from_str(&contents).unwrap();
        println!("配置文件加载成功-------------");
        config
    };
}
