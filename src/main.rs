use clap::{Parser, Subcommand};

mod new;
mod run;
mod solutions;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        day: usize,
    },
    Run {
        day: usize,
        #[clap(long, short, default_value_t = false)]
        test: bool,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::New { day } => new::new_day(day),
        Commands::Run { day, test } => run::run_day(day, test),
    }
}
