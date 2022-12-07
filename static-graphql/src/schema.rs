use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub struct Query;

#[async_graphql::Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub fn create_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}
