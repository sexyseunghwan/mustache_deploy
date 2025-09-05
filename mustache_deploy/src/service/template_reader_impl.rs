use crate::common::*;

use crate::traits::service::template_reader::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct TemplateReaderImpl;

impl TemplateReaderImpl {}

impl TemplateReader for TemplateReaderImpl {
    #[doc = "배포할 mustache template 리스트를 반환해주는 함수"]
    /// 
    /// # Arguments
    /// * `&self` - TemplateReaderImpl 인스턴스
    /// 
    /// # Returns
    /// * `Ok(Vec<String>)` - 배포할 템플릿 이름들의 목록
    /// * `Err(anyhow::Error)` - 파일 읽기 실패 또는 환경변수 누락
    /// 
    /// # Environment Variables
    /// * `DEPLOY_TARGET_PATH` - 배포 대상 목록 파일의 상대 경로
    /// 
    /// # Command Line Arguments
    /// * `--path <경로>` - 기본 경로 지정 (필수)
    fn read_to_deploy_template(&self) -> anyhow::Result<Vec<String>> {
        let args: Vec<String> = std::env::args().collect();

        let base_path: PathBuf = if args.len() >= 5 && args[3] == "--path" {
            PathBuf::from(&args[4])
        } else {
            return Err(anyhow::anyhow!(
                "[ERROR][TemplateReaderImpl->read_to_deploy_template] --path argument is required."
            ));
        };

        let sub_path_str: String = env::var("DEPLOY_TARGET_PATH").unwrap_or_else(|e| {
            error!(
                "[ERROR][TemplateReaderImpl->read_to_deploy_template] {:?}",
                e
            );
            panic!(
                "[ERROR][TemplateReaderImpl->read_to_deploy_template] {:?}",
                e
            )
        });

        let sub_path: PathBuf = PathBuf::from(sub_path_str);
        let full_path: PathBuf = base_path.join(sub_path);

        let contents: String = std::fs::read_to_string(&full_path)?;
        let lines: Vec<String> = contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Ok(lines)
    }
}
