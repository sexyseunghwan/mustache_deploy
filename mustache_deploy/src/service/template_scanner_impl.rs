use crate::common::*;

use crate::model::mustache_template::*;

use crate::traits::service::template_scanner::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct TemplateScannerImpl;

impl TemplateScanner for TemplateScannerImpl {
    
    #[doc = "반영할 mustache template 데이터 리스트를 반환해주는 함수"]
    fn get_template_datas(
        &self,
        template_name_list: &[String],
    ) -> anyhow::Result<Vec<MustacheTemplate>> {
        let args: Vec<String> = std::env::args().collect();

        let base_path: PathBuf = if args.len() >= 5 && args[3] == "--path" {
            PathBuf::from(&args[4])
        } else {
            return Err(anyhow::anyhow!(
                "[ERROR][TemplateReaderImpl->read_to_deploy_template] --path argument is required."
            ));
        };

        let sub_path_str: String = env::var("MUSTACHE_TEMPLATE_LIST_INFO_PATH").map_err(|e| {
            error!(
                "[ERROR][TemplateReaderImpl->read_to_deploy_template] {:?}",
                e
            );
            anyhow::anyhow!("Failed to get MUSTACHE_TEMPLATE_LIST_INFO_PATH: {:?}", e)
        })?;

        let sub_path: PathBuf = PathBuf::from(sub_path_str);
        let full_path: PathBuf = base_path.join(sub_path);

        let mut template_list: Vec<MustacheTemplate> = Vec::new();
        let re: Regex = Regex::new(r#""{3}(?s)(.*?)"{3}"#)?;

        for template_name in template_name_list {
            let template_file: String = format!("{}.es", template_name);
            let template_file_path: PathBuf = PathBuf::from(template_file);
            let full_template_file_path: PathBuf = full_path.join(template_file_path);

            let raw: String = match std::fs::read_to_string(full_template_file_path) {
                Ok(raw) => raw,
                Err(e) => {
                    error!(
                        "[ERROR][TemplateScannerImpl->read_file_return_form] {:?}",
                        e
                    );
                    continue;
                }
            };


            if let Some(caps) = re.captures(&raw) {
                let body: &str = caps
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("[ERROR][TemplateScannerImpl->get_template_datas] regex capture failed"))?
                    .as_str();

                let template: MustacheTemplate = MustacheTemplate::new(template_name.to_string(), body.to_string());
                template_list.push(template);
            }
        }

        Ok(template_list)
    }
}
