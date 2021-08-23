fn main() -> Result<(), Box<dyn std::error::Error>> {
    toktok_generator::process("rt_easy.toktok")?;

    Ok(())
}
