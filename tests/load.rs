use ra_ap_hir::Semantics;
use ra_ap_load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use ra_ap_paths::{AbsPathBuf, Utf8PathBuf};
use ra_ap_project_model::CargoConfig;
use ra_ap_vfs::VfsPath;
use std::path::{PathBuf, Path};

fn lib_dir() -> PathBuf { concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/lib").into() }

fn load(lib_dir: &Path) {
    println!("loading {}", lib_dir.display());
    let manifest = lib_dir.join("Cargo.toml");
    let cargo_config = CargoConfig::default();
    let load_cargo_config = LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
    };
    let progress = |s| println!("progress: {}", s);
    let (db, vfs, _) = load_workspace_at(&manifest, &cargo_config, &load_cargo_config, &progress)
        .expect("load_workspace_at");
    let lib_path = Utf8PathBuf::from_path_buf(lib_dir.join("src/lib.rs"))
        .ok()
        .and_then(|p| AbsPathBuf::try_from(p).ok())
        .expect("lib_path");
    let lib_file_id = vfs.file_id(&VfsPath::from(lib_path)).expect("file_id");
    let sema = Semantics::new(&db);
    let module = sema
        .file_to_module_def(lib_file_id)
        .expect("file_to_module_def");
    _ = module;
}

#[test]
fn load_non_canonicalized() {
	load(&lib_dir());
}

#[test]
fn load_canonicalized() {
	load(&lib_dir().canonicalize().expect("canonicalize"));
}

#[test]
fn load_dunce_canoninicalized() {
	load(&dunce::canonicalize(&lib_dir()).expect("dunce"));
}

#[test]
fn load_double_canoninicalized() {
	load(&dunce::canonicalize(&lib_dir().canonicalize().expect("canonicalize")).expect("dunce"));
}


