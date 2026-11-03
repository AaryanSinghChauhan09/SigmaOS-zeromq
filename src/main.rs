use anyhow::Result;
use clap::Parser;
use zeromq::zeromq;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = zeromq::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = zeromq::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-zeromq deployed");
    Ok(())
}
