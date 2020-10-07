use crate::catalogue::*;
use crate::extension::Extensions;
use async_graphql::{Context, Object, Result, Schema, Subscription, ID};
use sqlx::AnyPool;

pub struct GlobalContext {
    pool: AnyPool,
    extensions: Extensions,
}

impl GlobalContext {
    pub fn new(pool: AnyPool, extensions: Extensions) -> Self {
        Self { pool, extensions }
    }
}
