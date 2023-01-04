mod context;
mod error_code;

pub use self::context::Context;
pub use self::error_code::ErrorCode;
use std::mem::MaybeUninit;

/// # Safety
///
/// * The `as_ptr` function MUST return a valid, mutable pointer to potentially uninitialized memory.
/// * The `len` function MUST return the length of the section of uninitialized memory returned by `as_ptr`, in bytes.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait OutputBuffer {
    /// Get the ptr.
    fn get_ptr(&mut self) -> *mut MaybeUninit<u8>;

    /// Get the length of the buffer.
    fn get_size(&self) -> u64;
}

unsafe impl OutputBuffer for &mut [u8] {
    fn get_ptr(&mut self) -> *mut MaybeUninit<u8> {
        <[u8]>::as_mut_ptr(self).cast()
    }

    fn get_size(&self) -> u64 {
        <[u8]>::len(self).try_into().unwrap()
    }
}

unsafe impl OutputBuffer for &mut [MaybeUninit<u8>] {
    fn get_ptr(&mut self) -> *mut MaybeUninit<u8> {
        <[MaybeUninit<u8>]>::as_mut_ptr(self).cast()
    }

    fn get_size(&self) -> u64 {
        <[MaybeUninit<u8>]>::len(self).try_into().unwrap()
    }
}

unsafe impl OutputBuffer for &mut Vec<u8> {
    fn get_ptr(&mut self) -> *mut MaybeUninit<u8> {
        <Vec<u8>>::as_mut_ptr(self).cast()
    }

    fn get_size(&self) -> u64 {
        <Vec<u8>>::len(self).try_into().unwrap()
    }
}

unsafe impl OutputBuffer for &mut Vec<MaybeUninit<u8>> {
    fn get_ptr(&mut self) -> *mut MaybeUninit<u8> {
        <Vec<MaybeUninit<u8>>>::as_mut_ptr(self).cast()
    }

    fn get_size(&self) -> u64 {
        <Vec<MaybeUninit<u8>>>::len(self).try_into().unwrap()
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
