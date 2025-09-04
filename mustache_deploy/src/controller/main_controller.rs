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
        /* 배포대상이 되는 mustache template 읽기 -> repository 에서 읽어줌 */
        let to_deploy_list: Vec<String> = self.template_reader.read_to_deploy_template()?;

        /* 해당 template scan */
        //self.template_scanner.test(&to_deploy_list);

        /* 해당 template 현재맞는 use case 에 대응하여 배포 */

        Ok(())
    }
}
