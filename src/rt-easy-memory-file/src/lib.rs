#![deny(rust_2018_idioms)]

use std::collections::HashMap;
use std::fmt;

pub use rtcore::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryFile {
    ar_size: usize,
    dr_size: usize,
    data: HashMap<Value, Value>,
}

impl MemoryFile {
    /// All keys in `data` should have `Value::size() <= ar_size`.
    ///
    /// All values in `data` should have `Value::size() <= dr_size`.
    pub fn new(ar_size: usize, dr_size: usize, data: HashMap<Value, Value>) -> Result<Self, ()> {
        // Check and extend zero
        let data = data
            .into_iter()
            .map(|(mut addr, mut value)| {
                if addr.size() > ar_size {
                    return Err(());
                }
                addr.extend_zero(ar_size);

                if value.size() > dr_size {
                    return Err(());
                }
                value.extend_zero(dr_size);

                Ok((addr, value))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { ar_size, dr_size, data })
    }

    pub fn empty(ar_size: usize, dr_size: usize) -> Self {
        Self { ar_size, dr_size, data: HashMap::new() }
    }

    pub fn ar_size(&self) -> usize {
        self.ar_size
    }

    pub fn dr_size(&self) -> usize {
        self.dr_size
    }

    /// All keys are guaranteed to have `Value::size() == ar_size`.
    ///
    /// All values are guaranteed to have `Value::size() == dr_size`.
    pub fn data(&self) -> &HashMap<Value, Value> {
        &self.data
    }

    /// All keys are guaranteed to have `Value::size() == ar_size`.
    ///
    /// All values are guaranteed to have `Value::size() == dr_size`.
    pub fn into_data(self) -> HashMap<Value, Value> {
        self.data
    }
}

impl MemoryFile {
    pub fn parse(source: &str) -> Result<Self, ()> {
        // Split to lines
        let mut lines = source.lines().map(|line| {
            // Remove comment
            let line = match line.split_once('#') {
                Some((line, _comment)) => line,
                None => line,
            };

            // Trim
            line.trim()
        });

        // Parse header
        let header = lines.next().ok_or(())?;
        let mut parts = header.split(' ');
        let parse_fn = match parts.next() {
            Some("B") | Some("b") => Value::parse_bin,
            Some("H") | Some("h") => Value::parse_hex,
            _ => return Err(()),
        };
        let ar_size = match parts.next() {
            Some(ar_size) => ar_size.parse().map_err(|_| ())?,
            None => return Err(()),
        };
        let dr_size = match parts.next() {
            Some(dr_size) => dr_size.parse().map_err(|_| ())?,
            None => return Err(()),
        };

        // Parse data
        let mut current_address = Value::zero(ar_size);
        let mut data = HashMap::new();
        for line in lines {
            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Parse as address or data
            if line.ends_with(':') {
                let mut v = parse_fn(&line[0..line.len() - 1]).map_err(|_| ())?;
                if v.size() > ar_size {
                    return Err(());
                }
                v.extend_zero(ar_size);

                current_address = v;
            } else {
                let mut v = parse_fn(line).map_err(|_| ())?;
                if v.size() > dr_size {
                    return Err(());
                }
                v.extend_zero(dr_size);

                data.insert(current_address.clone(), v);
                current_address = current_address + Value::one(ar_size);
            }
        }

        // Memory file
        Ok(Self { ar_size, dr_size, data })
    }
}

impl fmt::Display for MemoryFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write header
        writeln!(f, "H {} {}", self.ar_size, self.dr_size)?;
        if !self.data.is_empty() {
            write!(f, "\n")?;
        }

        // Sort data (by address ASC)
        let mut data = self.data.iter().collect::<Vec<_>>();
        data.sort_by(|a, b| a.0.cmp(b.0));

        // Write data
        let mut current_address = Value::zero(self.ar_size);
        for (address, value) in data {
            if *address != current_address {
                writeln!(f, "\n{}:", address.as_hex())?;
            }
            writeln!(f, "{}", value.as_hex())?;

            current_address = address + Value::one(self.ar_size);
        }

        Ok(())
    }
}
