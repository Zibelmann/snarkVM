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

mod bytes;
mod encrypt;
mod equal;
mod find;
mod from_bits;
mod from_fields;
mod num_randomizers;
mod parse;
mod serialize;
mod size_in_fields;
mod to_bits;
mod to_fields;

use crate::{Ciphertext, Identifier, Literal};
use snarkvm_console_network::Network;
use snarkvm_console_types::prelude::*;

use indexmap::IndexMap;
use once_cell::sync::OnceCell;

#[derive(Clone)]
pub enum Plaintext<N: Network> {
    /// A literal.
    Literal(Literal<N>, OnceCell<Vec<bool>>),
    /// A struct.
    Struct(IndexMap<Identifier<N>, Plaintext<N>>, OnceCell<Vec<bool>>),
}

impl<N: Network> From<Literal<N>> for Plaintext<N> {
    /// Returns a new `Plaintext` from a `Literal`.
    fn from(literal: Literal<N>) -> Self {
        Self::Literal(literal, OnceCell::new())
    }
}

impl<N: Network> From<&Literal<N>> for Plaintext<N> {
    /// Returns a new `Plaintext` from a `Literal`.
    fn from(literal: &Literal<N>) -> Self {
        Self::Literal(literal.clone(), OnceCell::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;
    use snarkvm_console_types::Field;

    use core::str::FromStr;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_plaintext() -> Result<()> {
        let mut rng = TestRng::default();

        let value = Plaintext::<CurrentNetwork>::from_str("true")?;
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value =
            Plaintext::<CurrentNetwork>::Literal(Literal::Field(Field::new(Uniform::rand(&mut rng))), OnceCell::new());
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value = Plaintext::<CurrentNetwork>::Struct(
            IndexMap::from_iter(
                vec![
                    (Identifier::from_str("a")?, Plaintext::<CurrentNetwork>::from_str("true")?),
                    (
                        Identifier::from_str("b")?,
                        Plaintext::<CurrentNetwork>::Literal(
                            Literal::Field(Field::new(Uniform::rand(&mut rng))),
                            OnceCell::new(),
                        ),
                    ),
                ]
                .into_iter(),
            ),
            OnceCell::new(),
        );
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());

        let value = Plaintext::<CurrentNetwork>::Struct(
            IndexMap::from_iter(
                vec![
                    (Identifier::from_str("a")?, Plaintext::<CurrentNetwork>::from_str("true")?),
                    (
                        Identifier::from_str("b")?,
                        Plaintext::<CurrentNetwork>::Struct(
                            IndexMap::from_iter(
                                vec![
                                    (Identifier::from_str("c")?, Plaintext::<CurrentNetwork>::from_str("true")?),
                                    (
                                        Identifier::from_str("d")?,
                                        Plaintext::<CurrentNetwork>::Struct(
                                            IndexMap::from_iter(
                                                vec![
                                                    (
                                                        Identifier::from_str("e")?,
                                                        Plaintext::<CurrentNetwork>::from_str("true")?,
                                                    ),
                                                    (
                                                        Identifier::from_str("f")?,
                                                        Plaintext::<CurrentNetwork>::Literal(
                                                            Literal::Field(Field::new(Uniform::rand(&mut rng))),
                                                            OnceCell::new(),
                                                        ),
                                                    ),
                                                ]
                                                .into_iter(),
                                            ),
                                            OnceCell::new(),
                                        ),
                                    ),
                                    (
                                        Identifier::from_str("g")?,
                                        Plaintext::<CurrentNetwork>::Literal(
                                            Literal::Field(Field::new(Uniform::rand(&mut rng))),
                                            OnceCell::new(),
                                        ),
                                    ),
                                ]
                                .into_iter(),
                            ),
                            OnceCell::new(),
                        ),
                    ),
                    (
                        Identifier::from_str("h")?,
                        Plaintext::<CurrentNetwork>::Literal(
                            Literal::Field(Field::new(Uniform::rand(&mut rng))),
                            OnceCell::new(),
                        ),
                    ),
                ]
                .into_iter(),
            ),
            OnceCell::new(),
        );
        assert_eq!(value.to_bits_le(), Plaintext::<CurrentNetwork>::from_bits_le(&value.to_bits_le())?.to_bits_le());
        Ok(())
    }
}
