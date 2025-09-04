use crate::common::common::*;

pub trait TemplateReader {
    fn read_to_deploy_template(&self) -> anyhow::Result<Vec<String>>;
}
