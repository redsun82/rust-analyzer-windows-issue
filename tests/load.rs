use std::path::PathBuf;
use ra_ap_hir::Semantics;
use ra_ap_load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use ra_ap_paths::AbsPathBuf;
use ra_ap_project_model::CargoConfig;
use ra_ap_vfs::VfsPath;

#[test]
fn load() {
    let data_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data"));
    let manifest = data_dir.join("lib/Cargo.toml");
    let cargo_config = CargoConfig{

        ..Default::default()
    };
    let load_cargo_config = LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro_server: ProcMacroServerChoice::Sysroot,
        prefill_caches: false,
    };
    let progress = |s| println!("{}", s);
    let (db, vfs, _) = load_workspace_at(&manifest, &cargo_config, &load_cargo_config, &progress).expect("load_workspace_at");
    let lib_path = AbsPathBuf::try_from(data_dir.join("lib/src/lib.rs").to_string_lossy().as_ref()).expect("lib_path");
    let lib_file_id = vfs.file_id(&VfsPath::from(lib_path)).expect("file_id");
    let sema = Semantics::new(&db);
    let module = sema.file_to_module_def(lib_file_id).expect("file_to_module_def");
    _ = module;
}
