//! Genesis consensus parameters for each Zcash network.

use crate::{block, parameters::Network};

use super::network::AllParameters as _;

/// The previous block hash for the genesis block.
///
/// All known networks use the Bitcoin `null` value for the parent of the
/// genesis block. (In Bitcoin, `null` is `[0; 32]`.)
pub const GENESIS_PREVIOUS_BLOCK_HASH: block::Hash = block::Hash([0; 32]);

/// Returns the hash for the genesis block in `network`.
#[deprecated(note = "moved to AllParameters trait method")]
pub fn genesis_hash(network: Network) -> block::Hash {
    network.genesis_hash()
}
