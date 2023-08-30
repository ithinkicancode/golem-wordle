use chrono::{DateTime, Utc};

pub(crate) type Gmt = DateTime<Utc>;

pub trait Clock {
    fn now(&self) -> Gmt;
}

pub struct RealClock;

impl Clock for RealClock {
    fn now(&self) -> Gmt {
        Utc::now()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use chrono::{
        offset::TimeZone, Duration, Utc,
    };
    use std::cell::RefCell;

    pub(crate) struct TestClock {
        time: RefCell<Gmt>,
    }

    impl Default for TestClock {
        fn default() -> Self {
            Self::new(Utc::now())
        }
    }

    impl TestClock {
        pub(crate) fn new(
            start_time: Gmt,
        ) -> Self {
            Self {
                time: RefCell::new(
                    start_time,
                ),
            }
        }

        pub(crate) fn init(
            year: i32,
            month: u32,
            day: u32,
            hour: u32,
            minute: u32,
        ) -> Self {
            let time = Utc
                .with_ymd_and_hms(
                    year, month, day,
                    hour, minute, 0,
                )
                .unwrap();

            Self::new(time)
        }

        pub(crate) fn advance(
            &self,
            duration: Duration,
        ) {
            let time =
                *self.time.borrow()
                    + duration;

            self.reset(time);
        }

        pub(crate) fn reset(
            &self,
            time: Gmt,
        ) {
            let mut value =
                self.time.borrow_mut();

            *value = time;
        }
    }

    impl Clock for TestClock {
        fn now(&self) -> Gmt {
            self.time
                .borrow()
                .to_owned()
        }
    }
}
