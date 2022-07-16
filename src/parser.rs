use crate::types::Target;
use anyhow::Result;
use serde::Serialize;

pub fn parse(content: &String, input_target: Option<Target>, target: Target) -> Result<String> {
    if let Some(it) = input_target {
        match it {
            Target::Json => resolve(serde_json::from_str::<serde_json::Value>(content)?, target),
            Target::Toml => resolve(toml::from_str::<toml::Value>(content)?, target),
            Target::Yaml => resolve(serde_yaml::from_str::<serde_yaml::Value>(content)?, target),
        }
    } else {
        match parse(content, Some(Target::Json), target.clone()) {
            Ok(result) => Ok(result),
            Err(_) => match parse(content, Some(Target::Yaml), target.clone()) {
                Ok(result) => Ok(result),
                Err(_) => parse(content, Some(Target::Toml), target),
            },
        }
    }
}

fn resolve<T>(data: T, target: Target) -> Result<String>
where
    T: Serialize,
{
    match target {
        Target::Json => Ok(serde_json::to_string_pretty(&data).expect("failed to stringify Json")),
        Target::Toml => Ok(toml::to_string(&data).expect("failed to stringify toml")),
        Target::Yaml => Ok(serde_yaml::to_string(&data).expect("failed to stringify Yaml")),
    }
}
