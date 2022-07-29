use color_eyre::eyre::Result;

fn main() -> Result<()> {
    // Setup error handling
    color_eyre::install()?;

    Ok(())
}
