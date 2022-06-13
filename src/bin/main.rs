use web_abomination::start_server;

fn main() -> std::io::Result<()> {
    start_server("localhost:6969")?;

    Ok(())
}