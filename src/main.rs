use clap::Parser;

/// Monitoring tool for devices connected to your Ethernet network
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Monitoring tool for devices connected to your Ethernet network",
    long_about = "Argos is a monitoring tool for devices connected to your Ethernet network. It uses the ARP protocol to list devices currently connected to the network by their MAC addresses."
)]
struct Cli {
    /// Minutes interval the loop is called periodically
    #[arg(short, long, default_value_t = 1)]
    loop_minutes: u8,
}

fn main() {
    let args = Cli::parse();

    println!("loop_minutes: {:?}", args.loop_minutes);
    println!("Hello, world!");
}
