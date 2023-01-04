use std::path::PathBuf;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("missing `OUT_DIR`");
    let out_dir = PathBuf::from(out_dir);

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_file("lzoconf.h")
        .allowlist_file("lzodefs.h")
        .allowlist_function("lzo1x_.*")
        .allowlist_function("lzo_.*")
        .allowlist_type("lzo_.*")
        .allowlist_var("LZO1X.*")
        .allowlist_var("MINILZO_VERSION")
        .allowlist_var("LZO_E_.*")
        .clang_arg("-I./minilzo-2.10")
        .generate()
        .expect("failed to generate bindings");
    bindings.emit_warnings();
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings to file");
    println!("cargo:rerun-if-changed=wrapper.h");

    cc::Build::new()
        .include("minilzo-2.10")
        .file("minilzo-2.10/minilzo.c")
        .file("minilzo-rust.c")
        .compile("minilzo")
}
