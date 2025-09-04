use crate::common::common::*;

use crate::traits::service::template_scanner::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct TemplateScannerImpl;

impl TemplateScannerImpl {
    
    fn read_file_and_return_script(&self, template_names: &Vec<String>) -> anyhow::Result<()> {

        let args: Vec<String> = std::env::args().collect();

        let base_path: PathBuf = if args.len() >= 5 && args[3] == "--path" {
            PathBuf::from(&args[4])
        } else {
            return Err(anyhow::anyhow!("[ERROR][TemplateReaderImpl->read_to_deploy_template] --path argument is required."));
        };

        let sub_path_str: String = env::var("MUSTACHE_TEMPLATE_LIST_INFO_PATH").unwrap_or_else(|e| {
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

        println!("full_path: {:?}", full_path);

        for template in template_names {
            
            let template_file: String = format!("{}.es", template);
            let template_file_path: PathBuf = PathBuf::from(template_file);
            let full_template_file_path: PathBuf = full_path.join(template_file_path);

            let raw: String = match std::fs::read_to_string(full_template_file_path) {
                Ok(raw) => raw,
                Err(e) => {
                    error!("[ERROR][TemplateScannerImpl->read_file_return_form] {:?}", e);
                    continue
                }
            };

            let raw = std::fs::read_to_string(&template)?;
            let mut lines = raw.lines();

            // 첫 줄은 그냥 버림
            lines.next();

            let body: String = lines.collect::<Vec<_>>().join("\n");

            println!("body: {}", body);

            

        }




        Ok(())
    }
    

}

impl TemplateScanner for TemplateScannerImpl {

    fn get_template_datas(&self, list: &Vec<String>) -> anyhow::Result<()> {

        

        //self.read_file_and_return_script(list)?;

        Ok(())
    }

}
