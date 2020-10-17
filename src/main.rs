use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Imogen Kinsman <imogen@thezets.com")]
struct Opts {
  #[clap(short, long)]
  token: String,
}

#[derive(Clap)]
enum SubCommand {
  Info(Info),
}

#[derive(Clap)]
struct Info {}

pub fn main() {}
