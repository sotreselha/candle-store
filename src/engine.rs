use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Report {
    pub equity: f64,
    pub fills: u32,
    pub bars: u32,
}

pub fn ticker(symbol: &str, i: u32) -> f64 {
    let mut h = DefaultHasher::new();
    symbol.hash(&mut h);
    i.hash(&mut h);
    100.0 + (h.finish() % 10_000) as f64 / 100.0
}

pub fn allow(notional: f64, equity: f64, max_pos: f64) -> bool {
    equity > 0.0 && notional / equity <= max_pos
}

pub fn backtest(bars: u32) -> Report {
    let mut equity = 10_000.0;
    let mut fills = 0u32;
    for i in 0..bars {
        let price = ticker("ETHUSDT", i);
        let qty = 0.01;
        if !allow(qty * price, equity, 0.25) {
            continue;
        }
        equity -= qty * price * 0.0008;
        fills += 1;
    }
    Report { equity, fills, bars }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticker_positive() {
        assert!(ticker("BTCUSDT", 1) > 0.0);
    }

    #[test]
    fn risk() {
        assert!(!allow(50.0, 100.0, 0.1));
        assert!(allow(10.0, 100.0, 0.5));
    }

    #[test]
    fn run() {
        let r = backtest(16);
        assert_eq!(r.bars, 16);
        assert!(r.equity > 0.0);
    }
}
