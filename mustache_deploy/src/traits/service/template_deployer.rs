use crate::common::common::*;

use crate::model::mustache_template::*;

#[async_trait]
pub trait TemplateDeployer {
    async fn deploy_mustache_templates(&self, template_list: Vec<MustacheTemplate>) -> anyhow::Result<()>;
}
