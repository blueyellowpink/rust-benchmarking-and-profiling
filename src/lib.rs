use std::{
    convert::TryInto,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use csv::Reader;

#[cfg(feature = "unoptimized")]
use serde_bytes::ByteBuf;
#[cfg(feature = "unoptimized")]
use std::collections::HashMap;
#[cfg(feature = "unoptimized")]
type Record = HashMap<String, ByteBuf>;

const NULL: &'static str = "NULL"; // or whatever.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Field {
    Unknown,
    String,
    Integer,
    Float,
}

pub fn read_file(data: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(data)?;
    let meta = file.metadata()?;
    let mut reader = BufReader::new(file);
    let mut contents = Vec::with_capacity(meta.len().try_into()?);
    reader.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn read_csv(data: &[u8]) -> Result<Vec<Option<Field>>, Box<dyn Error>> {
    let mut reader = Reader::from_reader(data);
    let mut fields = vec![];

    #[cfg(feature = "unoptimized")]
    let headers = reader.headers().unwrap().clone().into_byte_record();
    #[cfg(feature = "unoptimized")]
    for record in reader.byte_records() {
        let record = record?;
        let record: Record = record.deserialize(Some(&headers))?;
        for (_, value) in record.into_iter() {
            fields.push(parse(&value));
        }
    }

    #[cfg(not(feature = "unoptimized"))]
    let mut record = csv::ByteRecord::new();
    #[cfg(not(feature = "unoptimized"))]
    while !reader.is_done() {
        reader.read_byte_record(&mut record).unwrap();
        for value in record.iter() {
            fields.push(parse(&value));
        }
    }

    Ok(fields)
}

fn parse(bytes: &[u8]) -> Option<Field> {
    let string = match std::str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_) => return Some(Field::Unknown),
    };
    if string == NULL {
        return None;
    }
    if string.parse::<i64>().is_ok() {
        return Some(Field::Integer);
    };
    if string.parse::<f64>().is_ok() {
        return Some(Field::Float);
    };
    Some(Field::String)
}
