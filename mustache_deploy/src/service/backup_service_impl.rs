use crate::common::common::*;

use crate::traits::service::backup_service::*;

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct BackupServiceImpl;

impl BackupService for BackupServiceImpl {}
