use crate::block::Height;

/// The first halving height in the testnet is at block height `1_116_000`
/// as specified in [protocol specification §7.10.1][7.10.1]
///
/// [7.10.1]: https://zips.z.cash/protocol/protocol.pdf#zip214fundingstreams
pub const FIRST_HALVING_TESTNET: Height = Height(1_116_000);
