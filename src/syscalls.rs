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
pub struct Syscall {
    name: String,
    number: u64,
    arguments: Vec<SyscallArgument>,
    #[serde(alias = "return")]
    return_type: SyscallReturn,
}

impl Display for Syscall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_fmt = self.arguments.iter().fold(String::new(), |mut acc, new| {
            acc += format!("{}{}", if acc.len() == 0 { "" } else { ", " }, new).as_str();
            acc
        });

        write!(f, "{}({})", self.name, arg_fmt)
    }
}

impl Display for SyscallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) = {}", self.name, self.arg_type, self.name)
    }
}

#[derive(Debug, Clone)]
pub struct LinuxSyscall {
    lookup_table: HashMap<u64, Syscall>,
}

impl LinuxSyscall {
    fn build() -> Self {
        // Process to lookup table.
        let t: Vec<Value> = serde_json::from_str(MEMORY_SYSCALLS).unwrap();
        let mut m = vec![];

        for v in t {
            let o = Syscall::deserialize(v).unwrap();
            m.push(o);
        }

        let t: Vec<Value> = serde_json::from_str(FILE_SOCKET_SYSCALLS).unwrap();
        for v in t {
            let o = Syscall::deserialize(v).unwrap();
            m.push(o);
        }

        // Build the hashmap.
        let lookup_table = m
            .iter()
            .map(|i| (i.number, i.clone()))
            .collect::<HashMap<u64, Syscall>>();

        Self { lookup_table }
    }

    /// Looks up if the passed register state was captured from
    /// a syscall related to io/memory operations.
    pub fn lookup(&self, _register_state: user_regs_struct) -> Option<Syscall> {
        // Check if rax is in our lookup table.
        self.lookup_table
            .get(&_register_state.orig_rax)
            .map(|t| t.to_owned())
    }

    /// Number of currently loaded syscalls being traced.
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
