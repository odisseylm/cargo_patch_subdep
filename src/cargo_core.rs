use std::path::Path;
//--------------------------------------------------------------------------------------------------


//
/// All cargo package related logic is put here to abstract over it changes easy.
//


#[cfg(any(
    cargo_core_ver_prefix = "05x",
    cargo_core_ver_prefix = "06x",
    cargo_core_ver_prefix = "07x",
    cargo_core_ver_prefix = "076x",
))]
type CargoCoreConfig = cargo::util::config::Config;


#[cfg(any(
    cargo_core_ver_prefix = "079x",
    cargo_core_ver_prefix = "08x",
    cargo_core_ver_prefix = "09x",
    cargo_core_ver_prefix = "010x",
    cargo_core_ver_prefix = "011x",
    cargo_core_ver_prefix = "012x",
    cargo_core_ver_prefix = "013x",
    cargo_core_ver_prefix = "014x",
    cargo_core_ver_prefix = "015x",
    cargo_core_ver_prefix = "1_0x",
    cargo_core_ver_prefix = "1_1x",
    cargo_core_ver_prefix = "1_2x",
    cargo_core_ver_prefix = "1_3x",
    cargo_core_ver_prefix = "1_4x",
    cargo_core_ver_prefix = "1_5x",
))]
type CargoCoreConfig = cargo::util::context::GlobalContext;



#[cfg(any(
    cargo_core_ver_prefix = "076x",
    cargo_core_ver_prefix = "079x",
    cargo_core_ver_prefix = "08x",
    cargo_core_ver_prefix = "09x",
    cargo_core_ver_prefix = "010x",
    cargo_core_ver_prefix = "011x",
    cargo_core_ver_prefix = "012x",
    cargo_core_ver_prefix = "013x",
    cargo_core_ver_prefix = "014x",
    cargo_core_ver_prefix = "015x",
    cargo_core_ver_prefix = "1_0x",
    cargo_core_ver_prefix = "1_1x",
    cargo_core_ver_prefix = "1_2x",
    cargo_core_ver_prefix = "1_3x",
    cargo_core_ver_prefix = "1_4x",
    cargo_core_ver_prefix = "1_5x",
))]
pub fn acquire_cargo_core_package_cache_lock(config: &CargoCoreConfig)
    -> anyhow::Result<cargo::util::cache_lock::CacheLock> {
    let lock = config.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?;
    Ok(lock)
}


#[cfg(any(
    cargo_core_ver_prefix = "05x",
    cargo_core_ver_prefix = "06x",
    cargo_core_ver_prefix = "07x",
))]
pub fn acquire_cargo_core_package_cache_lock(config: &CargoCoreConfig)
    -> anyhow::Result<cargo::util::config::PackageCacheLock> {
    let lock = config.acquire_package_cache_lock() ?;
    Ok(lock)
}


pub fn setup_cargo_core_config() -> Result<CargoCoreConfig, anyhow::Error> {
    let config = CargoCoreConfig::default()?;
    config.shell().set_verbosity(cargo::core::Verbosity::Quiet);
    Ok(config)
}


pub fn fetch_cargo_core_workspace<'a>(config: &'a CargoCoreConfig, path: &Path)
                                      -> Result<cargo::core::Workspace<'a>, anyhow::Error> {
    cargo::core::Workspace::new(path, config)
}


/*
pub struct CargoCoreWorkspace<'a> {
    context: cargo::util::context::GlobalContext,
    cache_lock: cargo::util::cache_lock::CacheLock<'a>,
    pub workspace: cargo::core::Workspace<'a>,
}

pub fn fetch_cargo_core_workspace_from_manifest_path<'a>(manifest_path: &Path)
    -> Result<CargoCoreWorkspace<'a>, anyhow::Error> {

    let context = cargo::util::context::GlobalContext::default()?;
    context.shell().set_verbosity(cargo::core::Verbosity::Quiet);

    let cache_lock: cargo::util::cache_lock::CacheLock = context.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?;
    let workspace = cargo::core::Workspace::new(manifest_path, &context) ?;
    Ok(CargoCoreWorkspace::<'a> { context, cache_lock, workspace })
}
*/

/*
pub struct CargoCoreWorkspace<'a> {
    context: cargo::util::context::GlobalContext,
    cache_lock: Option<cargo::util::cache_lock::CacheLock<'a>>,
    workspace: Option<cargo::core::Workspace<'a>>,
    pub packages: Option<cargo::core::PackageSet<'a>>,
    // _pin: core::marker::PhantomPinned,
}

// impl <'a> CargoCoreWorkspace<'a> {
//     fn init(&'a mut self, manifest_path: &Path) -> anyhow::Result<()> {
impl CargoCoreWorkspace<'_> {
    fn init(&mut self, manifest_path: &Path) -> anyhow::Result<()> {

        self.context.shell().set_verbosity(cargo::core::Verbosity::Quiet);

        let cache_lock = self.context.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?;
        let workspace: cargo::core::Workspace = cargo::core::Workspace::new(manifest_path, &self.context) ?;
        let (pkg_set, _resolve): (cargo::core::PackageSet, cargo::core::Resolve) = cargo::ops::resolve_ws(&workspace) ?;

        self.cache_lock = Some(cache_lock);
        self.workspace = Some(workspace);
        self.packages = Some(pkg_set);

        Ok(())
    }
}

pub fn resolve_cargo_core_sub_packages(manifest_path: &Path)
    -> Result<Box<CargoCoreWorkspace>, anyhow::Error> {

    // let context = cargo::util::context::GlobalContext::default() ?;
    // context.shell().set_verbosity(cargo::core::Verbosity::Quiet);
    // let context_ref: &'a cargo::util::context::GlobalContext = &context;

    // let mut res: Box<CargoCoreWorkspace<'a>> = Box::new(CargoCoreWorkspace::<'a> {
    //     context: cargo::util::context::GlobalContext::default() ?,
    //     cache_lock: None,
    //     workspace: None,
    //     packages: None,
    //     _pin: core::marker::PhantomPinned,
    // });
    let mut res: CargoCoreWorkspace = CargoCoreWorkspace {
    // let mut res: CargoCoreWorkspace = CargoCoreWorkspace {
        context: cargo::util::context::GlobalContext::default() ?,
        cache_lock: None,
        workspace: None,
        packages: None,
        // _pin: PhantomPinned,
    };
    // CargoCoreWorkspace::init(&mut res, manifest_path) ?;
    res.init(manifest_path) ?;

    /*
    res.context.shell().set_verbosity(cargo::core::Verbosity::Quiet);

    // let cache_lock: cargo::util::cache_lock::CacheLock<'a> =
    //     res.context.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?);
    res.cache_lock = Some(res.context.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?);
    let workspace: cargo::core::Workspace = cargo::core::Workspace::new(manifest_path, &res.context) ?;
    let (pkg_set, _resolve): (cargo::core::PackageSet, cargo::core::Resolve) = cargo::ops::resolve_ws(&workspace) ?;

    // res.cache_lock = Some(cache_lock);
    res.workspace = Some(workspace);
    res.packages = Some(pkg_set);

     */

    Ok(Box::new(res))
}
*/
