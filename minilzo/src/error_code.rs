use std::os::raw::c_int;

/// A library error code
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ErrorCode(pub c_int);

impl ErrorCode {
    /// No error
    const OK: Self = ErrorCode(minilzo_sys::LZO_E_OK as c_int);

    /// Returns true if this is OK
    pub fn is_ok(self) -> bool {
        self == Self::OK
    }
}
