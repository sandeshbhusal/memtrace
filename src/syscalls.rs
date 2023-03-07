#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

use nix::libc::user_regs_struct;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const MEMORY_SYSCALLS: &'static str = include_str!("memory_calls.json");
const FILE_SOCKET_SYSCALLS: &'static str = include_str!("io_calls.json");

#[derive(Debug, Serialize, Clone, Deserialize)]
struct SyscallArgument {
    name: String,
    #[serde(alias = "type")]
    arg_type: String,
    register: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SyscallReturn {
    name: String,
    #[serde(alias = "type")]
    ret_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleSyscall {
    name: String,
    number: u64,
    arguments: Vec<SyscallArgument>,
    #[serde(alias = "return")]
    return_type: SyscallReturn,
}

impl Display for SingleSyscall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name).unwrap();
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub struct LinuxSyscall {
    lookup_table: HashMap<u64, SingleSyscall>,
}

impl LinuxSyscall {
    fn build() -> Self {
        // Process to our table.
        let t: Vec<Value> = serde_json::from_str(MEMORY_SYSCALLS).unwrap();
        let mut m = vec![];

        for v in t {
            let o = SingleSyscall::deserialize(v).unwrap();
            m.push(o);
        }

        let t: Vec<Value> = serde_json::from_str(FILE_SOCKET_SYSCALLS).unwrap();
        for v in t {
            let o = SingleSyscall::deserialize(v).unwrap();
            m.push(o);
        }

        // Build the hashmap.
        let lookup_table = m
            .iter()
            .map(|i| (i.number, i.clone()))
            .collect::<HashMap<u64, SingleSyscall>>();

        Self { lookup_table }
    }

    pub fn lookup(&self, _register_state: user_regs_struct) -> Option<SingleSyscall> {
        // Check if rax is in our lookup table.
        if let Some(rax_syscall) = self.lookup_table.get(&_register_state.rax) {
            Some(rax_syscall.clone())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.lookup_table.len()
    }
}

pub fn syscalls() -> &'static LinuxSyscall {
    &__SYSCALLS
}

lazy_static::lazy_static! {
    pub static ref __SYSCALLS: LinuxSyscall = LinuxSyscall::build();
}
