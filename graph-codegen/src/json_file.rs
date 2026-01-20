use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Result};
use std::path::Path;

pub fn write_json_pretty<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
