use crate::common::*;

#[derive(Serialize, Deserialize, Debug, Getters, new)]
#[getset(get = "pub")]
pub struct MustacheTemplate {
    pub script_name: String,
    pub script: String,
}
