mod bus;
mod memory;
mod register;
mod register_array;
mod util;

use self::{
    bus::BusState, memory::MemoryState, register::RegisterState, register_array::RegisterArrayState,
};
use crate::{Changed, Result};
use anyhow::anyhow;
use rtcore::{
    common::{BusKind, RegisterKind},
    value::Value,
};
use rtprogram::{Declaration, Ident, Program};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct State {
    registers: HashMap<Ident, RegisterState>,
    buses: HashMap<Ident, BusState>,
    register_arrays: HashMap<Ident, RegisterArrayState>,
    memories: HashMap<Ident, MemoryState>,
}

impl State {
    pub fn init(program: &Program) -> Self {
        let mut registers = HashMap::new();
        let mut buses = HashMap::new();
        let mut memories = HashMap::new();
        let mut register_arrays = HashMap::new();

        // Init all, but memories (memories need access to registers)
        for declaration in program.declarations() {
            match declaration {
                Declaration::Register(declare_register) => {
                    for reg in &declare_register.registers {
                        registers
                            .insert(reg.ident.clone(), RegisterState::init(reg.range, reg.kind));
                    }
                }
                Declaration::Bus(declare_bus) => {
                    for bus in &declare_bus.buses {
                        buses.insert(bus.ident.clone(), BusState::init(bus.range, bus.kind));
                    }
                }
                Declaration::RegisterArray(declare_register_array) => {
                    for reg_array in &declare_register_array.register_arrays {
                        let data_size = reg_array.range.unwrap_or_default().size();
                        register_arrays.insert(
                            reg_array.ident.clone(),
                            RegisterArrayState::init(reg_array.len, data_size),
                        );
                    }
                }
                Declaration::Memory(_) => (),
            }
        }

        // Init memories
        for declaration in program.declarations() {
            if let Declaration::Memory(declare_memory) = declaration {
                for mem in &declare_memory.memories {
                    let ar_size =
                        registers.get(&mem.range.address_register).unwrap().range().size();
                    let dr_size = registers.get(&mem.range.data_register).unwrap().range().size();
                    memories.insert(
                        mem.ident.clone(),
                        MemoryState::init(mem.range.clone(), ar_size, dr_size),
                    );
                }
            }
        }

        Self { registers, buses, memories, register_arrays }
    }

    pub fn clock(&mut self) -> Changed {
        let mut changed = Changed::default();

        for (name, state) in &mut self.registers {
            if state.clock() {
                changed.registers.insert(name.clone());
            }
        }
        for (name, state) in &mut self.memories {
            if let Some(address) = state.clock() {
                changed.memories.insert((name.clone(), address));
            }
        }
        for (name, state) in &mut self.register_arrays {
            if let Some(index) = state.clock() {
                changed.register_arrays.insert((
                    name.clone(),
                    usize::from_str_radix(&index.as_bin(false), 2).unwrap(),
                ));
            }
        }

        changed
    }

    pub fn clear_intern_buses(&self, buses_persist: &HashSet<Ident>) {
        for (ident, bus) in &self.buses {
            if !buses_persist.contains(ident) && bus.kind() == BusKind::Intern {
                bus.write(None, Value::zero(bus.range().size())).unwrap();
            }
        }
    }

    pub fn register_names(&self, kind: RegisterKind) -> impl Iterator<Item = &Ident> {
        self.registers
            .iter()
            .filter_map(move |(name, state)| if state.kind() == kind { Some(name) } else { None })
    }
    pub fn register(&self, name: &Ident) -> Result<&RegisterState> {
        self.registers.get(name).ok_or(anyhow!("register `{}` does not exist", name.0))
    }
    pub fn register_mut(&mut self, name: &Ident) -> Result<&mut RegisterState> {
        self.registers.get_mut(name).ok_or(anyhow!("register `{}` does not exist", name.0))
    }

