extern crate cbindgen;

use std::env;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
    .with_crate(crate_dir)
    .with_language(Language::C)
    .with_no_includes() // First we strip the default include lines which are not needed
    .with_header("#define FFI_LIB \"libmerge_pdf.so\" \n#define FFI_SCOPE \"MergePdf\"") // then we insert our custom definitions, the .so path is relative to the header file we're writing
    .generate()
    .expect("Unable to generate bindings")
    .write_to_file("target/debug/merge_pdf.h");

}


