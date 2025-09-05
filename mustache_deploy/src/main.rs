/*
Author      : Seunghwan Shin
Create date : 2025-09-05
Description : Git Flow 를 통해서 Elasticsearch Mustache template 배포를 담당해주는 프로그램

History     : 2025-09-05 Seunghwan Shin       # [v.1.0.0] first create.

cargo run -- --env dev --path C:\Users\user\Desktop\private

*/
mod common;
use common::*;

mod utils_modules;
use utils_modules::logger_utils::*;

mod traits;

mod service;
use service::{template_deployer_impl::*, template_reader_impl::*, template_scanner_impl::*};

mod model;

mod controller;
use controller::main_controller::*;

mod repository;
use repository::es_repository_impl::*;

/// Elasticsearch Mustache Template 배포 시스템의 메인 함수
/// 
/// # 기능
/// - 환경 설정 로드 및 로거 초기화
/// - ES Repository와 템플릿 관련 서비스들 생성
/// - MainController를 통한 템플릿 배포 작업 실행
/// 
/// # 명령행 인수
/// - `--env dev|prod`: 실행 환경 설정
/// - `--path <경로>`: 템플릿 파일들이 위치한 기본 경로
#[tokio::main]
async fn main() {
    dotenv().ok();
    set_global_logger();
    info!("Run the mustache template distribution program.");

    let es_repository: EsRepositoryImpl = match EsRepositoryImpl::new() {
        Ok(repo) => repo,
        Err(e) => {
            error!("Failed to initialize ES repository: {:?}", e);
            return;
        }
    };
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
