use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};

mod scaffold;
mod templates;

#[derive(Parser, Debug)]
#[command(name = "gibrust", version, about = "Scaffold and manage Rust REST API projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "create:project", about = "Initialize a new REST API project")]
    CreateProject(CreateProjectArgs),

    #[command(name = "create:domain", about = "Generate a new domain (DDD folder with files) and register routes/docs")]
    CreateDomain(CreateDomainArgs),

    #[command(name = "migrate:generate", about = "Generate Diesel migration SQL files")]
    MigrateGenerate(MigrateGenerateArgs),

    #[command(name = "migrate:run", about = "Run Diesel migrations")]
    MigrateRun(MigrateRunArgs),

    #[command(name = "run:dev", about = "Run server in development mode")]
    RunDev(RunArgs),

    #[command(name = "run:prod", about = "Build and run in production mode")]
    RunProd(RunArgs),

    #[command(name = "run:build", about = "Build project binary")]
    RunBuild(RunArgs),

    #[command(name = "create:docs", about = "Generate OpenAPI docs to docs/swagger.json")]
    CreateDocs(CreateDocsArgs),
}

#[derive(Args, Debug)]
struct CreateProjectArgs {
    #[arg(long, value_name = "NAME")]
    name: String,
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

#[derive(Args, Debug)]
struct CreateDomainArgs {
    #[arg(long, value_name = "NAME")]
    name: String,
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

#[derive(Args, Debug, Default)]
struct MigrateGenerateArgs {
    #[arg(long, value_name = "NAME")]
    name: Option<String>,
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

#[derive(Args, Debug, Default)]
struct MigrateRunArgs {
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

#[derive(Args, Debug, Default)]
struct RunArgs {
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

#[derive(Args, Debug, Default)]
struct CreateDocsArgs {
    #[arg(long, value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::CreateProject(args) => scaffold::create_project(args),
        Commands::CreateDomain(args) => scaffold::create_domain(args),
        Commands::MigrateGenerate(args) => scaffold::migrate_generate(args),
        Commands::MigrateRun(args) => scaffold::migrate_run(args),
        Commands::RunDev(args) => scaffold::run_dev(args),
        Commands::RunProd(args) => scaffold::run_prod(args),
        Commands::RunBuild(args) => scaffold::run_build(args),
        Commands::CreateDocs(args) => scaffold::create_docs(args),
    }
    .context("command failed")
}
