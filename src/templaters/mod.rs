use crate::{commit::FileList, repository::Repository};
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

mod json;
mod nix;
mod yaml;

#[derive(Debug, Deserialize)]
#[serde(tag = "templater", rename_all = "snake_case")]
pub enum Mutation {
    Json {
        file: String,
        changes: HashMap<String, String>,
    },
    Yaml {
        file: String,
        changes: HashMap<String, String>,
    },
    Nix {
        file: String,
        changes: HashMap<String, String>,
    },
}

pub fn mutate(
    repository: &dyn Repository,
    branch: &str,
    mutations: &[Mutation],
) -> Result<FileList> {
    let mut changed: std::collections::HashMap<String, bytes::Bytes> = FileList::default();
    for mutation in mutations {
        let delta = match mutation {
            Mutation::Json { file, changes } => {
                let to_patch = repository.get(file, branch)?;
                log::debug!("patching JSON file file={file} branch={branch}");
                let patched = json::update_file(&to_patch, changes)?;
                FileList::from([(file.into(), patched)])
            }
            Mutation::Yaml { file, changes } => {
                let to_patch = repository.get(file, branch)?;
                log::debug!("patching YAML file file={file} branch={branch}");
                let patched = yaml::update_file(&to_patch, changes)?;
                FileList::from([(file.into(), patched)])
            }
            Mutation::Nix { file, changes } => {
                let to_patch = repository.get(file, branch)?;
                log::debug!("patching Nix file file={file} branch={branch}");
                let patched = nix::update_file(&to_patch, changes)?;
                FileList::from([(file.into(), patched)])
            }
        };
        changed.extend(delta);
    }
    Ok(changed)
}
