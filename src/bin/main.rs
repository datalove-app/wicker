// #[macro_use]
// extern crate anyhow;
extern crate failure;
//#[macro_use]
//extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use env_logger;
use lucet_runtime::DlModule;
use std::{
    env,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};
use structopt::StructOpt;
use wicker::{Config, Host, Mount};

fn main() {}

//fn run(binary_path: &str) {
//    env_logger::init();
//
//    let main_dir_path = String::from(".");
//    let main_dir = File::open(&main_dir_path)
//        .unwrap_or_else(|_| panic!("Failed to preopen dir: {}", &main_dir_path));
//    let preopened_dirs = [(main_dir_path, main_dir)];
//    let argv = [
//        APP_NAME.clone(),
//        String::from("wasiplay/test.txt"),
//        String::from("wasiplay/test_output.txt"),
//    ];
//
//    let module = DlModule::load(&APP_LOCATION.as_str()).unwrap();
//    let mut host = Host::new(
//        module.into(),
//        Config {
//            preopened_dirs: &preopened_dirs,
//            argv: &argv,
//            ..Config::default()
//        },
//    );
//
//    host.init();
//}
