fn main() -> Result<(), Box<dyn std::error::Error>> {
    toktok_generator::process("unit_test.toktok")?;

    Ok(())
}
