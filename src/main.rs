use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    contestents: String,
}

fn main() {
    println!("Hello, world!");
}
