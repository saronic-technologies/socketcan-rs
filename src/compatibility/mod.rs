// socketcan/src/compatibility.rs
//
// Handles SocketCAN compatibility between native Linux and
// "emulated" OSX to make sure cargo check passes for both OS'es
//
// This file is part of the Rust 'socketcan-rs' library.
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.

//! Handles SocketCAN compatibility between native Linux and
//! "emulated" OSX to make sure cargo check passes for both OS'es

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod osx;

#[cfg(target_os = "macos")]
pub use osx::*;
