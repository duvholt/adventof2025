use adventof2025::day_tasks;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,

    /// Task to run
    #[arg(short, long)]
    task: String,
}

fn main() {
    let args: Args = Args::parse();

    let contents = fs::read_to_string(args.input).unwrap();

    let map = day_tasks();

    let result = map.get(args.task.as_str()).expect("Unknown task")(contents);
    println!("Result for {}: \n{}", args.task, result);
}
