#[derive(Debug, Clone, PartialEq)]
pub struct RetryConf {
    max: u64,
    multiplier: f64,
}

impl Default for RetryConf {
    fn default() -> RetryConf {
        RetryConf {
            max: 10,
            multiplier: 5_f64,
        }
    }
}

impl RetryConf {
    pub fn new() -> RetryConf {
        Default::default()
    }

    pub fn max(mut self, max: u64) -> RetryConf {
        self.max = max;
        self
    }

    pub fn multiplier(mut self, multiplier: f64) -> RetryConf {
        self.multiplier = multiplier;
        self
    }

    pub fn build(self) -> (u64, f64) {
        (self.max, self.multiplier)
    }
}
