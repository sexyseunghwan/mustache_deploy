/*
Author      : Seunghwan Shin
Create date : 2025-09-05
Description : Git Flow 를 통해서 Elasticsearch Mustache template 배포를 담당해주는 프로그램

History     : 2025-09-05 Seunghwan Shin       # [v.1.0.0] first create.

cargo run -- --env dev --path C:\Users\user\Desktop\private

*/
mod common;
use common::common::*;

mod utils_modules;
use utils_modules::logger_utils::*;

mod traits;

mod service;
use service::{template_deployer_impl::*, template_reader_impl::*, template_scanner_impl::*};

use crate::controller::main_controller::MainController;

mod model;

mod controller;

mod config;

mod repository;
use repository::es_repository_impl::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();
    info!("Run the mustache template distribution program.");

    let es_repository: EsRepositoryImpl = EsRepositoryImpl::new();
    let template_deployer: TemplateDeployerImpl<EsRepositoryImpl> =
        TemplateDeployerImpl::new(es_repository);
    let template_reader: TemplateReaderImpl = TemplateReaderImpl::new();
    let template_scanner: TemplateScannerImpl = TemplateScannerImpl::new();

    let main_controller: MainController<
        TemplateDeployerImpl<EsRepositoryImpl>,
        TemplateReaderImpl,
        TemplateScannerImpl,
    > = MainController::new(template_deployer, template_reader, template_scanner);

    match main_controller.deploy_task().await {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e)
        }
    };
}
