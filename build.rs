use cmake::Config;

// CC_aarch64-unknown-none ...
//
// CC_aarch64_unknown_none=aarch64-linux-gnu-gcc
// cargo xbuild --target aarch64-unknown-none
fn main() {
    let dst = Config::new("cmake").no_build_target(true).build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=h264_decoder");
    println!("cargo:rerun-if-changed=cmake");
    println!("cargo:rerun-if-changed=bindgen");
    println!("cargo:rerun-if-changed=h264_decoder");
    println!("cargo:rerun-if-env-changed=CC");
}
