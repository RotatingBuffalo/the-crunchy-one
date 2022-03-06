use std::iter::Map;

use serde::Deserialize;
use serenity::model::Permissions;
#[derive(Deserialize, Debug)]
pub struct BasicCommand {
    api_key: Option<String>,
    cost: Option<u32>,
    enabled: Option<bool>,
    permission_level: Permissions,
}
#[derive(Deserialize, Debug)]
struct Category {
    enabled: bool,
    m: Map<String, BasicCommand>,
}
#[derive(Deserialize, Debug)]
struct CommandsConfig {
    currency: Category,
    voting: Category,
    color: Category,
}
#[derive(Deserialize, Debug)]
struct FullConfig {
    prefix: String,
    cmdcfg: CommandsConfig,
}
impl BasicCommand {
    fn is_enabled(&self) -> Result<bool, &'static str> {
        match self.enabled {
            Some(x) => Ok(x),
            None => Err("This command can only be enabled or disabled by configuring the category it is in."),
        }
    }
    fn api_key(&self) -> Result<String, &'static str> {
        match self.api_key.clone() {
            Some(x) => Ok(x),
            None => Err("This command's config does not include an api key."),
        }
    }
    fn cost(&self) -> Result<u32, &'static str> {
        match self.cost {
            Some(x) => Ok(x),
            None => Err("This command's config does not include a cost."),
        }
    }
    fn permission_level(&self) -> Permissions {
        self.permission_level
    }
}
