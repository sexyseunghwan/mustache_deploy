/*
Author      : Seunghwan Shin
Create date : 2025-09-00
Description :

History     : 2025-09-00 Seunghwan Shin       # [v.1.0.0] first create.

*/
mod common;
use common::common::*;

mod utils_modules;
use utils_modules::logger_utils::*;

mod traits;

mod service;
use service::{
    backup_service_impl::*, template_deployer_impl::*, template_reader_impl::*,
    template_scanner_impl::*,
};

mod model;

mod controller;

mod config;

mod repository;

#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();
    info!("Start the mustache template deployment Program");    
    

    println!("Hello, world!");
}
