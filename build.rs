extern crate cc;

fn main()   {
    cc::Build::new()
        .cpp(true)
        .file("src/llvm_wrapper.cpp")
        .include("src")
        .include("/usr/lib/llvm/12/include")
        .flag("-std=c++14")
        .flag("-D_GNU_SOURCE")
        .flag("-D__STDC_CONSTANT_MACROS")
        .flag("-D__STDC_FORMAT_MACROS")
        .flag("-D__STDC_LIMIT_MACROS")
        .flag("-L/usr/lib/llvm/12/lib64")
        .flag("-lLLVM-12")
        .shared_flag(true)
        .pic(true)
        .compile("libwrapper.so");
}
