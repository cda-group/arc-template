use arc_script_build::Builder;

fn main() {
    // This pre-builds any file in the crate whose filename is `main.arc`.
    Builder::default().build();
    println!("cargo:rerun-if-env-changed=ARCSCRIPT_MLIR_BACKEND");
}
