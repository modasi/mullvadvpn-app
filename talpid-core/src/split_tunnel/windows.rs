use self::winexclude::*;
use crate::logging::windows::log_sink;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

const LOGGING_CONTEXT: &[u8] = b"WinExclude\0";

/*
TODO:

WinExclude_SetAppPaths:

Success => "Updated exclusions paths",
NotFound => "One or more paths were not found",
InvalidArgument => "Invalid argument",
*/

/// Errors that may occur in [`SplitTunnel`].
#[derive(err_derive::Error, Debug)]
pub enum Error {
    /// The Windows DLL or kernel-mode driver returned an error.
    #[error(display = "Failed to initialize split tunneling")]
    InitializationFailed,

    /// Failed to update the paths to exclude from the tunnel.
    #[error(display = "Failed to update exclusions: {}", _0)]
    UpdatePaths(winexclude::WinExcludeUpdateStatus),
}

/// Manages applications whose traffic to exclude from the tunnel.
pub struct SplitTunnel(());

impl SplitTunnel {
    /// Initialize the driver.
    pub fn new() -> Result<Self, Error> {
        // TODO: set up the driver

        // TODO: connect
        // TODO: initialize
        // TODO: register processes
        // TODO: register IPs

        // TODO: set paths? empty list?
        // TODO: think about
        Ok(SplitTunnel(()))
    }

    /// Set a list of applications to exclude from the tunnel.
    pub fn set_paths<T: AsRef<OsStr>>(&mut self, paths: &[T]) -> Result<(), Error> {
        let mut u16_paths = Vec::<Vec<u16>>::new();

        for path in paths {
            let mut u16_path: Vec<u16> = path.as_ref().encode_wide().collect();
            u16_path.push(0u16);
            u16_paths.push(u16_path);
        }

        let mut u16_paths_ptrs: Vec<_> = u16_paths.iter().map(|path| path.as_ptr()).collect();
        u16_paths_ptrs.push(std::ptr::null());

        // TODO: We should reject non-absolute paths
        // TODO: The paths must be appropriately converted to physical paths
        //  TODO: take code from test project

        // TODO: Set app paths here
    }
}

impl Drop for SplitTunnel {
    fn drop(&mut self) {
        // TODO: Deinitialize driver here
    }
}
