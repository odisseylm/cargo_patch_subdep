use std::path::Path;
//--------------------------------------------------------------------------------------------------


//
/// All cargo package related logic is put here to abstract over it changes easy.
//

type CargoConfig = cargo::util::context::GlobalContext;

pub fn setup_cargo_core_config() -> Result<CargoConfig, anyhow::Error> {
    let config = cargo::util::context::GlobalContext::default()?;
    config.shell().set_verbosity(cargo::core::Verbosity::Quiet);
    Ok(config)
}


pub fn fetch_cargo_core_workspace<'a>(config: &'a CargoConfig, path: &Path)
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
