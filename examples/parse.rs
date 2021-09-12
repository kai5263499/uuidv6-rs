// 65a6e640-0542-61ec-b423-4608b4bb3c22
use env_logger::Env;
use log::debug;
extern crate uuidv6_rs;

use uuidv6_rs::Uuid;

fn main() {
    let env = Env::default()
    .filter_or("LOG_LEVEL", "debug")
    .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    debug!("id1={:?}", Uuid::from_string("1ec0348c-d8e6-6670-884e-d0c637c9be27".to_string()).unwrap().to_string());
    debug!("id2={:?}", Uuid::from_string("1ec04e1b-004a-663f-b4ff-9def2fd82815".to_string()).unwrap().to_string());
}