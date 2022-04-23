// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Types and functions related to desktop notifications.

#[cfg(windows)]
use std::path::MAIN_SEPARATOR;

/// The desktop notification definition.
///
/// Allows you to construct a Notification data and send it.
///
/// # Examples
/// ```rust,no_run
/// use tauri::api::notification::Notification;
/// // first we build the application to access the Tauri configuration
/// let app = tauri::Builder::default()
///   // on an actual app, remove the string argument
///   .build(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
///   .expect("error while building tauri application");
///
/// // shows a notification with the given title and body
/// Notification::new(&app.config().tauri.bundle.identifier)
///   .title("New message")
///   .body("You've got a new message.")
///   .show();
///
/// // run the app
/// app.run(|_app_handle, _event| {});
/// ```
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Notification {
  /// The notification body.
  body: Option<String>,
  /// The notification title.
  title: Option<String>,
  /// The notification icon.
  icon: Option<String>,
  /// The notification identifier
  identifier: String,
}

impl Notification {
  /// Initializes a instance of a Notification.
  pub fn new(identifier: impl Into<String>) -> Self {
    Self {
      identifier: identifier.into(),
      ..Default::default()
    }
  }

  /// Sets the notification body.
  #[must_use]
  pub fn body(mut self, body: impl Into<String>) -> Self {
    self.body = Some(body.into());
    self
  }

  /// Sets the notification title.
  #[must_use]
  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.title = Some(title.into());
    self
  }

  /// Sets the notification icon.
  #[must_use]
  pub fn icon(mut self, icon: impl Into<String>) -> Self {
    self.icon = Some(icon.into());
    self
  }

  /// Shows the notification.
  pub fn show(self) -> crate::api::Result<()> {
    let mut notification = notify_rust::Notification::new();
    if let Some(body) = self.body {
      notification.body(&body);
    }
    if let Some(title) = self.title {
      notification.summary(&title);
    }
    if let Some(icon) = self.icon {
      notification.icon(&icon);
    }
    #[cfg(windows)]
    {
      let exe = tauri_utils::platform::current_exe()?;
      let exe_dir = exe.parent().expect("failed to get exe directory");
      let curr_dir = exe_dir.display().to_string();
      // set the notification's System.AppUserModel.ID only when running the installed app
      if !(curr_dir.ends_with(format!("{S}target{S}debug", S = MAIN_SEPARATOR).as_str())
        || curr_dir.ends_with(format!("{S}target{S}release", S = MAIN_SEPARATOR).as_str()))
      {
        notification.app_id(&self.identifier);
      }
    }

    crate::async_runtime::spawn(async move {
      notification.show().expect("failed to show notification");
    });

    Ok(())
  }
}
