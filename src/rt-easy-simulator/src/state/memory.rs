use super::State;
use crate::Error;
use anyhow::anyhow;
use memory_file::MemoryFile;
use rtcore::value::Value;
use rtprogram::MemoryRange;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

const MEMORY_PAGE_SIZE_EXP: usize = 5;
const MEMORY_PAGE_SIZE: usize = 2usize.pow(MEMORY_PAGE_SIZE_EXP as u32);

#[derive(Debug)]
pub struct MemoryState {
    data: HashMap<Value, Value>,
    data_next: RefCell<Option<(Value, Value)>>,
    range: MemoryRange,
    ar_size: usize,
    dr_size: usize,
}

impl MemoryState {
    pub fn init(range: MemoryRange, ar_size: usize, dr_size: usize) -> Self {
        Self { data: HashMap::new(), data_next: RefCell::new(None), range, ar_size, dr_size }
    }

    pub fn value_next(&self) -> Option<(Value, Value)> {
        self.data_next.borrow().clone()
    }

    pub fn read(&self, state: &State) -> Result<(), Error> {
        // Get AR value
        let ar_value = state.register(&self.range.address_register)?.read(None)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Read from memory
        let value = self.data.get(&ar_value).cloned().unwrap_or_else(|| Value::zero(self.dr_size));

        // Write into data_register
        state.register(&self.range.data_register)?.write(None, value)?;

        Ok(())
    }

    pub fn write(&self, state: &State) -> Result<(), Error> {
        // Get AR value
        let ar_value = state.register(&self.range.address_register)?.read(None)?;
        debug_assert_eq!(ar_value.size(), self.ar_size);

        // Get DR value
        let dr_value = state.register(&self.range.data_register)?.read(None)?;
        debug_assert_eq!(dr_value.size(), self.dr_size);

        // Write to memory
        *self.data_next.borrow_mut() = Some((ar_value, dr_value));

        Ok(())
    }

    pub fn clock(&mut self) -> Option<Value> {
        match self.data_next.get_mut().take() {
            Some((ar_value, dr_value)) => {
                self.data.insert(ar_value.clone(), dr_value);
                Some(ar_value)
            }
            None => None,
        }
    }

    pub fn write_at(&mut self, addr: Value, value: Value) -> Result<(), Error> {
        // Check addr and value
        if addr.size() > self.ar_size {
            return Err(anyhow!("address too big"));
        }
        if value.size() > self.dr_size {
            return Err(anyhow!("value too big"));
        }

        // Reset data_next if same address
        if let Some((data_next_addr, _)) = self.data_next.get_mut() {
            if *data_next_addr == addr {
                *self.data_next.get_mut() = None;
            }
        }

        // Insert data
        self.data.insert(addr, value);

        Ok(())
    }

    pub fn page_count(&self) -> Value {
        if self.ar_size <= MEMORY_PAGE_SIZE_EXP {
            Value::one(1)
        } else {
            Value::one(self.ar_size + 1) << (self.ar_size - MEMORY_PAGE_SIZE_EXP)
        }
    }

    pub fn page_prev(&self, page_nr: Value) -> Option<Value> {
        if page_nr <= Value::one(1) {
            return None;
        }

        Some(page_nr - Value::one(1))
    }

    pub fn page_next(&self, page_nr: Value) -> Option<Value> {
        if page_nr >= self.page_count() {
            return None;
        }

        // Prevent overflow
        let size = page_nr.size() + 1;
        let page_nr = page_nr.with_size(size);

        Some(page_nr + Value::one(1))
    }

    pub fn page_nr_of_address(&self, address: Value) -> Option<Value> {
        let page_idx = address >> MEMORY_PAGE_SIZE_EXP;
        let page_nr = page_idx + Value::one(1);

        // Check in range (1..=page_count)
        if page_nr < Value::one(1) || page_nr > self.page_count() {
            return None;
        }

        Some(page_nr)
    }

    pub fn page(&self, page_nr: Value) -> Vec<(Value, Value)> {
        // Check in range (1..=page_count)
        if page_nr < Value::one(1) || page_nr > self.page_count() {
            return Vec::new();
        }

        // Page idx
        let page_idx = page_nr - Value::one(1);

        // Calc addr (with size = ar_size)
        let mut addr = page_idx.with_size(self.ar_size) << MEMORY_PAGE_SIZE_EXP;

        // Read from memory
        let mut result = Vec::new();
        for _ in 0..MEMORY_PAGE_SIZE {
            let addr_next = &addr + Value::one(1);
            let value = self.data.get(&addr).cloned().unwrap_or_else(|| Value::zero(self.dr_size));
            result.push((addr, value));
            addr = addr_next;

            // Break on overflow
            if addr.is_zero() {
                break;
            }
        }
        result
    }

    pub fn save<W>(&self, mut writer: W) -> Result<(), Error>
    where
        W: io::Write,
    {
        let mem =
            MemoryFile { ar_size: self.ar_size, dr_size: self.dr_size, data: self.data.clone() };
        writer
            .write_all(mem.to_string().as_bytes())
            .map_err(|e| anyhow!("failed to save memory: {}", e))
    }

    pub fn load_from_save<R>(&mut self, mut reader: R) -> Result<(), Error>
    where
        R: io::Read,
    {
        // Read
        let mut source = String::new();
        reader.read_to_string(&mut source).map_err(|_| anyhow!("failed to read memory file"))?;

        // Parse file
        let mem = match MemoryFile::parse(&source) {
            Ok(mem) => mem,
            Err(()) => {
                // Try to parse as deprecated
                parse_deprecated(&source).map_err(|()| anyhow!("invalid memory file"))?
            }
        };

        // Check
        if mem.ar_size != self.ar_size || mem.dr_size != self.dr_size {
            return Err(anyhow!("invalid memory size"));
        }

        // Load data
        self.data = mem
            .data
            .into_iter()
            .map(|(addr, value)| {
                if addr.size() > self.ar_size || value.size() > self.dr_size {
                    return Err(anyhow!("memory save contains faulty data"));
                }
                Ok((addr, value))
            })
            .collect::<Result<_, _>>()?;
        *self.data_next.get_mut() = None;

        Ok(())
    }

    pub fn dr_size(&self) -> usize {
        self.dr_size
    }
}

// impl fmt::Display for MemoryState {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut addresses = self.data.keys().collect::<Vec<_>>();
//         addresses.sort();
//
//         write!(f, "[\n")?;
//         for addr in addresses {
//             write!(f, "  {} = {}\n", addr.as_dec(), self.data.get(addr).unwrap().as_dec())?;
//         }
//         write!(f, "]")?;
//
//         Ok(())
//     }
// }

// Keep this for compatibility with older versions of memory saves
fn parse_deprecated(source: &str) -> Result<MemoryFile, ()> {
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct MemorySave {
        ar_size: usize,
        dr_size: usize,
        data: Vec<(String, String)>,
    }

    let save = serde_json::from_str::<MemorySave>(source).map_err(|_| ())?;

    Ok(MemoryFile {
        ar_size: save.ar_size,
        dr_size: save.dr_size,
        data: save
            .data
            .into_iter()
            .map(|(addr, value)| Ok((Value::parse_hex(&addr)?, Value::parse_hex(&value)?)))
            .collect::<Result<_, _>>()?,
    })
}
