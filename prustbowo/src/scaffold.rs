use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use chrono::Local;
use duct::cmd;
use include_dir::{include_dir, Dir};
use walkdir::WalkDir;

use crate::{CreateDocsArgs, CreateDomainArgs, CreateProjectArgs, MigrateGenerateArgs, MigrateRunArgs, RunArgs};

static TEMPLATE_PROJECT: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/project");
static TEMPLATE_DOMAIN: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/domain");

pub fn create_project(args: CreateProjectArgs) -> Result<()> {
    let project_dir = args.dir.join(&args.name);
    if project_dir.exists() {
        bail!("target directory already exists: {}", project_dir.display());
    }
    fs::create_dir_all(&project_dir).with_context(|| format!("create dir {}", project_dir.display()))?;

    // Copy template files
    TEMPLATE_PROJECT.extract(&project_dir).context("extract project template")?;

    // Replace placeholders like __PROJECT_NAME__ and __CRATE_IDENT__ in files
    replace_placeholders(&project_dir, "__PROJECT_NAME__", &args.name)?;
    let crate_ident = args.name.replace('-', "_");
    replace_placeholders(&project_dir, "__CRATE_IDENT__", &crate_ident)?;

    // Initialize git repo
    let _ = cmd("git", ["init"]).dir(&project_dir).run();
    let _ = cmd("git", ["add", "."]).dir(&project_dir).run();
    let _ = cmd("git", ["commit", "-m", "chore: init from prustbowo"]).dir(&project_dir).run();

    Ok(())
}

pub fn create_domain(args: CreateDomainArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    let domain_dir = project_dir.join("src/app/domain").join(&args.name);
    fs::create_dir_all(&domain_dir).with_context(|| format!("create dir {}", domain_dir.display()))?;

    TEMPLATE_DOMAIN.extract(&domain_dir).context("extract domain template")?;
    replace_placeholders(&domain_dir, "__DOMAIN_NAME__", &args.name)?;

    // Register route and utoipa
    let route_file = project_dir.join("src/app/route.rs");
    register_route_and_docs(&route_file, &args.name)?;

    // Register domain module in src/app/domain/mod.rs
    let domain_root_mod = project_dir.join("src/app/domain/mod.rs");
    register_domain_mod(&domain_root_mod, &args.name)?;

    Ok(())
}

pub fn migrate_generate(args: MigrateGenerateArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    let migrations_dir = project_dir.join("db/migrations");
    fs::create_dir_all(&migrations_dir)?;

    match args.name {
        Some(name) => create_migration_pair(&migrations_dir, &name)?,
        None => {
            // generate for all domains present
            let domain_root = project_dir.join("src/app/domain");
            if domain_root.exists() {
                for entry in fs::read_dir(domain_root)? {
                    let entry = entry?;
                    if entry.file_type()?.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        create_migration_pair(&migrations_dir, &format!("create_{}_table", name))?;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn migrate_run(args: MigrateRunArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    let status = cmd("cargo", ["run", "--bin", "migrate"]).dir(&project_dir).run();
    if let Err(err) = status {
        eprintln!("failed to run migrations via project migrate bin: {}", err);
        // Fallback: run diesel cli if available
        let _ = cmd("diesel", ["migration", "run"]).dir(&project_dir).run();
    }
    Ok(())
}

pub fn run_dev(args: RunArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    cmd("cargo", ["run", "--bin", "server"]).dir(&project_dir).run().context("cargo run --bin server")?;
    Ok(())
}

pub fn run_prod(args: RunArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    cmd("cargo", ["build", "--release"]).dir(&project_dir).run().context("cargo build --release")?;
    let exec = project_dir.join("target/release/server");
    if exec.exists() {
        cmd(exec, std::iter::empty::<&str>()).dir(&project_dir).run().context("run release binary")?;
    }
    Ok(())
}

pub fn run_build(args: RunArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    cmd("cargo", ["build"]).dir(&project_dir).run().context("cargo build")?;
    Ok(())
}

pub fn create_docs(args: CreateDocsArgs) -> Result<()> {
    let project_dir = args.dir.canonicalize().unwrap_or(args.dir);
    let docs_dir = project_dir.join("docs");
    fs::create_dir_all(&docs_dir)?;
    let out = cmd("cargo", ["run", "--bin", "export-openapi"]).dir(&project_dir).read();
    match out {
        Ok(json) => {
            fs::write(docs_dir.join("swagger.json"), json).context("write docs/swagger.json")?;
        }
        Err(err) => {
            eprintln!("failed to export OpenAPI via project export-openapi bin: {}", err);
        }
    }
    Ok(())
}

fn replace_placeholders(root: &Path, key: &str, value: &str) -> Result<()> {
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let mut content = fs::read_to_string(path).unwrap_or_default();
            if content.contains(key) {
                content = content.replace(key, value);
                fs::write(path, content).with_context(|| format!("replace {} in {}", key, path.display()))?;
            }
        }
    }
    Ok(())
}

fn register_route_and_docs(route_file: &Path, domain_name: &str) -> Result<()> {
    let content = fs::read_to_string(route_file).with_context(|| format!("read {}", route_file.display()))?;
    let mut new_content = content.clone();
    let marker_import = "// <prustbowo:imports>";
    let marker_routes = "// <prustbowo:routes>";
    if !content.contains(marker_import) || !content.contains(marker_routes) {
        bail!("route.rs missing markers {} and {}", marker_import, marker_routes);
    }
    if let Some(idx) = new_content.find(marker_import) {
        let insert_at = idx + marker_import.len();
        new_content.insert_str(
            insert_at,
            &format!(
                "\nuse crate::app::domain::{d}::handler::register_{d}_routes;\n",
                d = domain_name
            ),
        );
    }
    if let Some(idx) = new_content.find(marker_routes) {
        let insert_at = idx + marker_routes.len();
        new_content.insert_str(insert_at, &format!("\n    router = register_{d}_routes(router);\n", d = domain_name));
    }
    fs::write(route_file, new_content).with_context(|| format!("write {}", route_file.display()))?;
    Ok(())
}

fn create_migration_pair(migrations_dir: &Path, name: &str) -> Result<()> {
    let ts = Local::now().format("%Y%m%d%H%M%S");
    let dir_name = format!("{}_{}", ts, name);
    let dir = migrations_dir.join(dir_name);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("up.sql"), "-- TODO: write up migration")?;
    fs::write(dir.join("down.sql"), "-- TODO: write down migration")?;
    Ok(())
}

fn register_domain_mod(domain_root_mod: &Path, domain_name: &str) -> Result<()> {
    let mut content = fs::read_to_string(domain_root_mod).unwrap_or_default();
    let decl = format!("pub mod {};", domain_name);
    if !content.contains(&decl) {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&decl);
        content.push('\n');
        fs::write(domain_root_mod, content).with_context(|| format!("write {}", domain_root_mod.display()))?;
    }
    Ok(())
}

