/*
Author      : Seunghwan Shin
Create date : 2025-09-00
Description :

History     : 2025-09-00 Seunghwan Shin       # [v.1.0.0] first create.


실행방법 cargo run -- --env dev --path C:\Users\user\Desktop\private\mustache_template

cargo run -- --env dev --path C:\Users\user\Desktop\private

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

use crate::{
    controller::main_controller::{self, MainController},
    traits::service::{backup_service, template_deployer, template_reader, template_scanner},
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

    let backup_service: BackupServiceImpl = BackupServiceImpl::new();
    let template_deployer: TemplateDeployerImpl = TemplateDeployerImpl::new();
    let template_reader: TemplateReaderImpl = TemplateReaderImpl::new();
    let template_scanner: TemplateScannerImpl = TemplateScannerImpl::new();

    let main_controller: MainController<
        BackupServiceImpl,
        TemplateDeployerImpl,
        TemplateReaderImpl,
        TemplateScannerImpl,
    > = MainController::new(
        backup_service,
        template_deployer,
        template_reader,
        template_scanner,
    );

    match main_controller.deploy_task().await {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e)
        }
    };
}
