// SPDX-License-Identifier: Apache-2.0

use clap::Args;

/// Retrieve information about a file tree on an Enarx package host.
#[derive(Args, Debug)]
pub struct Options {}

impl Options {
    pub fn execute(self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
