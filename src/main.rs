extern crate clap;
extern crate hubcaps;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate indicatif;

use tokio_core::reactor::Core;
use hubcaps::Github;
use futures::future::Future;
use hubcaps::issues::{Issues, IssueListOptions};

fn main() {

    let core       = Core::new().expect("Cannot initialize tokyo Core");
    let github     = Github::new(
                        "user-agent",
                        None,
                        &core.handle());
    let issues     = Issues::new(github, "matthiasbeyer", "kairos");

    println!("start fetching issues");

    let issue_list = issues.list(&IssueListOptions::default())
        .wait()
        .expect("Failure getting issues");

    println!("end fetching issues");
}

