use env_logger::Builder;
use log::LevelFilter;
use structopt::StructOpt;

mod matcher;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "gcp-lookup",
    about = "Quick lookup for gcp projects and resources"
)]
pub struct GcpLookup {
    #[structopt(subcommand)]
    command: CliCommand,
}

#[derive(Debug, StructOpt)]
pub enum CliCommand {
    #[structopt(name = "project", about = "search for project")]
    Project(Query),

    #[structopt(name = "instance", about = "search for EC2 instances")]
    Instance(Query),

    #[structopt(name = "gke", about = "search for GKE cluster")]
    Gke(Query),

    #[structopt(name = "image", about = "search for Container Images")]
    Image(Query),
}

#[derive(Debug, StructOpt)]
pub struct Query {
    query: String,
}

fn main() {
    Builder::new().filter(None, LevelFilter::Info).init();

    matcher::search_matcher(GcpLookup::from_args());
}
