use crate::CliCommand;
use crate::GcpLookup;
use std::process::Command;
use std::str;

pub fn search_matcher(args: GcpLookup) {
    match &args.command {
        CliCommand::Project(query) => querier("projects", query.query.to_string()),
        CliCommand::Instance(query) => querier("compute instances", query.query.to_string()),
        CliCommand::Gke(query) => querier("gke", query.query.to_string()),
        CliCommand::Gcs(query) => querier("gcs", query.query.to_string()),
    }
}

fn querier(sub_command: &str, query: String) {
    log::info!("You are searching for '{}' with the expression '{}'", sub_command, query);
    println!("");

    let mut child = Command::new("gcloud")
        .arg(sub_command)
        .arg("list")
        .arg("--filter")
        .arg(query.to_string())
        .spawn()
        .expect("gcp-lookup failed. Ensure you have gcloud installed.");
    
    child.wait().expect("Failed to wait on gcp-lookup");
}
