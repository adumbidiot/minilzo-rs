use crate::ErrorCode;
use minilzo_sys::lzo_align_t;
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

static INIT_ERROR_CODE: Lazy<ErrorCode> =
    Lazy::new(|| ErrorCode(unsafe { minilzo_sys::lzo_init_func() }));

/// A compression and decompression context.
///
/// Serves as proof of library initialization.
/// This may be copied and cloned freely and cheaply; this is a zero-sized struct.
///
/// Note that creating a context incurs some cost,
/// as if it is the first context the library initialization routines are run.
/// Additionally,
/// there is synchronization performed to ensure that the initialization routine is run only once.
#[derive(Debug, Copy, Clone)]
pub struct Context {
    _data: PhantomData<bool>,
}

impl Context {
    /// Make a new [`Context`].
    pub fn new() -> Result<Self, ErrorCode> {
        let error = *INIT_ERROR_CODE;
        if !error.is_ok() {
            return Err(error);
        }

        Ok(Self { _data: PhantomData })
    }

    /// Get the version
    pub fn version(&self) -> u32 {
        unsafe { minilzo_sys::lzo_version() }
    }

    /// Get the version string
    pub fn version_string(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(minilzo_sys::lzo_version_string().cast()) }
    }

    /// Get the version date
    pub fn version_date(&self) -> &'static CStr {
        unsafe { CStr::from_ptr(minilzo_sys::lzo_version_date().cast()) }
    }

    /// Compress
    pub fn compress(
        &self,
        input: &[u8],
        output: &mut [MaybeUninit<u8>],
    ) -> Result<usize, ErrorCode> {
        const WORKSPACE_LEN_BYTES: usize = minilzo_sys::LZO1X_1_MEM_COMPRESS_ as usize;
        const WORKSPACE_LEN: usize = (WORKSPACE_LEN_BYTES + (std::mem::size_of::<lzo_align_t>())
            - 1)
            / std::mem::size_of::<lzo_align_t>();

        let input_len = input.len().try_into().unwrap();
        let mut output_len = output.len().try_into().unwrap();
        let mut workspace = [MaybeUninit::<lzo_align_t>::uninit(); WORKSPACE_LEN];
        let error = unsafe {
            minilzo_sys::lzo1x_1_compress(
                input.as_ptr(),
                input_len,
                output.as_mut_ptr().cast(),
                &mut output_len,
                workspace.as_mut_ptr().cast(),
            )
        };

        let error = ErrorCode(error);
        if !error.is_ok() {
            return Err(error);
        }

        Ok(output_len.try_into().unwrap())
    }

    /// Decompress
    pub fn decompress(
        &self,
        input: &[u8],
        output: &mut [MaybeUninit<u8>],
    ) -> Result<usize, ErrorCode> {
        let input_len = input.len().try_into().unwrap();
        let mut output_len = output.len().try_into().unwrap();

        let error = unsafe {
            minilzo_sys::lzo1x_decompress_safe(
                input.as_ptr(),
                input_len,
                output.as_mut_ptr().cast(),
                &mut output_len,
                std::ptr::null_mut(),
            )
        };

        let error = ErrorCode(error);
        if !error.is_ok() {
            return Err(error);
        }

        Ok(output_len.try_into().unwrap())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new().expect("failed to init minilzo")
    }
}
