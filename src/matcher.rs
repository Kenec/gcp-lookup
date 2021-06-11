use crate::CliCommand;
use crate::GcpLookup;
use std::process::Command;
use std::str;

pub fn search_matcher(args: GcpLookup) {
    match &args.command {
        CliCommand::Project(query) => querier("projects", query.query.to_string()),
        CliCommand::Instance(query) => querier("compute instances", query.query.to_string()),
        CliCommand::Gke(query) => querier("container clusters", query.query.to_string()),
        CliCommand::Image(query) => querier("container images", query.query.to_string()),
    }
}

fn querier(sub_command: &str, query: String) {
    log::info!("You are searching for '{}' with the expression '{}'", sub_command, query);
    println!("");

    let commands  = sub_command.split(" ");
    let mut child = Command::new("gcloud")
        .args(commands)
        .arg("list")
        .arg("--filter")
        .arg(query.to_string())
        .spawn()
        .expect("gcp-lookup failed. Ensure you have gcloud installed.");
    
    child.wait().expect("Failed to wait on gcp-lookup");
}
