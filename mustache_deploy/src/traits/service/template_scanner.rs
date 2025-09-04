use crate::common::common::*;

pub trait TemplateScanner {
    fn get_template_datas(&self, list: &Vec<String>) -> anyhow::Result<()>;
}
