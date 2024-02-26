#![forbid(unsafe_code)]
//! Network methods for fetching blockchain vectors.
//!

use std::collections::BTreeMap;

use crate::{
    block::Block,
    parameters::Network,
    serialization::{SerializationError, ZcashDeserializeInto},
};

use zebra_test::vectors::{
    BLOCK_MAINNET_1046400_BYTES, BLOCK_MAINNET_653599_BYTES, BLOCK_MAINNET_982681_BYTES,
    BLOCK_TESTNET_1116000_BYTES, BLOCK_TESTNET_583999_BYTES, BLOCK_TESTNET_925483_BYTES,
    CONTINUOUS_MAINNET_BLOCKS, CONTINUOUS_TESTNET_BLOCKS, MAINNET_BLOCKS,
    MAINNET_FINAL_SAPLING_ROOTS, MAINNET_FINAL_SPROUT_ROOTS,
    SAPLING_FINAL_ROOT_MAINNET_1046400_BYTES, SAPLING_FINAL_ROOT_TESTNET_1116000_BYTES,
    TESTNET_BLOCKS, TESTNET_FINAL_SAPLING_ROOTS, TESTNET_FINAL_SPROUT_ROOTS,
};

/// Network methods for fetching blockchain state.
impl Network {
    /// Returns true if network is of type Mainnet.
    pub fn is_mainnet(&self) -> bool {
        match self {
            Network::Mainnet => true,
            Network::Testnet => false,
        }
    }
    /// Returns true if network is type default Testnet.
    pub fn is_default_testnet(&self) -> bool {
        match self {
            Network::Mainnet => false,
            Network::Testnet => true,
        }
    }

    /// Returns iterator over blocks.
    pub fn get_block_iter(&self) -> std::collections::btree_map::Iter<'static, u32, &'static [u8]> {
        if self.is_mainnet() {
            MAINNET_BLOCKS.iter()
        } else {
            TESTNET_BLOCKS.iter()
        }
    }

