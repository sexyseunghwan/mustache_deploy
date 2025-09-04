use crate::common::common::*;

use crate::traits::{
    repository::es_repository::*,
    service::{backup_service::*, template_deployer::*, template_reader::*, template_scanner::*},
};

#[derive(Debug, new)]
pub struct MainController<
    B: BackupService,
    TD: TemplateDeployer,
    TR: TemplateReader,
    TS: TemplateScanner,
> {
    backup_service: B,
    template_deployer: TD,
    template_reader: TR,
    template_scanner: TS,
}

impl<B: BackupService, TD: TemplateDeployer, TR: TemplateReader, TS: TemplateScanner>
    MainController<B, TD, TR, TS>
{
    #[doc = "mustache template 배포함수"]
    pub async fn deploy_task(&self) -> anyhow::Result<()> {
        
        
        
        Ok(())
    }
}
