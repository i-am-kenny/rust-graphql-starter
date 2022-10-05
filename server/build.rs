use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let schema = schema::build_schema().finish();

    fs::write(out_dir.join("schema.graphql"), schema.sdl()).expect("failed writing schema to file");
}