    /// Return the map of heights to blocks
    pub fn get_block_map(&self) -> &BTreeMap<u32, &'static [u8]> {
        if self.is_mainnet() {
            &*zebra_test::vectors::MAINNET_BLOCKS
        } else {
            &*zebra_test::vectors::TESTNET_BLOCKS
        }
    }

    /// Returns genesis block for chain.
    pub fn get_gen_block(&self) -> std::option::Option<&[u8]> {
        if self.is_mainnet() {
            MAINNET_BLOCKS.get(&0)
        } else {
            TESTNET_BLOCKS.get(&0)
        }
        .cloned()
    }

    /// Returns block bytes
    pub fn get_block_bytes(
        &self,
        main_bytes: u32,
        test_bytes: u32,
    ) -> Result<Block, SerializationError> {
        if self.is_mainnet() {
            match main_bytes {
                653_599 => BLOCK_MAINNET_653599_BYTES.zcash_deserialize_into(),
                982_681 => BLOCK_MAINNET_982681_BYTES.zcash_deserialize_into(),
                _ => Err(SerializationError::NotACachedMainNetBlock(main_bytes)),
            }
        } else {
            match test_bytes {
                583_999 => BLOCK_TESTNET_583999_BYTES.zcash_deserialize_into(),
                925_483 => BLOCK_TESTNET_925483_BYTES.zcash_deserialize_into(),
                _ => Err(SerializationError::NotACachedTestNetBlock(test_bytes)),
            }
        }
    }

    /// Returns iterator over blockchain.
    pub fn get_blockchain_iter(&self) -> std::collections::btree_map::Iter<'_, u32, &[u8]> {
        if self.is_mainnet() {
            CONTINUOUS_MAINNET_BLOCKS.iter()
        } else {
            CONTINUOUS_TESTNET_BLOCKS.iter()
        }
    }

    /// Returns BTreemap of blockchain, keys are heights, and values are blocks.
    /// Why not represent as a vec?
    pub fn get_blockchain_map(&self) -> &BTreeMap<u32, &'static [u8]> {
        if self.is_mainnet() {
            &CONTINUOUS_MAINNET_BLOCKS
        } else {
            &CONTINUOUS_TESTNET_BLOCKS
        }
    }

    /// Returns iterator over blocks and sapling roots.
    pub fn get_block_sapling_roots_iter(
        &self,
    ) -> (
        std::collections::btree_map::Iter<'_, u32, &[u8]>,
        std::collections::BTreeMap<u32, &[u8; 32]>,
    ) {
        if self.is_mainnet() {
            (MAINNET_BLOCKS.iter(), MAINNET_FINAL_SAPLING_ROOTS.clone())
        } else {
            (TESTNET_BLOCKS.iter(), TESTNET_FINAL_SAPLING_ROOTS.clone())
        }
    }

    /// Returns BTreemap of blocks and sapling roots.
    pub fn get_block_sapling_roots_map(
        &self,
    ) -> (
        &std::collections::BTreeMap<u32, &'static [u8]>,
        &std::collections::BTreeMap<u32, &'static [u8; 32]>,
    ) {
        if self.is_mainnet() {
            (&*MAINNET_BLOCKS, &*MAINNET_FINAL_SAPLING_ROOTS)
        } else {
            (&*TESTNET_BLOCKS, &*TESTNET_FINAL_SAPLING_ROOTS)
        }
    }

    /// Returns block and sapling root bytes
    pub fn get_block_sapling_roots_bytes(
        &self,
        main_bytes: u32,
        test_bytes: u32,
    ) -> Result<(&[u8], [u8; 32]), SerializationError> {
        if self.is_mainnet() {
            match main_bytes {
                1_046_400 => Ok((
                    &BLOCK_MAINNET_1046400_BYTES[..],
                    *SAPLING_FINAL_ROOT_MAINNET_1046400_BYTES,
                )),
                _ => Err(SerializationError::NotACachedMainNetSaplingRootBytes(
                    main_bytes,
                )),
            }
        } else {
            match test_bytes {
                1_116_000 => Ok((
                    &BLOCK_TESTNET_1116000_BYTES[..],
                    *SAPLING_FINAL_ROOT_TESTNET_1116000_BYTES,
                )),
                _ => Err(SerializationError::NotACachedTestNetSaplingRootBytes(
                    test_bytes,
                )),
            }
        }
    }

    /// Returns BTreemap of blocks and sprout roots, and last split height.
    pub fn get_block_sprout_roots_height(
        &self,
    ) -> (
        &std::collections::BTreeMap<u32, &'static [u8]>,
        &std::collections::BTreeMap<u32, &'static [u8; 32]>,
        u32,
    ) {
        // The mainnet block height at which the first JoinSplit occurred.
        const MAINNET_FIRST_JOINSPLIT_HEIGHT: u32 = 396;

        // The testnet block height at which the first JoinSplit occurred.
        const TESTNET_FIRST_JOINSPLIT_HEIGHT: u32 = 2259;
        if self.is_mainnet() {
            (
                &*MAINNET_BLOCKS,
                &*MAINNET_FINAL_SPROUT_ROOTS,
                MAINNET_FIRST_JOINSPLIT_HEIGHT,
            )
        } else {
            (
                &*TESTNET_BLOCKS,
                &*TESTNET_FINAL_SPROUT_ROOTS,
                TESTNET_FIRST_JOINSPLIT_HEIGHT,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        block::Block,
        parameters::Network,
        serialization::{SerializationError, ZcashDeserializeInto},
    };

    #[test]
    fn is_mainnet() {
        let mainnet = Network::Mainnet;
        assert!(
            mainnet.is_mainnet(),
            "Mainnet should return true for is_mainnet"
        );

        let testnet = Network::Testnet;
        assert!(
            !testnet.is_mainnet(),
            "Testnet should return false for is_mainnet"
        );
    }

    #[test]
    fn is_default_testnet() {
        let mainnet = Network::Mainnet;
        assert!(
            !mainnet.is_default_testnet(),
            "Mainnet should return false for is_testnet"
        );

        let testnet = Network::Testnet;
        assert!(
            testnet.is_default_testnet(),
            "Testnet should return true for is_testnet"
        );
    }

    #[test]
    fn get_block_bytes() {
        let network = Network::Mainnet;
        let result = network.get_block_bytes(0, 583999);
        assert!(matches!(
            result,
            Err(SerializationError::NotACachedMainNetBlock(0))
        ));
        let result = network.get_block_bytes(653599, 0).unwrap();
        let _correct_main_bytes: Block =
            BLOCK_MAINNET_653599_BYTES.zcash_deserialize_into().unwrap();
        assert!(matches!(result, _correct_main_bytes));

        let network = Network::Testnet;
        let result = network.get_block_bytes(653599, 0);
        assert!(matches!(
            result,
            Err(SerializationError::NotACachedTestNetBlock(0))
        ));
        let network = Network::Testnet;
        let result = network.get_block_bytes(0, 583999);
        let _correct_test_bytes: Block =
            BLOCK_TESTNET_583999_BYTES.zcash_deserialize_into().unwrap();
        assert!(matches!(result, _correct_test_bytes));
    }

    #[test]
    fn get_block_sapling_roots_bytes() {
        let network = Network::Mainnet;
        let result = network.get_block_sapling_roots_bytes(0, 1116000);
        assert!(matches!(
            result,
            Err(SerializationError::NotACachedMainNetSaplingRootBytes(0))
        ));
        let result = network.get_block_sapling_roots_bytes(1046400, 0).unwrap();
        let _correct_main_result: (&[u8], [u8; 32]) = (
            &BLOCK_MAINNET_1046400_BYTES[..],
            *SAPLING_FINAL_ROOT_MAINNET_1046400_BYTES,
        );
        assert!(matches!(result, _correct_main_result));

        let network = Network::Testnet;
        let result = network.get_block_sapling_roots_bytes(1046400, 0);
        assert!(matches!(
            result,
            Err(SerializationError::NotACachedTestNetSaplingRootBytes(0))
        ));
        let result = network.get_block_sapling_roots_bytes(0, 1116000).unwrap();
        let _correct_test_result: (&[u8], [u8; 32]) = (
            &BLOCK_TESTNET_1116000_BYTES[..],
            *SAPLING_FINAL_ROOT_TESTNET_1116000_BYTES,
        );
        assert!(matches!(result, _correct_test_result));
    }
}
