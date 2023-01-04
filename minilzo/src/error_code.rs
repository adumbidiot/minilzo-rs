use std::os::raw::c_int;

/// A library error code
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ErrorCode(pub c_int);

impl ErrorCode {
    /// No error
    pub const OK: Self = ErrorCode(minilzo_sys::LZO_E_OK as c_int);
    /// Some error occured
    pub const ERROR: Self = ErrorCode(minilzo_sys::LZO_E_ERROR as c_int);
    /// Out of memory
    pub const OUT_OF_MEMORY: Self = ErrorCode(minilzo_sys::LZO_E_OUT_OF_MEMORY as c_int);
    /// Not compressible
    pub const NOT_COMPRESSIBLE: Self = ErrorCode(minilzo_sys::LZO_E_NOT_COMPRESSIBLE as c_int);
    /// Input overrun
    pub const INPUT_OVERRUN: Self = ErrorCode(minilzo_sys::LZO_E_INPUT_OVERRUN as c_int);
    /// Output overrun
    pub const OUTPUT_OVERRUN: Self = ErrorCode(minilzo_sys::LZO_E_OUTPUT_OVERRUN as c_int);
    /// Look behind overrun
    pub const LOOKBEHIND_OVERRUN: Self = ErrorCode(minilzo_sys::LZO_E_LOOKBEHIND_OVERRUN as c_int);
    /// EOF not found
    pub const EOF_NOT_FOUND: Self = ErrorCode(minilzo_sys::LZO_E_EOF_NOT_FOUND as c_int);
    /// Input not consumed
    pub const INPUT_NOT_CONSUMED: Self = ErrorCode(minilzo_sys::LZO_E_INPUT_NOT_CONSUMED as c_int);
    /// Not implemented
    pub const NOT_YET_IMPLEMENTED: Self =
        ErrorCode(minilzo_sys::LZO_E_NOT_YET_IMPLEMENTED as c_int);
    /// Invalid argument
    pub const INVALID_ARGUMENT: Self = ErrorCode(minilzo_sys::LZO_E_INVALID_ARGUMENT as c_int);
    /// Invalid pointer argument
    pub const INVALID_ALIGNMENT: Self = ErrorCode(minilzo_sys::LZO_E_INVALID_ALIGNMENT as c_int);
    /// Output not consumed
    pub const OUTPUT_NOT_CONSUMED: Self =
        ErrorCode(minilzo_sys::LZO_E_OUTPUT_NOT_CONSUMED as c_int);
    /// Internal error
    pub const INTERNAL_ERROR: Self = ErrorCode(minilzo_sys::LZO_E_INTERNAL_ERROR as c_int);

    /// Returns true if this is OK
    pub fn is_ok(self) -> bool {
        self == Self::OK
    }

    /// Get a human-readable description of the error
    pub fn reason(self) -> Option<&'static str> {
        match self {
            Self::OK => Some("no error"),
            Self::ERROR => Some("error"),
            Self::OUT_OF_MEMORY => Some("out of memory"),
            Self::NOT_COMPRESSIBLE => Some("not compressible"),
            Self::INPUT_OVERRUN => Some("input overrun"),
            Self::OUTPUT_OVERRUN => Some("output overrun"),
            Self::LOOKBEHIND_OVERRUN => Some("lookbehind overrun"),
            Self::EOF_NOT_FOUND => Some("eof not found"),
            Self::INPUT_NOT_CONSUMED => Some("input not consumed"),
            Self::NOT_YET_IMPLEMENTED => Some("not yet implemented"),
            Self::INVALID_ARGUMENT => Some("invalid argument"),
            Self::INVALID_ALIGNMENT => Some("invalid alignment"),
            Self::OUTPUT_NOT_CONSUMED => Some("output not consumed"),
            Self::INTERNAL_ERROR => Some("internal error"),
            _ => None,
        }
    }

    /// Transform this error into a result, mapping OK to Ok(()) and everything else to Err(self).
    pub fn into_result(self) -> Result<(), Self> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (code {:X})",
            self.reason().unwrap_or("unknown error"),
            self.0
        )
    }
}

impl std::error::Error for ErrorCode {}
