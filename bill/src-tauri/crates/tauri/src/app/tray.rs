// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub use crate::{
  runtime::{
    menu::{
      MenuHash, MenuId, MenuIdRef, MenuUpdate, SystemTrayMenu, SystemTrayMenuEntry, TrayHandle,
    },
    window::dpi::{PhysicalPosition, PhysicalSize},
    SystemTray, TrayIcon,
  },
  Runtime,
};

use tauri_macros::default_runtime;

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

pub(crate) fn get_menu_ids(map: &mut HashMap<MenuHash, MenuId>, menu: &SystemTrayMenu) {
  for item in &menu.items {
    match item {
      SystemTrayMenuEntry::CustomItem(c) => {
        map.insert(c.id, c.id_str.clone());
      }
      SystemTrayMenuEntry::Submenu(s) => get_menu_ids(map, &s.inner),
      _ => {}
    }
  }
}

/// System tray event.
#[cfg_attr(doc_cfg, doc(cfg(feature = "system-tray")))]
#[non_exhaustive]
pub enum SystemTrayEvent {
  /// Tray context menu item was clicked.
  #[non_exhaustive]
  MenuItemClick {
    /// The id of the menu item.
    id: MenuId,
  },
  /// Tray icon received a left click.
  ///
  /// ## Platform-specific
  ///
  /// - **Linux:** Unsupported
  #[non_exhaustive]
  LeftClick {
    /// The position of the tray icon.
    position: PhysicalPosition<f64>,
    /// The size of the tray icon.
    size: PhysicalSize<f64>,
  },
  /// Tray icon received a right click.
  ///
  /// ## Platform-specific
  ///
  /// - **Linux:** Unsupported
  /// - **macOS:** `Ctrl` + `Left click` fire this event.
  #[non_exhaustive]
  RightClick {
    /// The position of the tray icon.
    position: PhysicalPosition<f64>,
    /// The size of the tray icon.
    size: PhysicalSize<f64>,
  },
  /// Fired when a menu item receive a `Double click`
  ///
  /// ## Platform-specific
  ///
  /// - **macOS / Linux:** Unsupported
  ///
  #[non_exhaustive]
  DoubleClick {
    /// The position of the tray icon.
    position: PhysicalPosition<f64>,
    /// The size of the tray icon.
    size: PhysicalSize<f64>,
  },
}

/// A handle to a system tray. Allows updating the context menu items.
#[default_runtime(crate::Wry, wry)]
#[derive(Debug)]
pub struct SystemTrayHandle<R: Runtime> {
  pub(crate) ids: Arc<Mutex<HashMap<MenuHash, MenuId>>>,
  pub(crate) inner: R::TrayHandler,
}

impl<R: Runtime> Clone for SystemTrayHandle<R> {
  fn clone(&self) -> Self {
    Self {
      ids: self.ids.clone(),
      inner: self.inner.clone(),
    }
  }
}

/// A handle to a system tray menu item.
#[default_runtime(crate::Wry, wry)]
#[derive(Debug)]
pub struct SystemTrayMenuItemHandle<R: Runtime> {
  id: MenuHash,
  tray_handler: R::TrayHandler,
}

impl<R: Runtime> Clone for SystemTrayMenuItemHandle<R> {
  fn clone(&self) -> Self {
    Self {
      id: self.id,
      tray_handler: self.tray_handler.clone(),
    }
  }
}

impl<R: Runtime> SystemTrayHandle<R> {
  /// Gets a handle to the menu item that has the specified `id`.
  pub fn get_item(&self, id: MenuIdRef<'_>) -> SystemTrayMenuItemHandle<R> {
    for (raw, item_id) in self.ids.lock().unwrap().iter() {
      if item_id == id {
        return SystemTrayMenuItemHandle {
          id: *raw,
          tray_handler: self.inner.clone(),
        };
      }
    }
    panic!("item id not found")
  }

  /// Updates the tray icon. Must be a [`TrayIcon::File`] on Linux and a [`TrayIcon::Raw`] on Windows and macOS.
  pub fn set_icon(&self, icon: TrayIcon) -> crate::Result<()> {
    self.inner.set_icon(icon).map_err(Into::into)
  }

  /// Updates the tray menu.
  pub fn set_menu(&self, menu: SystemTrayMenu) -> crate::Result<()> {
    let mut ids = HashMap::new();
    get_menu_ids(&mut ids, &menu);
    self.inner.set_menu(menu)?;
    *self.ids.lock().unwrap() = ids;
    Ok(())
  }

  /// Support [macOS tray icon template](https://developer.apple.com/documentation/appkit/nsimage/1520017-template?language=objc) to adjust automatically based on taskbar color.
  #[cfg(target_os = "macos")]
  pub fn set_icon_as_template(&self, is_template: bool) -> crate::Result<()> {
    self
      .inner
      .set_icon_as_template(is_template)
      .map_err(Into::into)
  }
}

impl<R: Runtime> SystemTrayMenuItemHandle<R> {
  /// Modifies the enabled state of the menu item.
  pub fn set_enabled(&self, enabled: bool) -> crate::Result<()> {
    self
      .tray_handler
      .update_item(self.id, MenuUpdate::SetEnabled(enabled))
      .map_err(Into::into)
  }

  /// Modifies the title (label) of the menu item.
  pub fn set_title<S: Into<String>>(&self, title: S) -> crate::Result<()> {
    self
      .tray_handler
      .update_item(self.id, MenuUpdate::SetTitle(title.into()))
      .map_err(Into::into)
  }

  /// Modifies the selected state of the menu item.
  pub fn set_selected(&self, selected: bool) -> crate::Result<()> {
    self
      .tray_handler
      .update_item(self.id, MenuUpdate::SetSelected(selected))
      .map_err(Into::into)
  }

  #[cfg(target_os = "macos")]
  #[cfg_attr(doc_cfg, doc(cfg(target_os = "macos")))]
  pub fn set_native_image(&self, image: crate::NativeImage) -> crate::Result<()> {
    self
      .tray_handler
      .update_item(self.id, MenuUpdate::SetNativeImage(image))
      .map_err(Into::into)
  }
}
