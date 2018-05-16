fn main() {
    #[cfg(all(windows, target_pointer_width = "64"))]
    {
        println!(r"cargo:rustc-link-search=native\win-x64");
    }
}
