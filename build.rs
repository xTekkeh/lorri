//! build.rs is used to generate code at build time, which is then
//! imported elsewhere. This file is understood and executed by cargo.
use std::env;
use std::fs;
use std::path::Path;

/// Write version information to $OUT_DIR/build_rev.rs, which is included in src/lib.rs.
fn main() {
    println!("cargo:rerun-if-env-changed=BUILD_REV_COUNT");
    println!("cargo:rerun-if-env-changed=RUN_TIME_CLOSURE");
    println!("cargo:rerun-if-changed=build.rs");

    let rev_count = env::var("BUILD_REV_COUNT")
        .expect("BUILD_REV_COUNT not set, please reload nix-shell")
        .parse::<usize>()
        .expect("BUILD_REV_COUNT should be parseable as usize");

    fs::write(
        // cargo sets OUT_DIR: https://doc.rust-lang.org/cargo/reference/environment-variables.html
        Path::new(&env::var("OUT_DIR").unwrap()).join("build_rev.rs"),
        format!(
            r#"
/// lorri version in MAJOR.MINOR format.
pub const LORRI_VERSION: &str = "{major}.{minor}";

/// Number of revisions in the Git tree.
pub const VERSION_BUILD_REV: usize = {rev_count};

/// Run-time closure parameters. This argument points to a file
/// generated by ./nix/runtime.nix in Lorri's source.
pub const RUN_TIME_CLOSURE: &str = "{runtime_closure}";
"#,
            // cargo sets CARGO_PKG_VERSION_MAJOR and CARGO_PKG_VERSION_MINOR:
            // https://doc.rust-lang.org/cargo/reference/environment-variables.html
            major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap(),
            minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap(),
            rev_count = rev_count,
            runtime_closure = env::var("RUN_TIME_CLOSURE")
                .expect("RUN_TIME_CLOSURE not set, please reload nix-shell"),
        )
        .as_bytes(),
    )
    .unwrap();

    // Generate src/com_target_lorri.rs
    varlink_generator::cargo_build_tosource(
        "src/com.target.lorri.varlink",
        /* rustfmt */ true,
    );
}
