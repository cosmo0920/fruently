#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryConf {
    max: u32,
    timeout: u32,
}

impl Default for RetryConf {
    fn default() -> RetryConf {
        RetryConf {
            max: 10,
            timeout: 500,
        }
    }
}

impl RetryConf {
    pub fn new() -> RetryConf {
        Default::default()
    }

    pub fn max(mut self, max: u32) -> RetryConf {
        self.max = max;
        self
    }

    pub fn timeout(mut self, timeout: u32) -> RetryConf {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> (u32, u32) {
        (self.max, self.timeout)
    }
}
