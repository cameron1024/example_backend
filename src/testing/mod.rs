use std::sync::Arc;

use axum_test_helper::TestClient;

mod serde;
pub mod test_data;

use crate::{
    config::testing::test_config,
    make_app,
    model::user::User,
    state::{
        hasher::BcryptHasher, make_services, random::mock::MockRandom, time::mock::MockTime,
        Dependencies, Services,
    },
};

pub fn test_deps() -> Dependencies {
    Dependencies {
        time: Arc::new(MockTime::default()),
        random: Arc::new(MockRandom::new()),
        hasher: Arc::new(BcryptHasher), // uses lower cost modifier
        config: Arc::new(test_config()),
    }
}

pub fn test_services() -> Services {
    make_services(test_deps()).unwrap()
}

pub fn test_client(deps: Dependencies) -> (TestClient, Services) {
    let state = make_services(deps).unwrap();
    test_client_from_state(state)
}

fn test_client_from_state(state: Services) -> (TestClient, Services) {
    let client = TestClient::new(make_app(state.clone()));

    (client, state)
}

pub fn default_test_client() -> (TestClient, Services) {
    test_client(test_deps())
}

#[derive(Debug, Clone, Default)]
pub struct TestData {
    pub users: Vec<User>,
}

pub fn test_services_with(TestData { users }: TestData) -> Services {
    let services = test_services();

    let db = services.db.as_mock();
    db.users.lock().unwrap().extend(users);

    services
}

pub fn test_client_with(test_data: TestData) -> (TestClient, Services) {
    let services = test_services_with(test_data);
    test_client_from_state(services)
}
