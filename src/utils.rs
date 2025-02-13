//! Utility functions for interacting with the uiohook library.
//!
//! This module provides safe Rust wrappers around the low-level C functions
//! for retrieving system information and properties.

use crate::bindings;
use crate::error::{Result, UiohookError};
use std::slice;

/// Represents information about a screen.
#[derive(Debug, Clone, Copy)]
pub struct ScreenData {
    /// The screen number.
    pub number: u8,
    /// The x-coordinate of the screen.
    pub x: i16,
    /// The y-coordinate of the screen.
    pub y: i16,
    /// The width of the screen.
    pub width: u16,
    /// The height of the screen.
    pub height: u16,
}

impl From<bindings::screen_data> for ScreenData {
    fn from(data: bindings::screen_data) -> Self {
        ScreenData {
            number: data.number,
            x: data.x,
            y: data.y,
            width: data.width,
            height: data.height,
        }
    }
}

/// Retrieves information about all available screens.
///
/// # Returns
///
/// A `Result` containing a vector of `ScreenData` structs, each representing a screen.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```rust,no_run
/// use uiohook_rs::utils::create_screen_info;
///
/// match create_screen_info() {
///     Ok(screens) => {
///         for screen in screens {
///             println!("Screen {}: {}x{} at ({}, {})",
///                      screen.number, screen.width, screen.height, screen.x, screen.y);
///         }
///     },
///     Err(e) => eprintln!("Failed to get screen info: {}", e),
/// }
/// ```
pub fn create_screen_info() -> Result<Vec<ScreenData>> {
    unsafe {
        let mut count: u8 = 0;
        let ptr = bindings::hook_create_screen_info(&mut count);
        if ptr.is_null() {
            return Err(UiohookError::OutOfMemory);
        }
        let slice = slice::from_raw_parts(ptr, count as usize);
        let screens: Vec<ScreenData> = slice.iter().map(|&s| s.into()).collect();
        libc::free(ptr as *mut libc::c_void);
        Ok(screens)
    }
}

/// Retrieves the keyboard auto repeat rate.
///
/// # Returns
///
/// A `Result` containing the auto repeat rate as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_auto_repeat_rate;
///
/// match get_auto_repeat_rate() {
///     Ok(rate) => println!("Auto repeat rate: {} ms", rate),
///     Err(e) => eprintln!("Failed to get auto repeat rate: {}", e),
/// }
/// ```
pub fn get_auto_repeat_rate() -> Result<i64> {
    let rate = unsafe { bindings::hook_get_auto_repeat_rate() };
    if rate >= 0 {
        Ok(rate as i64)
    } else {
        Err(UiohookError::Unknown(rate as u32))
    }
}

/// Retrieves the keyboard auto repeat delay.
///
/// # Returns
///
/// A `Result` containing the auto repeat delay as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_auto_repeat_delay;
///
/// match get_auto_repeat_delay() {
///     Ok(delay) => println!("Auto repeat delay: {} ms", delay),
///     Err(e) => eprintln!("Failed to get auto repeat delay: {}", e),
/// }
/// ```
pub fn get_auto_repeat_delay() -> Result<i64> {
    let delay = unsafe { bindings::hook_get_auto_repeat_delay() };
    if delay >= 0 {
        Ok(delay as i64)
    } else {
        Err(UiohookError::Unknown(delay as u32))
    }
}

/// Retrieves the pointer acceleration multiplier.
///
/// # Returns
///
/// A `Result` containing the pointer acceleration multiplier as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_pointer_acceleration_multiplier;
///
/// match get_pointer_acceleration_multiplier() {
///     Ok(multiplier) => println!("Pointer acceleration multiplier: {}", multiplier),
///     Err(e) => eprintln!("Failed to get pointer acceleration multiplier: {}", e),
/// }
/// ```
pub fn get_pointer_acceleration_multiplier() -> Result<i64> {
    let multiplier = unsafe { bindings::hook_get_pointer_acceleration_multiplier() };
    if multiplier >= 0 {
        Ok(multiplier as i64)
    } else {
        Err(UiohookError::Unknown(multiplier as u32))
    }
}

