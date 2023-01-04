mod context;
mod error_code;

pub use self::context::Context;
pub use self::error_code::ErrorCode;

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
