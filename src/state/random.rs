use std::fmt::Debug;

use microtype::Microtype;
use uuid::Uuid;

use crate::model::types::UserId;

pub trait Random: Debug + Send + Sync + 'static {
    fn uuid(&self) -> Uuid;

    fn user_id(&self) -> UserId {
        UserId::new(self.uuid())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SystemRandom;

impl Random for SystemRandom {
    fn uuid(&self) -> Uuid {
        Uuid::new_v4()
    }
}

#[cfg(test)]
pub mod mock {
    use std::sync::{Arc, Mutex};

    use rand_chacha::{
        rand_core::{RngCore, SeedableRng},
        ChaChaRng,
    };
    use uuid::Uuid;

    use super::Random;

    #[derive(Debug, Clone)]
    pub struct MockRandom(pub Arc<Mutex<ChaChaRng>>);

    impl Default for MockRandom {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockRandom {
        pub fn new() -> Self {
            Self::new_with_seed([0; 32])
        }

        pub fn new_with_seed(seed: [u8; 32]) -> Self {
            Self(Arc::new(Mutex::new(ChaChaRng::from_seed(seed))))
        }
    }

    impl Random for MockRandom {
        fn uuid(&self) -> uuid::Uuid {
            let mut bytes = [0; 16];
            self.0.lock().unwrap().fill_bytes(&mut bytes);
            Uuid::from_bytes(bytes)
        }
    }
}
