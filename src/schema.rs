mod mutation;
mod query;

use juniper::RootNode;
use mutation::MutationRoot;
use query::QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}