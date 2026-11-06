use ::zeromq::{zeromq, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = zeromq::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.deploy().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-zeromq operational");
    Ok(())
}
