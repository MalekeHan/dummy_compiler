fn main() {
    // Link against libstdc++
    println!("cargo:rustc-link-lib=stdc++");

    // Specify the path to libstdc++
    println!("cargo:rustc-link-search=native=/usr/lib64");
}