/// Retrieves the pointer acceleration threshold.
///
/// # Returns
///
/// A `Result` containing the pointer acceleration threshold as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_pointer_acceleration_threshold;
///
/// match get_pointer_acceleration_threshold() {
///     Ok(threshold) => println!("Pointer acceleration threshold: {}", threshold),
///     Err(e) => eprintln!("Failed to get pointer acceleration threshold: {}", e),
/// }
/// ```
pub fn get_pointer_acceleration_threshold() -> Result<i64> {
    let threshold = unsafe { bindings::hook_get_pointer_acceleration_threshold() };
    if threshold >= 0 {
        Ok(threshold as i64)
    } else {
        Err(UiohookError::Unknown(threshold as u32))
    }
}

/// Retrieves the pointer sensitivity.
///
/// # Returns
///
/// A `Result` containing the pointer sensitivity as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_pointer_sensitivity;
///
/// match get_pointer_sensitivity() {
///     Ok(sensitivity) => println!("Pointer sensitivity: {}", sensitivity),
///     Err(e) => eprintln!("Failed to get pointer sensitivity: {}", e),
/// }
/// ```
pub fn get_pointer_sensitivity() -> Result<i64> {
    let sensitivity = unsafe { bindings::hook_get_pointer_sensitivity() };
    if sensitivity >= 0 {
        Ok(sensitivity as i64)
    } else {
        Err(UiohookError::Unknown(sensitivity as u32))
    }
}

/// Retrieves the multi-click time.
///
/// # Returns
///
/// A `Result` containing the multi-click time as an `i64`.
///
/// # Errors
///
/// Returns a `UiohookError` if the operation fails.
///
/// # Examples
///
/// ```
/// use uiohook_rs::utils::get_multi_click_time;
///
/// match get_multi_click_time() {
///     Ok(time) => println!("Multi-click time: {} ms", time),
///     Err(e) => eprintln!("Failed to get multi-click time: {}", e),
/// }
/// ```
pub fn get_multi_click_time() -> Result<i64> {
    let time = unsafe { bindings::hook_get_multi_click_time() };
    if time >= 0 {
        Ok(time as i64)
    } else {
        Err(UiohookError::Unknown(time as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_data_conversion() {
        let raw_data = bindings::screen_data {
            number: 1,
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        let screen_data: ScreenData = raw_data.into();
        assert_eq!(screen_data.number, 1);
        assert_eq!(screen_data.x, 0);
        assert_eq!(screen_data.y, 0);
        assert_eq!(screen_data.width, 1920);
        assert_eq!(screen_data.height, 1080);
    }

    // Note: The following tests are commented out because they interact with the system
    // and might not be suitable for automated testing environments.
    // Uncomment and modify as needed for local testing.

    /*
    #[test]
    fn test_create_screen_info() {
        let result = create_screen_info();
        assert!(result.is_ok());
        let screens = result.unwrap();
        assert!(!screens.is_empty());
    }

    #[test]
    fn test_get_auto_repeat_rate() {
        let result = get_auto_repeat_rate();
        assert!(result.is_ok());
        assert!(result.unwrap() >= 0);
    }

    #[test]
    fn test_get_auto_repeat_delay() {
        let result = get_auto_repeat_delay();
        assert!(result.is_ok());
        assert!(result.unwrap() >= 0);
    }

    #[test]
    fn test_get_pointer_acceleration_multiplier() {
        let result = get_pointer_acceleration_multiplier();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_pointer_acceleration_threshold() {
        let result = get_pointer_acceleration_threshold();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_pointer_sensitivity() {
        let result = get_pointer_sensitivity();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_multi_click_time() {
        let result = get_multi_click_time();
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }
    */
}
