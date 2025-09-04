use crate::common::common::*;

#[derive(Debug, Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct ScriptContent {
    pub lang: String,
    pub source: String,
}
