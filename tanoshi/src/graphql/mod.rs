use crate::catalogue::{CatalogueRoot, SourceRoot, LibraryMutationRoot};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, MergedObject, Object, Result, Schema, Subscription,
    ID,
};

pub type TanoshiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(SourceRoot, CatalogueRoot);

#[derive(MergedObject, Default)]
pub struct MutationRoot(LibraryMutationRoot);
