mod lib;

use clap::Clap;
use lib::{Client, TimeOpts};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Imogen Kinsman <imogen@thezets.com>")]
struct Opts {
    /// A personal access token.
    #[clap(short, long)]
    token: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// User info.
    Info(Info),
    /// Daily readiness summaries.
    Readiness(TimeOpts),
    /// Daily sleep summaries.
    Sleep(TimeOpts),
    /// Daily activity summaries.
    Activity(TimeOpts),
    /// Optimal bedtime window.
    Bedtime(Bedtime),
}

#[derive(Clap)]
struct Info {}

#[derive(Clap)]
struct Bedtime {}

fn main() {
    let opts: Opts = Opts::parse();
    let client = Client::new(opts.token);

    let result = match opts.subcmd {
        SubCommand::Info(_) => client.info(),
        SubCommand::Readiness(_) => client.readiness(),
        SubCommand::Sleep(_) => client.sleep(),
        SubCommand::Activity(time_opts) => client.activity(time_opts),
        SubCommand::Bedtime(_) => client.bedtime(),
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("Some shit went wrong: {:?}", e),
    }
}
