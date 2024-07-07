use crate::bytes::{Bytes, Bytes32};
use chia_bls::G1Element;
use chia_streamable_macro::streamable;
#[cfg(feature = "serde")]
use serde::{Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[streamable]
pub struct ProofOfSpace {
    challenge: Bytes32,
    #[cfg_attr(feature = "serde", serde(skip))]
    pool_public_key: Option<G1Element>,
    pool_contract_puzzle_hash: Option<Bytes32>,
    #[cfg_attr(feature = "serde", serde(skip))]
    plot_public_key: G1Element,
    size: u8,
    proof: Bytes,
}
