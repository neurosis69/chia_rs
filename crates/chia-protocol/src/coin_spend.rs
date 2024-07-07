use chia_streamable_macro::streamable;

use crate::coin::Coin;
use crate::program::Program;
#[cfg(feature = "serde")]
use serde::{Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[streamable]
pub struct CoinSpend {
    coin: Coin,
    puzzle_reveal: Program,
    solution: Program,
}
