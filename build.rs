fn main() {
    #[cfg(target_os = "windows")]
    {}
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("c_src/platform_macos.m")
            .flag("-x")
            .flag("objective-c")
            .compile("platform");

        println!("cargo:rustc-link-lib=framework=AppKit");
    }
    #[cfg(target_os = "linux")]
    {}
}
