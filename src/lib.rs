#![feature(int_log)]
use std::{
    iter,
    path::{Path, PathBuf},
};

use errors::LibraryResult;
use serde::Deserialize;
use toml::Value;

use crate::errors::LibraryError;

pub mod errors;
mod test;

#[derive(Debug, Deserialize)]
pub struct EchelonsConfiguration {
    pub padding: Option<u32>,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
struct EchelonsLoadedConfig<'a> {
    padding: Option<u32>,
    paths: Vec<(usize, &'a str)>,
}

#[derive(Debug)]
struct PathSpec {
    path: PathBuf,
    name: String,
}

impl EchelonsConfiguration {
    pub fn load(config_filename: &Path, target: &Path) -> LibraryResult<Self> {
        let config_data = std::fs::read_to_string(config_filename)?;
        let config_value: Value = toml::from_str(&config_data)?;

        let config = EchelonsLoadedConfig::from_toml_value(&config_value)?;

        let path_count = config.paths.len();
        let magnitude = config.padding.unwrap_or_else(|| match path_count {
            0 => 1,
            x => x.log10() + 1,
        });

        let top_paths = config
            .paths
            .into_iter()
            .map(|(idx, p)| create_path_spec(target, idx, magnitude, p));
        let paths: Vec<_> = top_paths
            .flat_map(|p| expand_path(&p, &config_value, magnitude))
            .collect();

        Ok(Self {
            paths,
            padding: config.padding,
        })
    }
}

impl<'a> EchelonsLoadedConfig<'a> {
    fn from_toml_value(config_data: &'a Value) -> Result<EchelonsLoadedConfig<'a>, LibraryError> {
        let padding = config_data
            .get("padding")
            .map(|mp| mp.as_integer().map(|p| p as u32))
            .unwrap_or(None);
        let paths = config_data.get("paths");
        if paths.is_none() {
            return Err(LibraryError::InvalidConfiguration(
                "No Top Level Paths Specified".to_owned(),
            ));
        }
        let paths = config_data["paths"].as_array();
        if paths.is_none() {
            return Err(LibraryError::InvalidConfiguration(
                "No Top Level Paths Specified".to_owned(),
            ));
        }
        let top_paths = paths_from_value(paths);
        let loaded = EchelonsLoadedConfig {
            padding,
            paths: top_paths,
        };
        Ok(loaded)
    }
}

fn expand_path(base_path: &PathSpec, config: &Value, magnitude: u32) -> Vec<PathBuf> {
    let sub_dirs = config[&base_path.name]["paths"].as_array();
    let sub_dirs = paths_from_value(sub_dirs);

    let sub_dirs: Vec<_> = sub_dirs
        .into_iter()
        .map(|(idx, p)| create_path_spec(&base_path.path, idx, magnitude, p))
        .map(|p| p.path)
        .collect();

    iter::once(base_path.path.to_owned())
        .chain(sub_dirs)
        .collect()
}

fn pad(num: usize, width: u32) -> String {
    let needed_zeros = if num == 0 {
        width - 1
    } else {
        let mag = num.log10() + 1;
        width - mag
    };
    let result: Vec<_> = iter::repeat_with(|| "0".to_owned())
        .take(needed_zeros as usize)
        .chain(iter::once(format!("{}", num)))
        .collect();
    result.join("")
}

fn create_path_spec(target: &Path, index: usize, magnitude: u32, path_name: &str) -> PathSpec {
    let full_dir_name = format!("{} {}", pad(index, magnitude), path_name);
    PathSpec {
        path: target.join(full_dir_name),
        name: path_name.to_owned(),
    }
}

fn paths_from_value(paths: Option<&Vec<Value>>) -> Vec<(usize, &str)> {
    let paths: Vec<_> = paths
        .unwrap()
        .iter()
        .map(value_to_string)
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|x| (x.0, x.1.unwrap()))
        .collect();
    paths
}

fn value_to_string(v: &Value) -> Option<&str> {
    v.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_from_config() {
        let config_str = r#"
padding = 3

paths = [ "test" ]
        "#;
        let config_value = toml::from_str(config_str).expect("Could not read toml data");

        let config =
            EchelonsLoadedConfig::from_toml_value(&config_value).expect("Loading did not succeed");
        assert_eq!(config.padding, Some(3));
        let spec = create_path_spec(Path::new("foo"), 1, config.padding.unwrap(), "test");
        assert_eq!(spec.path, Path::new("foo/001 test").to_path_buf());
    }

    #[test]
    fn test_padding_with_no_config() {
        let config_str = r#"
paths = [ "test" ]
                "#;
        let config_value = toml::from_str(config_str).expect("Could not read toml data");

        let config =
            EchelonsLoadedConfig::from_toml_value(&config_value).expect("Loading did not succeed");
        assert_eq!(config.padding, None);
    }
}
