mod lib;

use clap::Clap;
use lib::Client;

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
    Readiness(ReadinessOpts),
    /// Daily sleep summaries.
    Sleep(SleepOpts),
    /// Daily activity summaries.
    Activity(ActivityOpts),
    /// Optimal bedtime window.
    Bedtime(Bedtime),
}

#[derive(Clap)]
struct Info {}

#[derive(Clap)]
struct ReadinessOpts {
    /// <YYYY-MM-DD> Start date. Defaults to one week from now.
    #[clap(long)]
    start: Option<String>,
    /// <YYYY-MM-DD> End date. Defaults to today.
    #[clap(long)]
    end: Option<String>,
}

#[derive(Clap)]
struct SleepOpts {
    /// <YYYY-MM-DD> Start date. Defaults to one week from now.
    #[clap(long)]
    start: Option<String>,
    /// <YYYY-MM-DD> End date. Defaults to today.
    #[clap(long)]
    end: Option<String>,
}

#[derive(Clap)]
struct ActivityOpts {
    /// <YYYY-MM-DD> Start date. Defaults to one week from now.
    #[clap(long)]
    start: Option<String>,
    /// <YYYY-MM-DD> End date. Defaults to today.
    #[clap(long)]
    end: Option<String>,
}

#[derive(Clap)]
struct Bedtime {}

fn main() {
    let opts: Opts = Opts::parse();
    let client = Client::new(opts.token);

    let result = match opts.subcmd {
        SubCommand::Info(_) => client.info(),
        SubCommand::Readiness(_) => client.readiness(),
        SubCommand::Sleep(_) => client.sleep(),
        SubCommand::Activity(_) => client.activity(),
        SubCommand::Bedtime(_) => client.bedtime(),
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("Some shit went wrong: {:?}", e),
    }
}
