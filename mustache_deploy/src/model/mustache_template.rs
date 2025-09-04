use crate::common::common::*;

use crate::model::script_content::*;

#[derive(Serialize, Deserialize, Debug, Getters, new)]
#[getset(get = "pub")]
pub struct MustacheTemplate {
    pub script_name: String,
    pub scipt: ScriptContent,
}
