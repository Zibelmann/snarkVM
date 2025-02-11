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

use crate::snark::marlin::ahp::AHPError;
use snarkvm_fields::ConstraintFieldError;
use snarkvm_r1cs::SynthesisError;

#[derive(Debug, Error)]
pub enum SNARKError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),

    #[error("{}", _0)]
    ConstraintFieldError(#[from] ConstraintFieldError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Expected a circuit-specific SRS in SNARK")]
    ExpectedCircuitSpecificSRS,

    #[error("{}", _0)]
    Message(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),

    #[error("Batch size was zero; must be at least 1")]
    EmptyBatch,

    #[error("Batch size was different between public input and proof")]
    BatchSizeMismatch,

    #[error("Circuit not found")]
    CircuitNotFound,

    #[error("terminated")]
    Terminated,
}

impl From<AHPError> for SNARKError {
    fn from(err: AHPError) -> Self {
        SNARKError::Crate("AHPError", format!("{err:?}"))
    }
}

impl From<crate::polycommit::PCError> for SNARKError {
    fn from(err: crate::polycommit::PCError) -> Self {
        match err {
            crate::polycommit::PCError::Terminated => SNARKError::Terminated,
            err => SNARKError::Crate("PCError", format!("{err:?}")),
        }
    }
}
