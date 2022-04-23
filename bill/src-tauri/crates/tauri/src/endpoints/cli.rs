// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::{InvokeContext, InvokeResponse};
use crate::Runtime;
use serde::Deserialize;
use tauri_macros::{module_command_handler, CommandModule};

/// The API descriptor.
#[derive(Deserialize, CommandModule)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  /// The get CLI matches API.
  CliMatches,
}

impl Cmd {
  #[module_command_handler(cli, "CLI definition not set under tauri.conf.json > tauri > cli (https://tauri.studio/docs/api/config#tauri.cli)")]
  fn cli_matches<R: Runtime>(context: InvokeContext<R>) -> super::Result<InvokeResponse> {
    if let Some(cli) = &context.config.tauri.cli {
      crate::api::cli::get_matches(cli, &context.package_info)
        .map(Into::into)
        .map_err(Into::into)
    } else {
      Err(crate::Error::ApiNotAllowlisted("CLI definition not set under tauri.conf.json > tauri > cli (https://tauri.studio/docs/api/config#tauri.cli)".into()).into_anyhow())
    }
  }
}

#[cfg(test)]
mod tests {
  #[tauri_macros::module_command_test(cli, "CLI definition not set under tauri.conf.json > tauri > cli (https://tauri.studio/docs/api/config#tauri.cli)")]
  #[quickcheck_macros::quickcheck]
  fn cli_matches() {
    let res = super::Cmd::cli_matches(crate::test::mock_invoke_context());
    crate::test_utils::assert_not_allowlist_error(res);
  }
}