    pub fn bus_names(&self, kind: BusKind) -> impl Iterator<Item = &Ident> {
        self.buses
            .iter()
            .filter_map(move |(name, state)| if state.kind() == kind { Some(name) } else { None })
    }
    pub fn bus(&self, name: &Ident) -> Result<&BusState> {
        self.buses.get(name).ok_or(anyhow!("bus `{}` does not exist", name.0))
    }
    pub fn bus_mut(&mut self, name: &Ident) -> Result<&mut BusState> {
        self.buses.get_mut(name).ok_or(anyhow!("bus `{}` does not exist", name.0))
    }

    pub fn register_array_names(&self) -> impl Iterator<Item = &Ident> {
        self.register_arrays.keys()
    }
    pub fn register_array(&self, name: &Ident) -> Result<&RegisterArrayState> {
        self.register_arrays.get(name).ok_or(anyhow!("register array `{}` does not exist", name.0))
    }
    pub fn register_array_mut(&mut self, name: &Ident) -> Result<&mut RegisterArrayState> {
        self.register_arrays
            .get_mut(name)
            .ok_or(anyhow!("register array `{}` does not exist", name.0))
    }

    pub fn memory_names(&self) -> impl Iterator<Item = &Ident> {
        self.memories.keys()
    }
    pub fn memory(&self, name: &Ident) -> Result<&MemoryState> {
        self.memories.get(name).ok_or(anyhow!("memory `{}` does not exist", name.0))
    }
    pub fn memory_mut(&mut self, name: &Ident) -> Result<&mut MemoryState> {
        self.memories.get_mut(name).ok_or(anyhow!("memory `{}` does not exist", name.0))
    }
}

// impl fmt::Display for State {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Inputs
//         write!(f, "--- Inputs ---\n")?;
//         let mut inputs = self.bus_names(BusKind::Input).collect::<Vec<_>>();
//         inputs.sort();
//         for (idx, bus) in inputs.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 bus.0,
//                 self.bus(bus).unwrap().read(None).unwrap().as_dec()
//             )?;
//         }
//         write!(f, "\n\n")?;
//
//         // Outputs
//         write!(f, "--- Outputs ---\n")?;
//         let mut outputs = self.register_names(RegisterKind::Output).collect::<Vec<_>>();
//         outputs.sort();
//         for (idx, reg) in outputs.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 reg.0,
//                 self.register(reg).unwrap().read(None).unwrap().as_dec()
//             )?;
//         }
//         write!(f, "\n\n")?;
//
//         // Registers
//         write!(f, "--- Registers ---\n")?;
//         let mut registers = self.register_names(RegisterKind::Intern).collect::<Vec<_>>();
//         registers.sort();
//         for (idx, reg) in registers.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 reg.0,
//                 self.register(reg).unwrap().read(None).unwrap().as_dec()
//             )?;
//         }
//         write!(f, "\n\n")?;
//
//         // Buses
//         write!(f, "--- Buses ---\n")?;
//         let mut buses = self.bus_names(BusKind::Intern).collect::<Vec<_>>();
//         buses.sort();
//         for (idx, bus) in buses.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 bus.0,
//                 self.bus(bus).unwrap().read(None).unwrap().as_dec()
//             )?;
//         }
//         write!(f, "\n\n")?;
//
//         // Register arrays
//         write!(f, "--- Register Arrays ---\n")?;
//         let mut register_arrays = self.register_array_names().collect::<Vec<_>>();
//         register_arrays.sort();
//         for (idx, reg_array) in register_arrays.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 reg_array.0,
//                 self.register_array(reg_array).unwrap()
//             )?;
//         }
//         write!(f, "\n\n")?;
//
//         // Memories
//         write!(f, "--- Memories ---\n")?;
//         let mut memories = self.memory_names().collect::<Vec<_>>();
//         memories.sort();
//         for (idx, mem) in memories.into_iter().enumerate() {
//             write!(
//                 f,
//                 "{}{} = {}",
//                 if idx != 0 { "\n" } else { "" },
//                 mem.0,
//                 self.memory(mem).unwrap()
//             )?;
//         }
//
//         Ok(())
//     }
// }
