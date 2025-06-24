use rprime::{pollard_rho, wheel_factorize};
use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum FactorizationMethod {
    PollardRho,
    Wheel,
}

#[derive(Parser, Debug)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    about = "Find the prime factors of a positive integer"
)]
struct Args {
    #[clap(short, long, help = "The number to factorise")]
    number: u128,
    
    #[clap(
        short = 'm', 
        long, 
        value_enum, 
        default_value_t = FactorizationMethod::Wheel, 
        help = "Factorization method to use"
    )]
    method: FactorizationMethod,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let factors = match args.method {
        FactorizationMethod::PollardRho => pollard_rho(args.number)?,
        FactorizationMethod::Wheel => wheel_factorize(args.number)?,
    };
    println!("Factors of {} are: {:?}", args.number, factors);
    Ok(())
}