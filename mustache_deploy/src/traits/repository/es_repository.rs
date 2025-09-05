use crate::common::common::*;

//use crate::model::script_content::*;

#[async_trait]
pub trait EsRepository: Send + Sync {
    // #[doc = "특정 elasticsearch cluster 에서 mustache 정보를 가져와주는 함수"]
    // async fn get_mustache_template_infos(&self) -> anyhow::Result<Value>;

    // #[doc = "특정 mustache template 을 가져와주는 함수"]
    // async fn get_mustache_script(&self, template_name: &str) -> anyhow::Result<ScriptContent>;

    #[doc = "mustache template을 elasticsearch 서버에 PUT 하는 함수"]
    async fn put_mustache_template(
        &self,
        template_name: &str,
        template_content: &str,
    ) -> anyhow::Result<bool>;
}
