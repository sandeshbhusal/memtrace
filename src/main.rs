// use std::{error::Error, os::unix::process::CommandExt, process::Command};

use clap::parser;
// use nix::{
//     libc::getpid,
//     sys::{
//         ptrace::{self, traceme},
//         wait::waitpid,
//     },
//     unistd::{fork, ForkResult},
// };

struct PidArgument {}

#[derive(Debug, clap::Parser)]
struct BinArguments {
    binary: String,
}

struct Arguments {}

fn main() {
    // Initialize logging.
    env_logger::init();

    // Check for binary arguments.
}

// #[cfg(target_os = "linux")]
// fn main() -> Result<(), Box<dyn Error>> {
//     // Trace an external program.

//     use std::{env, fs::File, io::Read};

//     use nix::unistd::Pid;
//     let args = env::args().collect::<Vec<String>>();
//     if args.len() == 1 {
//         // continue in self-contained mode.
//         return fork_and_run();
//     } else {
//         // take the first number as pid.
//         let pid = args.get(1);
//         if pid.is_none() {
//             panic!("Wut?");
//         }

//         let pid = pid.unwrap();
//         let pid = Pid::from_raw(
//             pid.parse::<i32>()
//                 .expect("That does not look like a PID to me."),
//         );
//         // Try to read proc filesystem to find all PID sockets.
//         let filepath = format!("/proc/{}/fd", pid);
//         let mut procfile = File::open(filepath).unwrap();
//         let mut buf = String::new();
//         procfile.read_to_string(&mut buf).unwrap();
//         for line in buf.lines() {
//             // Parse line.
//         }
//     }

//     Ok(())
// }
// fn fork_and_run() -> Result<(), Box<dyn Error>> {
//     let mut calls_and_locs = std::collections::HashMap::new();
//     match unsafe { fork() } {
//         Ok(ForkResult::Parent { child: pid }) => {
//             let self_pid = unsafe { getpid() };
//             println!(
//                 "Child created with PID: {} for parent: {}",
//                 i32::from(pid),
//                 self_pid
//             );

//             let child_pid = pid;

//             waitpid(child_pid, None)?;

//             let mut is_syscall_entry = 0;
//             loop {
//                 ptrace::syscall(child_pid, None)?;
//                 _ = waitpid(child_pid, None)?;
//                 let regs = nix::sys::ptrace::getregs(pid).unwrap().orig_rax;
//                 let ripr = nix::sys::ptrace::getregs(pid).unwrap().rip;
//                 is_syscall_entry += 1;
//                 if is_syscall_entry % 2 == 0 {
//                     calls_and_locs.insert(regs, ripr);
//                     // Also print the current stack trace for the process.
//                     // let mut stackfile = File::open(format!("/proc/{}/stack", child_pid)).unwrap();
//                     // let mut buf = String::new();
//                     // stackfile.read_to_string(&mut buf).unwrap();
//                     // println!("Stack trace: \n{}", buf);
//                 }
//             }
//         }

//         Ok(ForkResult::Child) => {
//             // Set current process to be traceable.
//             let mut command = Command::new("cat");
//             command.arg("/tmp/outz");
//             unsafe { command.pre_exec(|| traceme().map_err(|e| e.into())) };
//             command.exec();
//             Ok(())
//         }

//         Err(_) => panic!("Could nnot fork."),
//     }
// }
// #[cfg(target_os = "windows")]
// fn main() {
//     println!("Sorry, I do not support windows :(")
// }
