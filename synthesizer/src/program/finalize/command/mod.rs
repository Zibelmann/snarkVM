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

mod finalize;
pub use finalize::*;

mod get;
pub use get::*;

mod get_or_use;
pub use get_or_use::*;

mod set;
pub use set::*;

use crate::{program::Instruction, FinalizeOperation, FinalizeRegisters, FinalizeStorage, FinalizeStore, Stack};
use console::network::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Command<N: Network> {
    /// Evaluates the instruction.
    Instruction(Instruction<N>),
    /// Gets the value stored at the `key` operand in `mapping` and stores the result into `destination`.
    Get(Get<N>),
    /// Gets the value stored at the `key` operand in `mapping` and stores the result into `destination`.
    /// If the key is not present, `default` is stored `destination`.
    GetOrUse(GetOrUse<N>),
    /// Sets the value stored at the `key` operand in the `mapping` to `value`.
    Set(Set<N>),
}

impl<N: Network> Command<N> {
    /// Finalizes the command.
    #[inline]
    pub fn finalize<P: FinalizeStorage<N>>(
        &self,
        stack: &Stack<N>,
        store: &FinalizeStore<N, P>,
        registers: &mut FinalizeRegisters<N>,
    ) -> Result<Option<FinalizeOperation<N>>> {
        match self {
            // Finalize the instruction, and return no finalize operation.
            Command::Instruction(instruction) => instruction.finalize(stack, registers).map(|_| None),
            // Finalize the 'get' command, and return no finalize operation.
            Command::Get(get) => get.finalize(stack, store, registers).map(|_| None),
            // Finalize the 'get.or_use' command, and return the (optional) finalize operation.
            Command::GetOrUse(get_or_use) => get_or_use.finalize(stack, store, registers).map(|_| None),
            // Finalize the 'set' command, and return the finalize operation.
            Command::Set(set) => set.finalize(stack, store, registers).map(Some),
        }
    }
}

impl<N: Network> FromBytes for Command<N> {
    /// Reads the command from a buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the variant.
        let variant = u8::read_le(&mut reader)?;
        match variant {
            // Read the instruction.
            0 => Ok(Self::Instruction(Instruction::read_le(&mut reader)?)),
            // Read the `get` operation.
            1 => Ok(Self::Get(Get::read_le(&mut reader)?)),
            // Read the `get.or_use` operation.
            2 => Ok(Self::GetOrUse(GetOrUse::read_le(&mut reader)?)),
            // Read the `set` operation.
            3 => Ok(Self::Set(Set::read_le(&mut reader)?)),
            // Invalid variant.
            4.. => Err(error(format!("Invalid command variant: {variant}"))),
        }
    }
}

impl<N: Network> ToBytes for Command<N> {
    /// Writes the command to a buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        match self {
            Self::Instruction(instruction) => {
                // Write the variant.
                0u8.write_le(&mut writer)?;
                // Write the instruction.
                instruction.write_le(&mut writer)
            }
            Self::Get(get) => {
                // Write the variant.
                1u8.write_le(&mut writer)?;
                // Write the `get` operation.
                get.write_le(&mut writer)
            }
            Self::GetOrUse(get_or_use) => {
                // Write the variant.
                2u8.write_le(&mut writer)?;
                // Write the defaulting `get` operation.
                get_or_use.write_le(&mut writer)
            }
            Self::Set(set) => {
                // Write the variant.
                3u8.write_le(&mut writer)?;
                // Write the set.
                set.write_le(&mut writer)
            }
        }
    }
}

impl<N: Network> Parser for Command<N> {
    /// Parses the string into the command.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the command.
        // Note that the order of the parsers is important.
        alt((
            map(GetOrUse::parse, |get_or_use| Self::GetOrUse(get_or_use)),
            map(Get::parse, |get| Self::Get(get)),
            map(Set::parse, |set| Self::Set(set)),
            map(Instruction::parse, |instruction| Self::Instruction(instruction)),
        ))(string)
    }
}

impl<N: Network> FromStr for Command<N> {
    type Err = Error;

    /// Parses the string into the command.
    #[inline]
    fn from_str(string: &str) -> Result<Self> {
        match Self::parse(string) {
            Ok((remainder, object)) => {
                // Ensure the remainder is empty.
                ensure!(remainder.is_empty(), "Failed to parse string. Found invalid character in: \"{remainder}\"");
                // Return the object.
                Ok(object)
            }
            Err(error) => bail!("Failed to parse string. {error}"),
        }
    }
}

impl<N: Network> Debug for Command<N> {
    /// Prints the command as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for Command<N> {
    /// Prints the command as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Instruction(instruction) => Display::fmt(instruction, f),
            Self::Get(get) => Display::fmt(get, f),
            Self::GetOrUse(get_or_use) => Display::fmt(get_or_use, f),
            Self::Set(set) => Display::fmt(set, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_command_bytes() {
        // Decrement
        let expected = "decrement object[r0] by r1;";
        Command::<CurrentNetwork>::parse(expected).unwrap_err();

        // Instruction
        let expected = "add r0 r1 into r2;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        let bytes = command.to_bytes_le().unwrap();
        assert_eq!(command, Command::from_bytes_le(&bytes).unwrap());

        // Increment
        let expected = "increment object[r0] by r1;";
        Command::<CurrentNetwork>::parse(expected).unwrap_err();

        // Get
        let expected = "get object[r0] into r1;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        let bytes = command.to_bytes_le().unwrap();
        assert_eq!(command, Command::from_bytes_le(&bytes).unwrap());

        // GetOr
        let expected = "get.or_use object[r0] r1 into r2;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        let bytes = command.to_bytes_le().unwrap();
        assert_eq!(command, Command::from_bytes_le(&bytes).unwrap());

        // Set
        let expected = "set r0 into object[r1];";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        let bytes = command.to_bytes_le().unwrap();
        assert_eq!(command, Command::from_bytes_le(&bytes).unwrap());
    }

    #[test]
    fn test_command_parse() {
        // Decrement
        let expected = "decrement object[r0] by r1;";
        Command::<CurrentNetwork>::parse(expected).unwrap_err();

        // Instruction
        let expected = "add r0 r1 into r2;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        assert_eq!(Command::Instruction(Instruction::from_str(expected).unwrap()), command);
        assert_eq!(expected, command.to_string());

        // Increment
        let expected = "increment object[r0] by r1;";
        Command::<CurrentNetwork>::parse(expected).unwrap_err();

        // Get
        let expected = "get object[r0] into r1;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        assert_eq!(Command::Get(Get::from_str(expected).unwrap()), command);
        assert_eq!(expected, command.to_string());

        // GetOr
        let expected = "get.or_use object[r0] r1 into r2;";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        assert_eq!(Command::GetOrUse(GetOrUse::from_str(expected).unwrap()), command);
        assert_eq!(expected, command.to_string());

        // Set
        let expected = "set r0 into object[r1];";
        let command = Command::<CurrentNetwork>::parse(expected).unwrap().1;
        assert_eq!(Command::Set(Set::from_str(expected).unwrap()), command);
        assert_eq!(expected, command.to_string());
    }
}
