use clap::Parser;

mod cli;
mod obsidian_kanban;
mod trello;

fn main() {
    let args = cli::Args::parse();
}
