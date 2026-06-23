//! Kalshi venue metadata.
//!
//! The execution core, risk layer, and strategy implementations live in the
//! shared engine crate. This module just describes the venue this binary targets.

/// Display name of this venue.
pub const NAME: &str = "Kalshi";

/// Venue category.
pub const VENUE_TYPE: &str = "CFTC-regulated (US)";

/// Strategies this venue runs on the shared engine.
pub const STRATEGIES: &[&str] = &[
    "Cross-Market Arbitrage",
    "Resolution Sniper",
    "Orderbook Imbalance",
    "Market Making",
    "Directional Arbitrage",
    "Spread Farming",
    "Sports Execution",
];
