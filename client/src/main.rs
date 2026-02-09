use clap::{Arg, Command};

mod api;
mod cli;
mod error;

pub const SERVER_URL: &str = "http://localhost:8000";

fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new(cli::AGENTS).about("List all agents"))
        .subcommand(Command::new(cli::JOBS).about("List all jobs"))
        .subcommand(
            Command::new(cli::JOB_INFO)
                .about("Get the result of a job")
                .arg(
                    Arg::new("job")
                        .short('j')
                        .long("job")
                        .help("The id of the job to get the details")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new(cli::EXEC)
                .about("Execute a command")
                .arg(
                    Arg::new("agent")
                        .short('a')
                        .long("agent")
                        .help("The agent id to execute the command on")
                        .required(true),
                )
                .arg(
                    Arg::new("command")
                        .help("The command to execute, with its arguments.")
                        .required(true)
                        .index(1),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    let api_client = api::Client::new(SERVER_URL.to_string());

    if cli.subcommand_matches(cli::AGENTS).is_some() {
        cli::agents::run(&api_client)?;
    } else if cli.subcommand_matches(cli::JOBS).is_some() {
        cli::job::run(&api_client)?;
    } else if let Some(job) = (cli.subcommand_matches(cli::JOB_INFO)) {
        // safe to unwrap require argument
        let job_id: &String = job.get_one("job").unwrap();
        cli::job::get_job_res(&api_client, job_id)?;
    } else if let Some(matches) = cli.subcommand_matches(cli::EXEC) {
        // safe to unwrap required arguments
        let agent_id: &String = matches.get_one("agent").unwrap();
        let command: &String = matches.get_one("command").unwrap();
        cli::exec::run(&api_client, agent_id, command)?;
    }

    Ok(())
}
