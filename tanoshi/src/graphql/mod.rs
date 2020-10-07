use crate::catalogue::{MangaRoot, SourceRoot};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, MergedObject, Object, Result, Schema, Subscription,
    ID,
};

pub type TanoshiSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SourceRoot, MangaRoot);
