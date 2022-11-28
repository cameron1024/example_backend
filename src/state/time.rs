use std::fmt::Debug;

use chrono::{DateTime, Utc};

pub trait Time: Debug + Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTime;

impl Time for SystemTime {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(test)]
pub mod mock {
    use chrono::{DateTime, TimeZone, Utc};
    use once_cell::sync::Lazy;

    pub static DEFAULT_DATE_TIME: Lazy<DateTime<Utc>> =
        Lazy::new(|| Utc.ymd(2020, 1, 1).and_hms(0, 0, 0));

    #[derive(Debug, Clone, Copy)]
    pub struct MockTime(pub DateTime<Utc>);


    impl Default for MockTime {
        fn default() -> Self {
            Self(*DEFAULT_DATE_TIME)
        }
    }

    impl super::Time for MockTime {
        fn now(&self) -> DateTime<Utc> {
            self.0
        }
    }
}
