fn main() -> Result<(), Box<dyn std::error::Error>> {
    toktok_generator::process("json.toktok")?;

    Ok(())
}
