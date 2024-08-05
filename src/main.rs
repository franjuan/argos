use clap::Parser;
use std::process::Command;

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

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("sudo arp-scan --localnet --numeric -g -x")
            .output()
            .expect("failed to execute process")
    };

    print!("{}", String::from_utf8(output.stdout).unwrap());
}
