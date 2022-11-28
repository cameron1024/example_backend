use once_cell::sync::Lazy;

use crate::model::user::mock::default_user;

use super::TestData;

pub static TEST_DATA: Lazy<TestData> = Lazy::new(|| TestData {
    users: vec![default_user()],
});
