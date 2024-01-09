mod json;

pub fn parse_json(reader: &mut impl std::io::Read) -> Result<json::Board, anyhow::Error> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer;

    Ok(serde_json::from_str(&buffer)?)
}
