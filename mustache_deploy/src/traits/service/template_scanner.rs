
use crate::model::mustache_template::*;

/// 템플릿 파일을 스캔하여 내용을 추출하는 trait
/// 
/// 템플릿 이름 목록을 받아서 해당하는 파일들을 읽고 실제 스크립트 내용을 추출하는 기능을 정의합니다.
pub trait TemplateScanner {
    /// 주어진 템플릿 이름 목록에서 실제 템플릿 데이터를 추출하는 함수
    /// 
    /// # Arguments
    /// * `list` - 스캔할 템플릿 이름들의 목록
    /// 
    /// # Returns
    /// * `Ok(Vec<MustacheTemplate>)` - 스캔된 템플릿 데이터 목록
    /// * `Err(anyhow::Error)` - 파일 읽기 실패 또는 포맷 오류
    fn get_template_datas(&self, list: &[String]) -> anyhow::Result<Vec<MustacheTemplate>>;
}
