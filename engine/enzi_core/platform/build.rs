fn main() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=X11-xcb");
}
