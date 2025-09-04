use crate::common::common::*;

use crate::traits::service::template_deployer::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct TemplateDeployerImpl;

impl TemplateDeployer for TemplateDeployerImpl {}
