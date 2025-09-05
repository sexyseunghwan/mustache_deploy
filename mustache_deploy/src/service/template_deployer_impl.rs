use crate::common::*;

use crate::traits::repository::es_repository::*;
use crate::traits::service::template_deployer::*;

use crate::model::mustache_template::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct TemplateDeployerImpl<R: EsRepository> {
    es_repository: R,
}


#[async_trait]
impl<R: EsRepository> TemplateDeployer for TemplateDeployerImpl<R> {

    #[doc = "mustache template 을 직접 배포해주는 함수"]
    async fn deploy_mustache_templates(&self, template_list: Vec<MustacheTemplate>) -> anyhow::Result<()> {

        for template in template_list {            
                 
            match self.es_repository.put_mustache_template(template.script_name(), template.script()).await {
                Ok(_) => {
                    info!("{} Deployment success.", template.script_name());
                },
                Err(e) => {
                    error!("[ERROR][TemplateDeployerImpl->deploy_mustache_templates] {:?}", e);
                    continue;
                }
            }
        }
        
        Ok(())
    }

}
