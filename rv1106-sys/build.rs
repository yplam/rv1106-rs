use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let work_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let work_path = work_dir.to_str().unwrap();
    println!("cargo:rustc-link-search={}/lib", work_path);
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());

    let mut cc_builder = cc::Build::new();
    cc_builder.cpp(true)
        .flag("-std=c++11")
        .file("src/rockit.cpp")
        .flag("-march=armv7")
        .flag(&format!("-I{}", work_path))
        .flag(&format!("-I{}/include", work_path));

    cc_builder.compile("librv1106rs.a");


    let cc_compiler = cc_builder.get_compiler();
    let cc_sysroot = PathBuf::from(
        String::from_utf8(
            cc_compiler
                .to_command()
                .arg("-print-sysroot")
                .output()
                .unwrap()
                .stdout,
        )
            .unwrap()
            .trim(),
    )
        .canonicalize()
        .unwrap();

    let toolchain_dir = cc_sysroot.parent().unwrap().parent().unwrap();

    let toolchain_gcc_include_dir = toolchain_dir
        .join("lib/gcc/arm-rockchip830-linux-uclibcgnueabihf/8.3.0/include")
        .canonicalize()
        .unwrap();

    // let toolchain_cpp_include_dir = toolchain_dir
    //     .join("arm-rockchip830-linux-uclibcgnueabihf/include/c++/8.3.0")
    //     .canonicalize()
    //     .unwrap();

    println!("cargo:rustc-link-lib=static=rv1106rs");
    println!("cargo:rustc-link-lib=static=rockit");
    println!("cargo:rustc-link-lib=static=rockchip_mpp");
    println!("cargo:rustc-link-lib=static=rga");
    // println!("cargo:rustc-link-lib=static=rkaiq");
    // println!("cargo:rustc-link-lib=static=smartIr");

    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=atomic");

    println!("cargo:rerun-if-changed=rockit.hpp");

    let builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}/usr/include", cc_sysroot.to_str().unwrap()))
        .clang_arg(format!("-I{}", toolchain_gcc_include_dir.to_str().unwrap()))
        .clang_arg(format!("-I{}/include", work_path))
        .clang_arg(format!("-I{}/include/rockit", work_path))
        .clang_arg(format!("-I{}/include/mpp", work_path))
        .clang_arg("-std=c++11")
        .clang_arg("-mfloat-abi=hard")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-DRGA_CURRENT_API_HEADER_VERSION=RGA2-Enhance")
        .clang_arg("-march=armv7")
        .clang_arg("-fPIC")
        .derive_default(true);
    let bindings = builder
        .header("rockit.hpp")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("rockit_bindings.rs"))
        .expect("Couldn't write bindings!");
}
