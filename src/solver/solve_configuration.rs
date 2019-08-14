//! Configuration about which strategies, in which order, are to be used when solving.

use strategies::{Strategy, ALL_STRATEGIES};

/// Configuration that determines how solving should proceed.
pub struct SolveConfiguration {
    strategies: Vec<Strategy>,
}

impl SolveConfiguration {

    /// Create a `SolveConfiguration` with all strategies enabled, in default order.
    pub fn with_all_strategies() -> SolveConfiguration {
        SolveConfiguration { strategies: ALL_STRATEGIES.to_vec() }
    }

    /// Create a `SolveConfiguration` with all strategies enabled, except the given strategies, in
    /// default order.
    pub fn without_strategies(strategies: Vec<Strategy>) -> SolveConfiguration {
        SolveConfiguration::with_strategies(
            ALL_STRATEGIES.iter().filter(|&x| !strategies.contains(x)).map(|x| *x).collect()
        )
    }

    /// Create a `SolveConfiguration` with the given strategies in the given order.
    pub fn with_strategies(strategies: Vec<Strategy>) -> SolveConfiguration {
        SolveConfiguration { strategies }
    }

    /// Get a slice over the allowed strategies.
    pub fn strategies(&self) -> &[Strategy] {
        &self.strategies
    }
}
