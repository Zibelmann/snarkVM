// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> Header<N> {
    /// Initializes the genesis block header.
    pub fn genesis(transactions: &Transactions<N>) -> Result<Self> {
        // Prepare a genesis block header.
        let previous_state_root = Field::zero();
        let transactions_root = transactions.to_transactions_root()?;
        let finalize_root = transactions.to_finalize_root()?;
        let coinbase_accumulator_point = Field::zero();
        let metadata = Metadata::genesis()?;

        // Return the genesis block header.
        Self::from(previous_state_root, transactions_root, finalize_root, coinbase_accumulator_point, metadata)
    }

    /// Returns `true` if the block header is a genesis block header.
    pub fn is_genesis(&self) -> bool {
        // Ensure the previous ledger root is zero.
        self.previous_state_root == Field::zero()
            // Ensure the transactions root is nonzero.
            && self.transactions_root != Field::zero()
            // Ensure the finalize root is nonzero.
            && self.finalize_root != Field::zero()
            // Ensure the coinbase accumulator point is zero.
            && self.coinbase_accumulator_point == Field::zero()
            // Ensure the metadata is a genesis metadata.
            && self.metadata.is_genesis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    /// Returns the expected block header size by summing its subcomponent sizes.
    /// Update this method if the contents of a block header have changed.
    fn get_expected_size<N: Network>() -> usize {
        // Previous state root, transactions root, finalize root, and accumulator point size.
        (Field::<N>::size_in_bytes() * 4)
            // Metadata size.
            + 1 + 8 + 4 + 8 + 16 + 8 + 8 + 8 + 8 + 8
            // Add an additional 3 bytes for versioning.
            + 1 + 2
    }

    #[test]
    fn test_genesis_header_size() {
        let mut rng = TestRng::default();

        // Prepare the expected size.
        let expected_size = get_expected_size::<CurrentNetwork>();
        // Prepare the genesis block header.
        let genesis_header = *crate::vm::test_helpers::sample_genesis_block(&mut rng).header();
        // Ensure the size of the genesis block header is correct.
        assert_eq!(expected_size, genesis_header.to_bytes_le().unwrap().len());
    }

    #[test]
    fn test_genesis_header() {
        let mut rng = TestRng::default();

        // Prepare the genesis block header.
        let header = *crate::vm::test_helpers::sample_genesis_block(&mut rng).header();
        // Ensure the block header is a genesis block header.
        assert!(header.is_genesis());

        // Ensure the genesis block contains the following.
        assert_eq!(header.previous_state_root(), Field::zero());
        assert_eq!(header.coinbase_accumulator_point(), Field::zero());
        assert_eq!(header.network(), CurrentNetwork::ID);
        assert_eq!(header.round(), 0);
        assert_eq!(header.height(), 0);
        assert_eq!(header.total_supply_in_microcredits(), CurrentNetwork::STARTING_SUPPLY);
        assert_eq!(header.cumulative_weight(), 0);
        assert_eq!(header.coinbase_target(), CurrentNetwork::GENESIS_COINBASE_TARGET);
        assert_eq!(header.proof_target(), CurrentNetwork::GENESIS_PROOF_TARGET);
        assert_eq!(header.last_coinbase_target(), CurrentNetwork::GENESIS_COINBASE_TARGET);
        assert_eq!(header.last_coinbase_timestamp(), CurrentNetwork::GENESIS_TIMESTAMP);
        assert_eq!(header.timestamp(), CurrentNetwork::GENESIS_TIMESTAMP);

        // Ensure the genesis block does *not* contain the following.
        assert_ne!(header.transactions_root(), Field::zero());
    }
}
