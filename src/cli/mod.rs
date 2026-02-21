use clap::{Command, crate_name, crate_version};

mod ssh;

pub fn run() {
    let app = app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("ssh", matches)) => ssh::execute(matches),
        _ => unreachable!(),
    }
}

fn app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .long_version(crate_version!())
        .subcommand_required(true)
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(ssh::command())
}
