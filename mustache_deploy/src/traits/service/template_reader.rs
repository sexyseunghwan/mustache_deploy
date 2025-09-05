
/// 배포 대상 템플릿 목록을 읽어오는 trait
/// 
/// 파일이나 설정에서 배포할 템플릿 이름들을 읽어오는 기능을 정의합니다.
pub trait TemplateReader {
    /// 배포할 템플릿 이름 목록을 읽어오는 함수
    /// 
    /// # Returns
    /// * `Ok(Vec<String>)` - 배포할 템플릿 이름들의 목록
    /// * `Err(anyhow::Error)` - 파일 읽기 실패 또는 설정 오류
    fn read_to_deploy_template(&self) -> anyhow::Result<Vec<String>>;
}
