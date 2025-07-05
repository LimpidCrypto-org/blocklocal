use std::{fmt::Display, process::Output};

use serde_json::Value;

use crate::errors::{BLError, Result};

use super::errors::BLOutputError;

pub struct BLOutput {
    output: Output,
}

impl BLOutputFn for BLOutput {
    fn get(&self) -> &Output {
        &self.output
    }

    fn json(&self) -> Result<Value> {
        let output = self.get();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            return Err(BLOutputError::StdErr(stderr.into()).into());
        }

        let json: Value = serde_json::from_str(&stdout)?;

        Ok(json)
    }
}

impl Display for BLOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.output.stderr.is_empty() {
            write!(f, "{}", String::from_utf8_lossy(&self.output.stderr))
        } else {
            write!(f, "{}", String::from_utf8_lossy(&self.output.stdout))
        }
    }
}

pub trait BLOutputFn {
    /// Returns the output of the command
    fn get(&self) -> &Output;
    /// Returns the output of the command as a JSON value
    fn json(&self) -> Result<Value>;
}

impl From<Output> for BLOutput {
    fn from(output: Output) -> Self {
        BLOutput { output }
    }
}

impl TryFrom<BLOutputError> for BLOutput {
    type Error = BLError;

    fn try_from(error: BLOutputError) -> Result<Self> {
        match error {
            BLOutputError::StdErr(e) => Ok(Output {
                stdout: Vec::new(),
                stderr: e.into_bytes(),
                status: Default::default(),
            }
            .into()),
            _ => Err(error.into()),
        }
    }
}
