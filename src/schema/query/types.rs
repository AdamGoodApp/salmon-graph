#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
  pub id: String,
  pub name: String,
  pub home_planet: String,
}