pub mod config;
pub mod discovery;
pub mod library;
pub mod model;
pub mod settings;
pub mod tui;

use clap::Parser;

use crate::config::load_config;
use crate::discovery::DEFAULT_MAX_DEPTH;
use crate::discovery::find_claude_files;
use crate::discovery::find_global_claude_file;
use crate::model::Cli;
use crate::model::ExitOutcome;
use crate::model::SourceRoot;
use crate::tui::app::App;

pub fn run() -> ExitOutcome {
    let cli = Cli::parse();
    let config = load_config().unwrap_or_default();

    // CLI args override config; config overrides built-in defaults.
    let is_default_paths = cli.paths.len() == 1 && cli.paths[0] == std::path::Path::new(".");
    let paths = if is_default_paths {
        config
            .default_paths
            .as_deref()
            .unwrap_or(&cli.paths)
            .to_vec()
    } else {
        cli.paths.clone()
    };
    let depth = cli
        .depth
        .or(config.default_depth)
        .unwrap_or(DEFAULT_MAX_DEPTH);

    let mut roots: Vec<SourceRoot> = Vec::new();
    let mut failed_count: usize = 0;

    eprintln!(
        "Scanning {} {}...",
        paths.len(),
        if paths.len() == 1 {
            "directory"
        } else {
            "directories"
        }
    );

    for path in &paths {
        if !path.exists() {
            eprintln!("Warning: path does not exist: {}", path.display());
            failed_count += 1;
            continue;
        }
        if !path.is_dir() {
            eprintln!("Warning: not a directory: {}", path.display());
            failed_count += 1;
            continue;
        }

        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
        let files = find_claude_files(&canonical, depth);
        roots.push(SourceRoot {
            path: canonical,
            files,
        });
    }

    if roots.is_empty() && failed_count > 0 {
        return ExitOutcome::AllPathsFailed;
    }

    if let Some(global_path) = find_global_claude_file() {
        let already_found = roots.iter().any(|root| root.files.contains(&global_path));
        if !already_found && let Some(claude_dir) = global_path.parent() {
            roots.insert(
                0,
                SourceRoot {
                    path: claude_dir.to_path_buf(),
                    files: vec![global_path],
                },
            );
        }
    }

    if cli.list {
        print_list(&roots);
    } else {
        let mut terminal = ratatui::init();
        let mut app = App::new(roots, &config);
        let result = app.run(&mut terminal);
        ratatui::restore();
        if let Err(err) = result {
            eprintln!("TUI error: {err}");
        }
    }

    ExitOutcome::Success
}

fn print_list(roots: &[SourceRoot]) {
    let total: usize = roots.iter().map(|r| r.file_count()).sum();

    if total == 0 {
        println!("No CLAUDE.md files found.");
    } else {
        for root in roots {
            println!();
            print!("{root}");
        }
        println!(
            "Found {} CLAUDE.md {} in {} {}.",
            total,
            if total == 1 { "file" } else { "files" },
            roots.len(),
            if roots.len() == 1 {
                "directory"
            } else {
                "directories"
            }
        );
    }
}
