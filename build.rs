use cmake::Config;

// CC_aarch64-unknown-none ...
//
// CC_aarch64_unknown_none=aarch64-linux-gnu-gcc
// cargo xbuild --target aarch64-unknown-none
// .env("CC_aarch64_unknown_none", "aarch64-linux-gnu-gcc")
fn main() {
    let dst = Config::new("cmake")
        .env("CC", "aarch64-linux-gnu-gcc")
        .no_build_target(true)
        //.define("H264DEC_TRACE", "1")
        //.define("_RANGE_CHECK", "1")
        //.define("_DEBUG_PRINT", "1")
        //.define("_ERROR_PRINT", "1")
        //.define("SINGLE_SLICE", "1")
        .cflag("-fno-stack-protector")
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=h264_decoder");
    println!("cargo:rerun-if-changed=cmake");
    println!("cargo:rerun-if-changed=bindgen");
    println!("cargo:rerun-if-changed=h264_decoder");
    println!("cargo:rerun-if-env-changed=CC");
}
