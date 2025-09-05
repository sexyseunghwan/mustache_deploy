use crate::common::*;

//use crate::model::script_content::*;

/// Elasticsearch와의 상호작용을 추상화하는 Repository trait
/// 
/// Mustache 템플릿의 Elasticsearch 저장소 관련 작업을 정의합니다.
/// Send + Sync를 구현하여 멀티스레드 환경에서 안전하게 사용할 수 있습니다.
#[async_trait]
pub trait EsRepository: Send + Sync {
    // #[doc = "특정 elasticsearch cluster 에서 mustache 정보를 가져와주는 함수"]
    // async fn get_mustache_template_infos(&self) -> anyhow::Result<Value>;

    // #[doc = "특정 mustache template 을 가져와주는 함수"]
    // async fn get_mustache_script(&self, template_name: &str) -> anyhow::Result<ScriptContent>;

    #[doc = "mustache template을 elasticsearch 서버에 PUT 하는 함수"]
    /// 
    /// # Arguments
    /// * `template_name` - 저장할 템플릿의 이름 (Elasticsearch script ID로 사용)
    /// * `template_content` - 템플릿의 실제 mustache 스크립트 내용
    /// 
    /// # Returns
    /// * `Ok(true)` - 템플릿 저장 성공
    /// * `Err(anyhow::Error)` - 네트워크 오류, 권한 오류, 또는 잘못된 템플릿 형식
    async fn put_mustache_template(
        &self,
        template_name: &str,
        template_content: &str,
    ) -> anyhow::Result<bool>;
}
