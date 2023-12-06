use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Heidi Robayo", about = "A Marco Polo game")]

// cli cargo: cargo run -- play --name "Marco"
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No command given"),
    }
}
