mod lib;

use clap::Clap;
use lib::Client;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Imogen Kinsman <imogen@thezets.com>")]
struct Opts {
    #[clap(short, long)]
    token: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Info(Info),
    Readiness(Readiness),
    Sleep(Sleep),
    Activity(Activity),
}

#[derive(Clap)]
struct Info {}

#[derive(Clap)]
struct Readiness {}

#[derive(Clap)]
struct Sleep {}

#[derive(Clap)]
struct Activity {}

fn main() {
    let opts: Opts = Opts::parse();
    let client = Client::new(opts.token);

    let result = match opts.subcmd {
        SubCommand::Info(_) => client.info(),
        SubCommand::Readiness(_) => client.readiness(),
        SubCommand::Sleep(_) => client.sleep(),
        SubCommand::Activity(_) => client.activity(),
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("Some shit went wrong: {:?}", e),
    }
}
