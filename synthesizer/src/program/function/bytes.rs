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

impl<N: Network> FromBytes for Function<N> {
    /// Reads the function from a buffer.
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the function name.
        let name = Identifier::<N>::read_le(&mut reader)?;

        // Read the inputs.
        let num_inputs = u16::read_le(&mut reader)?;
        let mut inputs = Vec::with_capacity(num_inputs as usize);
        for _ in 0..num_inputs {
            inputs.push(Input::read_le(&mut reader)?);
        }

        // Read the instructions.
        let num_instructions = u32::read_le(&mut reader)?;
        if num_instructions > N::MAX_INSTRUCTIONS as u32 {
            return Err(error(format!("Failed to deserialize a function: too many instructions ({num_instructions})")));
        }
        let mut instructions = Vec::with_capacity(num_instructions as usize);
        for _ in 0..num_instructions {
            instructions.push(Instruction::read_le(&mut reader)?);
        }

        // Read the outputs.
        let num_outputs = u16::read_le(&mut reader)?;
        let mut outputs = Vec::with_capacity(num_outputs as usize);
        for _ in 0..num_outputs {
            outputs.push(Output::read_le(&mut reader)?);
        }

        // Determine if there is a finalize scope.
        let variant = u8::read_le(&mut reader)?;
        let finalize = match variant {
            0 => None,
            1 => Some((FinalizeCommand::read_le(&mut reader)?, Finalize::read_le(&mut reader)?)),
            _ => return Err(error(format!("Failed to deserialize a function: invalid finalize variant ({variant})"))),
        };

        // Initialize a new function.
        let mut function = Self::new(name);
        inputs.into_iter().try_for_each(|input| function.add_input(input)).map_err(|e| error(e.to_string()))?;
        instructions
            .into_iter()
            .try_for_each(|instruction| function.add_instruction(instruction))
            .map_err(|e| error(e.to_string()))?;
        outputs.into_iter().try_for_each(|output| function.add_output(output)).map_err(|e| error(e.to_string()))?;
        finalize.map(|(command, finalize)| function.add_finalize(command, finalize));

        Ok(function)
    }
}

impl<N: Network> ToBytes for Function<N> {
    /// Writes the function to a buffer.
    #[inline]
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Write the function name.
        self.name.write_le(&mut writer)?;

        // Write the number of inputs for the function.
        let num_inputs = self.inputs.len();
        match num_inputs <= N::MAX_INPUTS {
            true => (num_inputs as u16).write_le(&mut writer)?,
            false => return Err(error(format!("Failed to write {num_inputs} inputs as bytes"))),
        }

        // Write the inputs.
        for input in self.inputs.iter() {
            input.write_le(&mut writer)?;
        }

        // Write the number of instructions for the function.
        let num_instructions = self.instructions.len();
        match num_instructions <= N::MAX_INSTRUCTIONS {
            true => (num_instructions as u32).write_le(&mut writer)?,
            false => return Err(error(format!("Failed to write {num_instructions} instructions as bytes"))),
        }

        // Write the instructions.
        for instruction in self.instructions.iter() {
            instruction.write_le(&mut writer)?;
        }

        // Write the number of outputs for the function.
        let num_outputs = self.outputs.len();
        match num_outputs <= N::MAX_OUTPUTS {
            true => (num_outputs as u16).write_le(&mut writer)?,
            false => return Err(error(format!("Failed to write {num_outputs} outputs as bytes"))),
        }

        // Write the outputs.
        for output in self.outputs.iter() {
            output.write_le(&mut writer)?;
        }

        // If the finalize scope exists, write it.
        match &self.finalize {
            None => 0u8.write_le(&mut writer)?,
            Some((command, logic)) => {
                1u8.write_le(&mut writer)?;
                // Write the finalize scope command.
                command.write_le(&mut writer)?;
                // Write the finalize scope logic.
                logic.write_le(&mut writer)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_function_bytes() -> Result<()> {
        let function_string = r"
function main:
    input r0 as field.public;
    input r1 as field.private;
    add r0 r1 into r2;
    add r0 r1 into r3;
    add r0 r1 into r4;
    add r0 r1 into r5;
    add r0 r1 into r6;
    add r0 r1 into r7;
    add r0 r1 into r8;
    add r0 r1 into r9;
    add r0 r1 into r10;
    add r0 r1 into r11;
    output r11 as field.private;";

        let expected = Function::<CurrentNetwork>::from_str(function_string)?;
        let expected_bytes = expected.to_bytes_le()?;
        println!("String size: {:?}, Bytecode size: {:?}", function_string.as_bytes().len(), expected_bytes.len());

        let candidate = Function::<CurrentNetwork>::from_bytes_le(&expected_bytes)?;
        assert_eq!(expected.to_string(), candidate.to_string());
        assert_eq!(expected_bytes, candidate.to_bytes_le()?);
        Ok(())
    }
}
