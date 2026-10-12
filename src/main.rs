//! A command-line SSH tarpit server.

/// Builds the command-line interface and dispatches its subcommands.
mod cli;
/// Runs a TCP tarpit server that keeps clients occupied with delayed output.
mod tarpit;

fn main() {
    cli::run();
}
