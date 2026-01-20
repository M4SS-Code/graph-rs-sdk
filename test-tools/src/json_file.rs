use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufReader, Result};
use std::path::Path;

pub fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
