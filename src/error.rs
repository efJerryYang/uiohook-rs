use thiserror::Error;
use std::result;

/// A specialized Result type for uiohook operations.
pub type Result<T> = result::Result<T, UiohookError>;

/// Represents all possible errors returned by the uiohook library.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiohookError {
    /// The operation failed.
    #[error("Operation failed")]
    Failure,

    /// Out of memory error.
    #[error("Out of memory")]
    OutOfMemory,

    /// X11 failed to open display.
    #[error("X11 failed to open display")]
    XOpenDisplay,

    /// X11 failed to find the XRecord extension.
    #[error("X11 failed to find the XRecord extension")]
    XRecordNotFound,

    /// X11 failed to allocate XRecord range.
    #[error("X11 failed to allocate XRecord range")]
    XRecordAllocRange,

    /// X11 failed to create XRecord context.
    #[error("X11 failed to create XRecord context")]
    XRecordCreateContext,

    /// X11 failed to enable XRecord context.
    #[error("X11 failed to enable XRecord context")]
    XRecordEnableContext,

    /// X11 failed to get XRecord context.
    #[error("X11 failed to get XRecord context")]
    XRecordGetContext,

    /// Windows failed to set hook.
    #[error("Windows failed to set hook")]
    SetWindowsHookEx,

    /// Windows failed to get module handle.
    #[error("Windows failed to get module handle")]
    GetModuleHandle,

    /// macOS failed to create event port.
    #[error("macOS failed to create event port")]
    CreateEventPort,

    /// macOS failed to create run loop source.
    #[error("macOS failed to create run loop source")]
    CreateRunLoopSource,

    /// macOS failed to get run loop.
    #[error("macOS failed to get run loop")]
    GetRunLoop,

    /// macOS failed to create observer.
    #[error("macOS failed to create observer")]
    CreateObserver,

    /// The hook is already running.
    #[error("The hook is already running")]
    AlreadyRunning,

    /// The hook is not running.
    #[error("The hook is not running")]
    NotRunning,

    /// The hook is not initialized.
    #[error("The hook is not initialized")]
    NotInitialized,

    /// The specified mouse button is not recognized.
    #[error("Unknown mouse button: {0}")]
    UnknownMouseButton(u32),

    /// An unknown error occurred.
    #[error("Unknown error: {0}")]
    Unknown(u32),
}

impl From<u32> for UiohookError {
    fn from(error_code: u32) -> Self {
        use crate::bindings::*;
        match error_code {
            UIOHOOK_FAILURE => UiohookError::Failure,
            UIOHOOK_ERROR_OUT_OF_MEMORY => UiohookError::OutOfMemory,
            UIOHOOK_ERROR_X_OPEN_DISPLAY => UiohookError::XOpenDisplay,
            UIOHOOK_ERROR_X_RECORD_NOT_FOUND => UiohookError::XRecordNotFound,
            UIOHOOK_ERROR_X_RECORD_ALLOC_RANGE => UiohookError::XRecordAllocRange,
            UIOHOOK_ERROR_X_RECORD_CREATE_CONTEXT => UiohookError::XRecordCreateContext,
            UIOHOOK_ERROR_X_RECORD_ENABLE_CONTEXT => UiohookError::XRecordEnableContext,
            UIOHOOK_ERROR_X_RECORD_GET_CONTEXT => UiohookError::XRecordGetContext,
            UIOHOOK_ERROR_SET_WINDOWS_HOOK_EX => UiohookError::SetWindowsHookEx,
            UIOHOOK_ERROR_GET_MODULE_HANDLE => UiohookError::GetModuleHandle,
            UIOHOOK_ERROR_CREATE_EVENT_PORT => UiohookError::CreateEventPort,
            UIOHOOK_ERROR_CREATE_RUN_LOOP_SOURCE => UiohookError::CreateRunLoopSource,
            UIOHOOK_ERROR_GET_RUNLOOP => UiohookError::GetRunLoop,
            UIOHOOK_ERROR_CREATE_OBSERVER => UiohookError::CreateObserver,
            _ => UiohookError::Unknown(error_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::*;

    #[test]
    fn test_error_conversion() {
        assert!(matches!(UiohookError::from(UIOHOOK_FAILURE), UiohookError::Failure));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_OUT_OF_MEMORY), UiohookError::OutOfMemory));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_OPEN_DISPLAY), UiohookError::XOpenDisplay));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_RECORD_NOT_FOUND), UiohookError::XRecordNotFound));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_RECORD_ALLOC_RANGE), UiohookError::XRecordAllocRange));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_RECORD_CREATE_CONTEXT), UiohookError::XRecordCreateContext));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_RECORD_ENABLE_CONTEXT), UiohookError::XRecordEnableContext));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_X_RECORD_GET_CONTEXT), UiohookError::XRecordGetContext));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_SET_WINDOWS_HOOK_EX), UiohookError::SetWindowsHookEx));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_GET_MODULE_HANDLE), UiohookError::GetModuleHandle));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_CREATE_EVENT_PORT), UiohookError::CreateEventPort));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_CREATE_RUN_LOOP_SOURCE), UiohookError::CreateRunLoopSource));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_GET_RUNLOOP), UiohookError::GetRunLoop));
        assert!(matches!(UiohookError::from(UIOHOOK_ERROR_CREATE_OBSERVER), UiohookError::CreateObserver));
        assert!(matches!(UiohookError::from(999), UiohookError::Unknown(999)));
    }

    #[test]
    fn test_error_display() {
        assert_eq!(UiohookError::Failure.to_string(), "Operation failed");
        assert_eq!(UiohookError::OutOfMemory.to_string(), "Out of memory");
        assert_eq!(UiohookError::XOpenDisplay.to_string(), "X11 failed to open display");
        assert_eq!(UiohookError::Unknown(999).to_string(), "Unknown error: 999");
    }
}