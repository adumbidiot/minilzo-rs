pub use minilzo_sys::lzo_align_t;
use once_cell::sync::Lazy;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

static INIT_ERROR_CODE: Lazy<ErrorCode> =
    Lazy::new(|| ErrorCode(unsafe { minilzo_sys::lzo_init_func() }));

/// A library error code
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ErrorCode(pub c_int);

impl ErrorCode {
    /// No error
    const OK: Self = ErrorCode(minilzo_sys::LZO_E_OK as c_int);

    /// Returns true if this is OK
    pub fn is_ok(self) -> bool {
        self == Self::OK
    }
}

#[derive(Debug, Clone)]
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
            minilzo_sys::lzo1x_decompress(
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testmini_c_port() {
        let context = Context::new().expect("init failed");
        dbg!(context.version());
        dbg!(context.version_string());
        dbg!(context.version_date());

        let input: Vec<u8> = vec![0; 128 * 1024];
        let compress_output_len = input.len() + input.len() / 16 + 64 + 3;
        let mut compress_output = vec![MaybeUninit::uninit(); compress_output_len];
        let compressed_len = context
            .compress(&input, &mut compress_output)
            .expect("failed to compress");
        let compressed: &[u8] =
            unsafe { std::slice::from_raw_parts(compress_output.as_ptr().cast(), compressed_len) };

        let mut decompressed = vec![std::mem::MaybeUninit::uninit(); 128 * 1024];
        let decompressed_len = context
            .decompress(compressed, &mut decompressed)
            .expect("failed to decompress");
        let decompressed: &[u8] =
            unsafe { std::slice::from_raw_parts(decompressed.as_ptr().cast(), decompressed_len) };
        assert!(decompressed == input);
    }
}
