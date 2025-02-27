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

/// Compiles an Aleo program.
#[derive(Debug, Parser)]
pub struct Build {
    /// Uses the specified endpoint.
    #[clap(long)]
    endpoint: Option<String>,
    /// Toggles offline mode.
    #[clap(long)]
    offline: bool,
}

impl Build {
    /// Compiles an Aleo program with the specified name.
    pub fn parse(self) -> Result<String> {
        // Derive the program directory path.
        let path = std::env::current_dir()?;

        // Load the package.
        let package = Package::open(&path)?;

        // Build the package, if the package requires building.
        package.build::<Aleo>(self.endpoint)?;

        // package.build::<Aleo>(match self.offline {
        //     true => None,
        //     false => Some(endpoint.unwrap_or("https://vm.aleo.org/testnet3/build".to_string())),
        // })?;

        // Prepare the path string.
        let path_string = format!("(in \"{}\")", path.display());

        // Log the build as successful.
        Ok(format!("✅ Built '{}' {}", package.program_id().to_string().bold(), path_string.dimmed()))
    }
}
