use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::Path};

#[derive(Parser)]
#[command(name = "symfoni", version, about = "Symfoni CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize coordination files in the current repository
    Init {
        #[arg(long, default_value = "swift-rust")]
        preset: String,
        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },
    /// Task helpers for GSD tasks
    Task {
        #[command(subcommand)]
        cmd: TaskCommand,
    },
}

#[derive(Subcommand)]
enum TaskCommand {
    /// Create a new GSD task file from templates/task.gsd.md
    New {
        /// File name (placed under tasks/)
        name: String,
        #[arg(long)]
        force: bool,
    },
    /// Validate that required sections exist
    Validate { path: String },
    /// Generate an agent-ready prompt from a GSD task
    Prompt { path: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Init { preset, force } => init(&preset, force),
        Command::Task { cmd } => match cmd {
            TaskCommand::New { name, force } => task_new(&name, force),
            TaskCommand::Validate { path } => task_validate(&path),
            TaskCommand::Prompt { path } => task_prompt(&path),
        },
    }
}

fn write_file(path: &str, content: &str, force: bool) -> Result<()> {
    let p = Path::new(path);
    if p.exists() && !force {
        bail!("Refusing to overwrite {} (use --force)", path);
    }
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create dir {:?}", parent))?;
    }
    fs::write(p, content).with_context(|| format!("write {}", path))?;
    Ok(())
}

fn init(preset: &str, force: bool) -> Result<()> {
    if preset != "swift-rust" {
        // keep simple for now
        bail!("Unknown preset: {} (supported: swift-rust)", preset);
    }

    // Minimal set of files; users can customize after install.
    write_file(
        ".github/pull_request_template.md",
        include_str!("../../../.github/pull_request_template.md"),
        force,
    )?;
    write_file(
        ".github/copilot-instructions.md",
        include_str!("../../../.github/copilot-instructions.md"),
        force,
    )?;
    write_file(
        ".github/ISSUE_TEMPLATE/vertical-slice.yml",
        include_str!("../../../.github/ISSUE_TEMPLATE/vertical-slice.yml"),
        force,
    )?;
    write_file(
        ".github/workflows/swift.yml",
        include_str!("../../../.github/workflows/swift.yml"),
        force,
    )?;
    write_file(
        ".github/workflows/rust.yml",
        include_str!("../../../.github/workflows/rust.yml"),
        force,
    )?;
    write_file(
        ".codex/repo-instructions.md",
        include_str!("../../../.codex/repo-instructions.md"),
        force,
    )?;
    write_file("AGENTS.md", include_str!("../../../AGENTS.md"), force)?;
    write_file("CLAUDE.md", include_str!("../../../CLAUDE.md"), force)?;
    write_file(
        "tools/verify.sh",
        include_str!("../../../tools/verify.sh"),
        force,
    )?;
    write_file(
        "tools/format.sh",
        include_str!("../../../tools/format.sh"),
        force,
    )?;
    write_file(
        "templates/task.gsd.md",
        include_str!("../../../templates/task.gsd.md"),
        force,
    )?;

    println!("Initialized symfoni files (preset: {}).", preset);
    println!(
        "Next: customize Xcode scheme names in .github/workflows/swift.yml and tools/verify.sh."
    );
    Ok(())
}

fn task_new(name: &str, force: bool) -> Result<()> {
    let file = format!("tasks/{}.gsd.md", name);
    write_file(&file, include_str!("../../../templates/task.gsd.md"), force)?;
    println!("Created {}", file);
    Ok(())
}

fn task_validate(path: &str) -> Result<()> {
    let s = fs::read_to_string(path).with_context(|| format!("read {}", path))?;
    let required = [
        "## Goal",
        "## Non-Goals",
        "## Constraints",
        "## Acceptance Criteria",
        "## Definition of Done",
        "## Verification Commands",
    ];
    let mut missing = vec![];
    for r in required {
        if !s.contains(r) {
            missing.push(r);
        }
    }
    if !missing.is_empty() {
        bail!("Task missing sections: {}", missing.join(", "));
    }
    println!("OK: {}", path);
    Ok(())
}

fn task_prompt(path: &str) -> Result<()> {
    let s = fs::read_to_string(path).with_context(|| format!("read {}", path))?;
    println!(
        "Execute the following GSD task strictly. Do not invent scope. Keep PRs small and run verification commands.\n\n{}",
        s
    );
    Ok(())
}
