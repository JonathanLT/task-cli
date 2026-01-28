mod commands;
mod display;

use clap::{ArgAction, Command, arg, command};
use task_cli::{storage, task};

///
/// Powerful commands-line task manager with data persistence
///
fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(
                    arg!([description] "Task description")
                        .required(true)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-t --tags [tags] "Tags, split by comma")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-p --priority [priority] "Priority level")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-d --due [due] "Due date")
                        .required(false)
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task")
                .arg(arg!([id] "Task id").required(true).action(ArgAction::Set))
                .arg(
                    arg!([description] "Task description")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-t --tags [tags] "Tags, split by comma")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-p --priority [priority] "Priority level")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-s --status [status] "Task status")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-d --due [due] "Due date")
                        .required(false)
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("list").about("Lists all tasks").arg(
                arg!(-s --status [status] "Task status")
                    .required(false)
                    .action(ArgAction::Append),
            ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a task")
                .arg(arg!([id] "Task id").required(true).action(ArgAction::Set))
                .arg(
                    arg!(-f --force "Delete without confirmation")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("complete")
                .about("Completes a task")
                .arg(arg!([id] "Task id").required(true).action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("search")
                .about("Searches tasks")
                .arg(
                    arg!([pattern] "Searching pattern")
                        .required(true)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-t --tag [tag] "Tag name")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-p --priority [priority] "Priority level")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-d --due [due] "Due date")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    arg!(-s --status [status] "Task status")
                        .required(false)
                        .action(ArgAction::Append),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => commands::add::handle_add(sub_m),
        Some(("edit", sub_m)) => commands::edit::handle_edit(sub_m),
        Some(("list", sub_m)) => commands::list::handle_list(sub_m),
        Some(("delete", sub_m)) => commands::delete::handle_delete(sub_m),
        Some(("complete", sub_m)) => commands::complete::handle_complete(sub_m),
        Some(("search", sub_m)) => commands::search::handle_search(sub_m),
        _ => unreachable!(),
    }
    // Continued program logic goes here...
}
