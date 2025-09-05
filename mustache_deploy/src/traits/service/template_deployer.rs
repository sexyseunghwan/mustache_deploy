use crate::common::*;

use crate::model::mustache_template::*;

/// Mustache 템플릿 배포를 담당하는 trait
/// 
/// Elasticsearch에 템플릿을 실제로 배포하는 기능을 정의합니다.
#[async_trait]
pub trait TemplateDeployer {
    /// 주어진 템플릿 목록을 Elasticsearch에 배포하는 함수
    /// 
    /// # Arguments
    /// * `template_list` - 배포할 MustacheTemplate 객체들의 목록
    /// 
    /// # Returns
    /// * `Ok(())` - 모든 템플릿 배포 완료
    /// * `Err(anyhow::Error)` - 배포 과정에서 오류 발생
    async fn deploy_mustache_templates(&self, template_list: Vec<MustacheTemplate>) -> anyhow::Result<()>;
}
