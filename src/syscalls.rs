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
    #[serde(skip)]
    captured_regs: Option<user_regs_struct>,
    #[serde(skip)]
    is_memory_syscall: bool,
}

impl Display for Syscall {
    // TODO: Add a result output as well. For e.g. "-1 EINVAL (invalid argument)"
    // as `strace` does.

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [", self.name)?;

        let get_reg_val = |regname: &str| match regname {
            "rax" => self.captured_regs.unwrap().orig_rax,
            "rbx" => self.captured_regs.unwrap().rbx,
            "rdi" => self.captured_regs.unwrap().rdi,
            "rsi" => self.captured_regs.unwrap().rsi,
            "rdx" => self.captured_regs.unwrap().rdx,
            "r8" => self.captured_regs.unwrap().r8,
            "r9" => self.captured_regs.unwrap().r9,
            "r10" => self.captured_regs.unwrap().r10,

            _ => 0,
        };

        for argument in self.arguments.iter() {
            if argument.arg_type.ends_with('*') {
                write!(
                    f,
                    "{}({}) = {:x}, ",
                    argument.name,
                    argument.arg_type,
                    get_reg_val(argument.register.as_ref())
                )?;
            } else {
                write!(
                    f,
                    "{}({}) = {}, ",
                    argument.name,
                    argument.arg_type,
                    get_reg_val(argument.register.as_ref())
                )?;
            }
        }

        write!(f, "] = {}", self.captured_regs.unwrap().rax)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SyscallTable {
    lookup_table: HashMap<u64, Syscall>,
}

impl SyscallTable {
    fn build() -> Self {
        // Process to lookup table.
        let t: Vec<Value> = serde_json::from_str(MEMORY_SYSCALLS).unwrap();
        let mut m = vec![];

        for v in t {
            let mut o = Syscall::deserialize(v).unwrap();
            o.is_memory_syscall = true;
            m.push(o);
        }

        m.append(
            &mut serde_json::from_str::<Vec<Value>>(FILE_SOCKET_SYSCALLS)
                .unwrap()
                .iter()
                .map(|s| Syscall::deserialize(s).unwrap())
                .collect::<Vec<_>>(),
        );

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
        let mut syscall = self
            .lookup_table
            .get(&_register_state.orig_rax)
            .map(|t| t.to_owned());

        if let Some(ref mut syscall) = syscall {
            syscall.captured_regs = Some(_register_state)
        }

        syscall
    }

    /// Number of currently loaded syscalls being traced.
    pub fn len(&self) -> usize {
        self.lookup_table.len()
    }
}

pub fn gen_syscalls_table() -> &'static SyscallTable {
    &__SYSCALLS
}

lazy_static::lazy_static! {
    pub static ref __SYSCALLS: SyscallTable = SyscallTable::build();
}
