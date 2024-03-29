use clap::{Arg, ArgGroup, Command};
use nix::{
    libc::exit,
    sys::{
        ptrace,
        ptrace::attach,
        wait::{waitpid, WaitStatus},
    },
    unistd::Pid,
};

mod bin;
mod fd;
mod pid;
mod syscalls;
mod procinfo;

#[cfg(target_os = "linux")]
fn main() {
    // Initialize logging.

    use crate::syscalls::gen_syscalls_table;
    env_logger::init();

    let all_syscalls = gen_syscalls_table().clone();

    // Check for binary arguments.
    let args = Command::new("memtrace").args(&[
        Arg::new("pid").short('p').long("pid").help("The pid to trace into."),
        Arg::new("bin").help("The binary to run and trace (will be executed as a child. If it escapes to init, pls kill")
    ]).group(ArgGroup::new("exclusive_p_b").args(["pid", "bin"]).required(true)).get_matches();

    println!(
        r#"
.___  ___.  _______ .___  ___. .___________..______          ___       ______  _______ 
|   \/   | |   ____||   \/   | |           ||   _  \        /   \     /      ||   ____|
|  \  /  | |  |__   |  \  /  | `---|  |----`|  |_)  |      /  ^  \   |  ,----'|  |__   
|  |\/|  | |   __|  |  |\/|  |     |  |     |      /      /  /_\  \  |  |     |   __|  
|  |  |  | |  |____ |  |  |  |     |  |     |  |\  \----./  _____  \ |  `----.|  |____ 
|__|  |__| |_______||__|  |__|     |__|     | _| `._____/__/     \__\ \______||_______|
    "#
    );

    log::info!("Loaded {} syscalls list.", all_syscalls.len());
    if let Some(pid) = args.get_one::<String>("pid") {
        // Pid based workflow.
        let pid = pid
            .parse::<i32>()
            .map_err(|_| {
                log::error!(
                    "{} could not be parsed as a valid PID (only numbers supported). Exiting...",
                    pid
                );
                unsafe { exit(0) }
            })
            .unwrap(); // Owned pid.

        let pid = Pid::from_raw(pid);

        // Attach ptrace to the pid.
        match attach(pid) {
            Ok(_) => {
                log::info!("Attached to pid {}", pid);
            }
            Err(e) => log::error!("Could not attach to pid {} due to an error: {}", pid, e),
        }

        // Wait for PID and if successful, run a syscall trace on it.
        match waitpid(pid, None) {
            Ok(status) => match status {
                WaitStatus::Exited(_, _) => {
                    log::error!("The PID {} has already exited.", pid);
                }

                _ => {
                    // Run a syscall trace on it.
                    ptrace::syscall(pid, None).unwrap();
                    _ = waitpid(pid, None);
                    let mut entry_syscall = false;
                    // While the process does not exit, run a ptrace on it.
                    loop {
                        let regs = ptrace::getregs(pid).unwrap();
                        if entry_syscall {
                            let s = all_syscalls.lookup(regs);
                            if let Some(s) = s {
                                log::debug!("{}", s);
                            }
                        }

                        // Resubmit for tracing.
                        ptrace::syscall(pid, None).unwrap();
                        entry_syscall = !entry_syscall;
                        _ = waitpid(pid, None);
                    }
                }
            },
            Err(e) => {
                log::error!("Cannot proceed with trace due to an error: {}", e);
            }
        }
    } else {
        // Binary based workflow.
        // if let Some(command) = args.get_one::<String>("bin") {
        //     let command = command.split(' ').collect::<Vec<&str>>();
        //     let process = std::process::Command::new(command.get(0).unwrap());
        // } else {
        //     panic!("No binary argument passed.")
        // }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    env_logger::init();
    log::error!("Sorry, memtrace does not support windows :(");
}
