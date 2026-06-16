use glob::glob;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let build_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR always present in build scripts"));
    let c_dir = Path::new(build_dir).join("libpg_query");
    println!("cargo:rerun-if-changed=libpg_query");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=pg_query");

    bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/parsenodes.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_file(
            c_dir
                .join("src/postgres/include/nodes/primnodes.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_item("Node")
        .allowlist_item("MemoryContext")
        .allowlist_item("pg_query_init")
        .allowlist_item("AllocSetContextCreateInternal")
        .allowlist_item("ALLOCSET_DEFAULT_MINSIZE")
        .allowlist_item("ALLOCSET_DEFAULT_INITSIZE")
        .allowlist_item("ALLOCSET_DEFAULT_MAXSIZE")
        .allowlist_item("get_top_memory_context")
        .allowlist_item("MemoryContextSwitchTo")
        .allowlist_item("MemoryContextDelete")
        .allowlist_item("PgQueryError")
        .allowlist_item("pg_query_free_error")
        .allowlist_item("pg_query_raw_parse")
        .allowlist_item("PgQueryParseMode")
        .clang_arg(format!("-I{}", c_dir.display()))
        .clang_arg(format!(
            "-I{}",
            c_dir.join("src/postgres/include").display()
        ))
        .wrap_static_fns(true)
        .wrap_static_fns_path(out_dir.join("wrap_static_fns"))
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    let mut build = cc::Build::new();
    build
        .files(glob("libpg_query/src/*.c").unwrap().map(Result::unwrap))
        .files(
            glob("libpg_query/src/postgres/*.c")
                .unwrap()
                .map(Result::unwrap),
        )
        .file(out_dir.join("wrap_static_fns.c"))
        .include(&*c_dir)
        .include(c_dir.join("vendor"))
        .include(c_dir.join("src/postgres/include"))
        .include(c_dir.join("src/include"))
        .include(build_dir)
        .warnings(false)
        .compile("pg_query");
}
