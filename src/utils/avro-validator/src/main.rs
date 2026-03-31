use std::fs;

use apache_avro::Schema;
use risingwave_connector_codec::decoder::avro::avro_schema_to_fields;

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .expect("usage: risingwave_avro_validator <path-to-avro-schema>");

    let content = fs::read_to_string(&path)?;
    let schema = Schema::parse_str(&content)?;
    let fields = avro_schema_to_fields(&schema, None)?;

    for field in &fields {
        println!("{}: {:?}", field.name, field.data_type);
    }

    Ok(())
}
