mod types;

use juniper::FieldResult;
use types::Human;

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            home_planet: "Mars".to_owned()
        })
    }
});