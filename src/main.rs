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
}

#[derive(Clap)]
struct Info {}

fn main() {
  let opts: Opts = Opts::parse();

  println!("Value for token: {}", opts.token);

  let client = Client::new(opts.token);

  let result = match opts.subcmd {
    SubCommand::Info(_) => client.info(),
  };

  match result {
    Ok(_) => (),
    Err(e) => println!("Some shit went wrong: {:?}", e),
  }
}
