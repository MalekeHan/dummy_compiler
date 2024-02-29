fn main() {
    //  link with libstdc++
    println!("cargo:rustc-link-lib=stdc++");

    // specify the search path for libstdc++.so
    println!("cargo:rustc-link-search=native=/usr/lib64");

    // build script is re-executed only if it changes
    println!("cargo:rerun-if-changed=build.rs");
}
