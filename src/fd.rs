//! This module is responsible for reading into the current
//! file descriptors being held by the process.
//!
//! Given a process ID, all file descriptors currently in use should
//! be enumerated, and categorised based on "socket" and "file" (nothing else supported for now).

use std::{
    collections::HashMap,
    ffi::OsString,
    fs::{read_dir, DirEntry, FileType},
    io,
};

use nix::unistd::Pid;

pub enum FDType {
    SOCKET,
    FILE,

    OTHER,
}

pub type FDList = HashMap<u32, FDType>;

fn get_all_fds(pid: Pid) -> Result<Vec<(Option<u32>, FileType)>, Box<dyn std::error::Error>> {
    let dir_result = read_dir(format!("/proc/{pid}/fd/"))
        .expect("Could not read proc dir")
        .map(|e| e.ok())
        .filter_map(|e| e)
        .map(|e| (e.file_name(), e.file_type().ok()))
        .filter_map(|e| {
            if e.1.is_some() {
                Some((e.0.to_str()?.parse::<u32>().ok(), e.1?))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(dir_result)
}

#[test]
fn test_fdlist() {
    dbg!(get_all_fds(Pid::from_raw(26022)).unwrap());
}
