use async_graphql::{Context, Object, Result, Schema, Subscription, ID};

pub struct Source {
    pub name: String,
    pub url: String,
    pub path: String,
    pub rustc_version: String,
    pub core_version: String,
    pub installed_version: String,
    pub version: String,
    pub installed: bool,
    pub update: bool,
}

#[Object]
impl Source {
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn url(&self) -> String {
        self.url.clone()
    }

    async fn path(&self) -> String {
        self.path.clone()
    }

    async fn rustc_version(&self) -> String {
        self.rustc_version.clone()
    }

    async fn core_version(&self) -> String {
        self.core_version.clone()
    }

    async fn installed_version(&self) -> String {
        self.installed_version.clone()
    }

    async fn version(&self) -> String {
        self.version.clone()
    }

    async fn installed(&self) -> bool {
        self.installed
    }

    async fn update(&self) -> bool {
        self.update
    }
}
