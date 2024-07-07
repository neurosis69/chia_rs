use chia_streamable_macro::streamable;

use crate::Bytes32;
#[cfg(feature = "serde")]
use serde::{Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[streamable]
pub struct PoolTarget {
    puzzle_hash: Bytes32,
    max_height: u32, // A max height of 0 means it is valid forever
}
