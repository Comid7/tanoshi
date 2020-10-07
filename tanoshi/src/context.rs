use crate::catalogue::*;
use crate::extension::Extensions;
use sqlx::AnyPool;

pub struct GlobalContext {
    pub pool: AnyPool,
    pub extensions: Extensions,
}

impl GlobalContext {
    pub fn new(pool: AnyPool, extensions: Extensions) -> Self {
        Self { pool, extensions }
    }
}
