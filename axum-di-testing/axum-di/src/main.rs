use axum::{
    async_trait,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::time;

#[cfg(test)]
use mockall::automock;

#[tokio::main]
async fn main() {
    // Create the repository for our API. Note that the respository is a trait object.
    // It implements the trait DynHeroesRepository. This is done to allow mocking the repository
    // during unit testing of our handler functions.
    let repo = Arc::new(HeroesRepository()) as DynHeroesRepository;

    // Setup the top-level router. Here we only nest one sub-router (heroes). In practice, you would
    // nest more sub-routers here. Note that we create the router with state. The previously created
    // repository is our state.
    let app = Router::new()
        .nest("/heroes", heroes_routes())
        .with_state(repo);

    // Start the server. Note that for brevity, we do not add logging, graceful shutdown, etc.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Setup the sub-router for the heroes endpoint.
fn heroes_routes() -> Router<DynHeroesRepository> {
    Router::new().route("/", get(get_heroes))
}

/// The model for our API. We are maintaining heroes.
#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize, Eq, PartialEq, Debug, Copy, Clone, Default))]
pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
}

/// Represents an error that happened during data access.
enum DataAccessError {
    NotFound,
    #[allow(dead_code)]
    TechnicalError,
    #[allow(dead_code)]
    OtherError,
}

/// The repository trait for heroes
///
/// This trait is used to abstract the data access layer. At runtime, the trait is implemented by the
/// HeroesRepository struct. During unit testing, the trait is mocked by the MockHeroesRepositoryTrait.
/// This allows us to test the handler functions without having to access the database.
/// MockHeroesRepositoryTrait is generated by the automock macro (mockall crate).
#[cfg_attr(test, automock)]
#[async_trait]
trait HeroesRepositoryTrait {
    /// Gets a list of heroes from the DB filted by name
    async fn get_by_name(&self, name: &str) -> Result<Vec<Hero>, DataAccessError>;
}

/// Dummy implementation for our repository
/// 
/// In real life, this repository would access a database with persisted heroes.
/// However, for the sake of simplicity, we just simulate a database access by
/// using an in-memory collection of heroes.
struct HeroesRepository();

#[async_trait]
impl HeroesRepositoryTrait for HeroesRepository {
    async fn get_by_name(&self, name: &str) -> Result<Vec<Hero>, DataAccessError> {
        // We do not really use a database in this example. We just simulate one.
        // Our data is just a const array of heroes:
        const HEROES: [Hero; 2] = [
            Hero {
                id: "1",
                name: "Wonder Woman",
            },
            Hero {
                id: "2",
                name: "Deadpool",
            },
        ];

        // Simulate a real database access by doing an async wait operation.
        time::sleep(Duration::from_millis(100)).await;

        // As we do not have a real database, we just filter the heroes array in memory.
        // Note that we simulate the DB behaviour of `LIKE` and the % wildcard at the
        // end of the name filter by using the `starts_with` method.
        let found_heroes = HEROES
            .into_iter()
            .filter(|hero| {
                if let Some(stripped_name) = name.strip_suffix('%') {
                    // We have a % at the end of the name filter. This means that we have to use
                    // the `starts_with` method to simulate the `LIKE` behaviour of the DB.
                    hero.name.starts_with(stripped_name)
                } else {
                    // No % at the end, so we can use the `==` operator.
                    hero.name == name
                }
            })
            .collect::<Vec<Hero>>();

        if found_heroes.is_empty() {
            // We did not find any heroes. This means that we have to return a "not found" error.
            Err(DataAccessError::NotFound)
        } else {
            // We found some heroes. Return them.
            Ok(found_heroes)
        }
    }
}

/// The repository trait for the state of our router.
type DynHeroesRepository = Arc<dyn HeroesRepositoryTrait + Send + Sync>;

/// The filter struct for the get_heroes handler function.
#[derive(Deserialize)]
pub struct GetHeroFilter {
    name: Option<String>,
    // In practice, add additional query parameters here
}

/// The handler function for the get_heroes endpoint.
#[debug_handler]
async fn get_heroes(
    State(repo): State<DynHeroesRepository>,
    filter: Query<GetHeroFilter>,
) -> impl IntoResponse {
    // Use "%" as the default filter if no filter was provided.
    let mut name_filter = filter.name.to_owned().unwrap_or("%".to_string());

    // Append % wildcard to the filter if it does not already end with %.
    if !name_filter.ends_with('%') {
        name_filter.push('%');
    }

    // Call the repository to get the heroes from the DB.
    let result = repo.get_by_name(name_filter.as_str()).await;

    // Here comes the core logic of our API handler. It translates the result of the DB access
    // into the correct HTTP status codes. In practice, you would do more error handling and
    // probably some logging here. However, in our example we want to keep it simple and focus
    // on the basic principles of mocking and testing.
    match result {
        // We did not find any heroes. Return a 404.
        Err(DataAccessError::NotFound) => StatusCode::NOT_FOUND.into_response(),

        // We found some heroes. Return them as JSON.
        Ok(heroes) => Json(heroes).into_response(),

        // We got any other error from the DB. Return a 500.
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use mockall::predicate::*;
    use rstest::rstest;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    /// Helper function to create a GET request for a given URI.
    fn send_get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method("GET")
            .body(Body::empty())
            .unwrap()
    }

    /// Test function for the successful execution of the get_heroes handler function.
    ///
    /// This test function uses the rstest crate to test the handler function with different
    /// query parameters. The test function is executed for each test case.
    #[rstest]
    #[case("/?name=Wonder", "Wonder%")] // Verify that % is appended to the filter
    #[case("/?name=Wonder%", "Wonder%")] // Verify that % is not appended to the filter if it already ends with %
    #[case("/", "%")] // Verify that % is used as the default filter
    #[tokio::test]
    async fn get_by_name_success(#[case] uri: &'static str, #[case] expected_filter: &'static str) {
        // Create a vector of dummy heroes to return from the mock repository.
        let dummy_heroes = vec![Default::default()];

        // Create a mock repository and set the expectations for the get_by_name method.
        // Note that we are filtering the expectation by the expected filter string.
        let mut repo_mock = MockHeroesRepositoryTrait::new();
        let result = Ok(dummy_heroes.clone());
        repo_mock
            .expect_get_by_name()
            .with(eq(expected_filter))
            .return_once(move |_| result);

        // Create mock repository
        let repo = Arc::new(repo_mock) as DynHeroesRepository;

        // Create the app with the mock repository as state.
        let app = heroes_routes().with_state(repo);

        // Call the app with a GET request to the get_heroes endpoint.
        let response = app.oneshot(send_get_request(uri)).await.unwrap();

        // Check the response status code.
        assert_eq!(response.status(), StatusCode::OK);

        // Check the response body.
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(&dummy_heroes));
    }

    /// Test DB error handling in get_heroes handler function.
    #[rstest]
    #[case(DataAccessError::NotFound, StatusCode::NOT_FOUND)]
    #[case(DataAccessError::TechnicalError, StatusCode::INTERNAL_SERVER_ERROR)]
    #[tokio::test]
    async fn get_by_name_failure(
        #[case] db_result: DataAccessError,
        #[case] expected_status: StatusCode,
    ) {
        // Create a mock repository and set the expectations for the get_by_name method.
        // Note that this time, we are returning an error.
        let mut repo_mock = MockHeroesRepositoryTrait::new();
        repo_mock
            .expect_get_by_name()
            .with(eq("Spider%"))
            .return_once(|_| Err(db_result));

        // Create mock repository
        let repo = Arc::new(repo_mock) as DynHeroesRepository;

        // Create the app with the mock repository as state.
        let app = heroes_routes().with_state(repo);

        // Call the app with a GET request to the get_heroes endpoint.
        let response = app
            .oneshot(send_get_request("/?name=Spider"))
            .await
            .unwrap();

        // Check the response status code.
        assert_eq!(response.status(), expected_status);
    }
}
