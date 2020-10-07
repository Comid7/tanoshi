use crate::context::GlobalContext;
use async_graphql::{Context, Object};
use serde::Deserialize;
use tanoshi_lib::extensions::Extension;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct SourceIndex {
    pub name: String,
    pub path: String,
    pub rustc_version: String,
    pub core_version: String,
    pub version: String,
}

#[derive(Clone)]
pub struct Source {
    pub name: String,
    pub url: Url,
    pub installed_version: String,
    pub available_version: String,
    pub installed: bool,
    pub update: bool,
}

#[Object]
impl Source {
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn url(&self) -> String {
        self.url.as_str().to_string()
    }

    async fn installed_version(&self) -> String {
        self.installed_version.clone()
    }

    async fn available_version(&self) -> String {
        self.available_version.clone()
    }

    async fn installed(&self) -> bool {
        self.installed
    }

    async fn update(&self) -> bool {
        self.update
    }
}

impl From<tanoshi_lib::model::Source> for Source {
    fn from(s: tanoshi_lib::model::Source) -> Self {
        Self {
            name: s.name,
            url: s.url,
            installed_version: s.version,
            available_version: "".to_string(),
            installed: false,
            update: false,
        }
    }
}

#[derive(Default)]
pub struct SourceRoot;

#[Object]
impl SourceRoot {
    async fn sources(&self, ctx: &Context<'_>) -> Vec<Source> {
        let exts = ctx
            .data_unchecked::<GlobalContext>()
            .extensions
            .extentions();

        let resp = ureq::get(
            format!(
                "https://raw.githubusercontent.com/faldez/tanoshi-extensions/repo-{}/index.json",
                std::env::consts::OS
            )
            .as_str(),
        )
        .call();
        let mut available_sources = resp.into_json_deserialize::<Vec<SourceIndex>>().unwrap();
        available_sources
            .iter_mut()
            .map(|s| {
                let mut installed = false;
                let mut update = false;
                let mut url = None;
                let mut version = "".to_string();
                if let Some(ext) = exts.get(&s.name) {
                    installed = true;
                    url = Some(ext.detail().url.clone());
                    version = ext.detail().version.clone();
                    let installed_version = ext
                        .detail()
                        .version
                        .split(".")
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    let available_version = s
                        .version
                        .split(".")
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    if installed_version[0] < available_version[0] {
                        update = true;
                    } else if installed_version[0] == available_version[0]
                        && installed_version[1] < available_version[1]
                    {
                        update = true;
                    } else if installed_version[0] == available_version[0]
                        && installed_version[1] == available_version[1]
                        && installed_version[2] < available_version[2]
                    {
                        update = true;
                    }
                    if s.core_version != tanoshi_lib::CORE_VERSION
                        || s.rustc_version != tanoshi_lib::RUSTC_VERSION
                    {
                        update = false;
                    }
                }
                Source {
                    name: s.name.clone(),
                    url: url.unwrap(),
                    installed_version: version,
                    available_version: s.version.clone(),
                    installed,
                    update,
                }
            })
            .collect()
    }
}
