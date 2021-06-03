use crate::Command;
use crate::GcpLookup;

pub fn search_matcher(args: GcpLookup) {
    match &args.command {
        Command::Project(query) => querier("project", query.query.to_string()),
        Command::Instance(query) => querier("instance", query.query.to_string()),
        Command::Gke(query) => querier("gke", query.query.to_string()),
        Command::Gcs(query) => querier("gcs", query.query.to_string()),
    }
}

fn querier(sub_command: &str, query: String) {
    log::info!("You are searching for {} in {:?}", query, sub_command);
}
