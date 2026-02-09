use crate::{api, error};
use prettytable::{Cell, Row, Table};
use uuid::Uuid;

pub fn run(api_client: &api::Client) -> Result<(), error::Error> {
    let jobs = api_client.list_jobs()?;

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Job ID"),
        Cell::new("Created At"),
        Cell::new("Executed At"),
        Cell::new("command"),
        Cell::new("Args"),
        Cell::new("Output"),
        Cell::new("Agent ID"),
    ]));

    for job in jobs {
        table.add_row(Row::new(vec![
            Cell::new(job.id.to_string().as_str()),
            Cell::new(job.created_at.to_string().as_str()),
            Cell::new(
                job.executed_at
                    .map(|t| t.to_string())
                    .unwrap_or(String::new())
                    .as_str(),
            ),
            Cell::new(job.command.as_str()),
            Cell::new(job.args.join(" ").as_str()),
            Cell::new(job.output.unwrap_or("".to_string()).as_str()),
            Cell::new(job.agent_id.to_string().as_str()),
        ]));
    }

    table.printstd();

    Ok(())
}

pub fn get_job_res(api_client: &api::Client, job_id: &str) -> Result<(), error::Error> {
    let job_id = Uuid::parse_str(job_id)?;
    let job_res = api_client.get_job_results(job_id)?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Job ID"), Cell::new("Job Result")]));

    if let Some(res) = job_res {
        // safe to unwrap cause it is some
        table.add_row(Row::new(vec![
            Cell::new(job_id.to_string().as_str()),
            Cell::new(res.as_str()),
        ]));
    } else {
        table.add_row(Row::new(vec![Cell::new("None"), Cell::new("None")]));
    }

    table.printstd();

    Ok(())
}
