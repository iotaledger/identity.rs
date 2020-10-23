use core::time::Duration;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PromoteOptions {
    pub(crate) timeout: Option<Duration>,
    pub(crate) interval: Duration,
    pub(crate) ts_depth: u8,
}

impl PromoteOptions {
    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60 * 3); // promote for 3m
    const DEFAULT_INTERVAL: Duration = Duration::from_secs(5); // promote every 5s

    const DEFAULT_TIP_SELECTION_DEPTH: u8 = 2;

    pub const fn new() -> Self {
        Self {
            timeout: Some(Self::DEFAULT_TIMEOUT),
            interval: Self::DEFAULT_INTERVAL,
            ts_depth: Self::DEFAULT_TIP_SELECTION_DEPTH,
        }
    }

    pub fn timeout(mut self, value: impl Into<Option<Duration>>) -> Self {
        self.timeout = value.into();
        self
    }

    pub fn interval(mut self, value: Duration) -> Self {
        self.interval = value;
        self
    }

    pub fn ts_depth(mut self, value: u8) -> Self {
        self.ts_depth = value;
        self
    }
}

impl Default for PromoteOptions {
    fn default() -> Self {
        Self::new()
    }
}
