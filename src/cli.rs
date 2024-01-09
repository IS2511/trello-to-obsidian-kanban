use clap::Parser;
use clio::{ClioPath, InputPath, OutputPath};

/// Convert a Trello board export (JSON) to an Obsidian Kanban board
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to input file, '-' for stdin
    /// 
    /// Format: Trello JSON
    /// 
    /// More info: https://support.atlassian.com/trello/docs/making-sense-of-trellos-json-export/
    #[arg(short, long, default_value = "-")]
    input: InputPath,

    /// Path to output file, '-' for stdout
    /// 
    /// Format: Markdown - Obsidian Kanban
    /// 
    /// More info: https://github.com/mgmeyers/obsidian-kanban/
    #[arg(short, long, default_value = "-")]
    output: OutputPath,

    /// Download attachments, IDs as filenames
    #[arg(long, default_value_t = false)]
    download_attachments: bool,

    /// Path to directory for attachments
    #[arg(long, default_value = "attachments")]
    attachments_path: ClioPath,

    /// Trello API key (for attachments)
    #[arg(short = 'k', long)]
    api_key: Option<String>,
}


