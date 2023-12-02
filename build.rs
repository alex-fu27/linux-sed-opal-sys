use bindgen::{builder, EnumVariation};
use std::env;
use std::path::PathBuf;

fn main() {
    let build = builder()
        .header("wrapper.h")
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: true,
            is_global: false,
        })
        .generate_comments(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    build
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

