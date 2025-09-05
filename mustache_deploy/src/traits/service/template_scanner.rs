
use crate::model::mustache_template::*;

pub trait TemplateScanner {
    fn get_template_datas(&self, list: &Vec<String>) -> anyhow::Result<Vec<MustacheTemplate>>;
}
