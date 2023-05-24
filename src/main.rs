use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[command(author, version, about)]
enum Command {
    /// Find the date that is a given number of days from now
    Count {
        /// The number of days to count from now
        count: u32,
    },
    /// Find the number of days from now until a given date
    Until {
        /// The date to count days until
        date: NaiveDate,
    },
    /// Find the number of days since a given date
    Since {
        /// The date to count days since
        date: NaiveDate,
    },
}

#[derive(Parser)]
#[command(name = "days", bin_name = "days")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Count { count } => {
            let date = Local::now().date_naive();
            let future = date + Duration::days(count as i64);
            println!("{future}");
        }
        Command::Until { date } => {
            let today = Local::now().date_naive();
            let delta = date.signed_duration_since(today);
            println!("{}", delta.num_days());
        }
        Command::Since { date } => {
            let today = Local::now().date_naive();
            let delta = today.signed_duration_since(date);
            println!("{}", delta.num_days());
        }
    }
}
