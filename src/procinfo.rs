//! This module contains some utilities for working with `proc` filesystem
//! in Linux. It parses the procfile, and returns data based on what was read
//! from different files in the proc fs.

use std::{fs::File, io::Read};

use nix::unistd::Pid;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::anychar,
    sequence::delimited,
};

/// The MemInfo is extracted from `/proc/<>/maps`.
pub struct StatInfo {
    pid: Pid,
    name: String,
    state: ProcessState,
    ppid: Pid,
    pgrp: usize,
    sess_id: usize,
    tty_nr: usize,
    tpgid: usize,
    utime: usize,
    stime: usize,
    cutime: usize,
    cstime: usize,
    prio: usize,
    nice: usize,
    num_threads: usize,
    starttime: usize,
    vsize: usize,
    rss: usize,
    rsslim: usize,
}

pub enum ProcessState {
    Running,
    Sleeping,
    UintSleep,
    Zombie,
    Stopped,
    TracingStop,
    Paging,
}

#[derive(Debug)]
pub struct ProcessStat {
    pub pid: i32,
    pub name: String,
    pub state: char,
    pub ppid: i32,
    pub pgrp: i32,
    pub session: i32,
    pub tty_nr: i32,
    pub tpgid: i32,
    pub utime: u64,
    pub stime: u64,
    pub cutime: u64,
    pub cstime: u64,
    pub priority: i32,
    pub nice: i32,
    pub num_threads: i32,
    pub starttime: u64,
    pub vsize: u64,
    pub rss: i64,
    pub rsslim: u64,
}

fn parse_stat(input: &str) -> nom::IResult<&str, ProcessStat> {
    let (input, pid) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;

    let (input, name) = delimited(tag("("), take_until(")"), tag(") "))(input)?;

    let (input, state) = anychar(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, ppid) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, pgrp) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, session) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, tty_nr) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, tpgid) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, utime) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, stime) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, cutime) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, cstime) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, priority) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, nice) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, num_threads) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i32>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, starttime) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, vsize) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    let (input, _) = tag(" ")(input)?;
    let (input, rss) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i64>)(input)?;

    // let (input, rss) = nom::combinator::map_res(
    //     nom::combinator::opt(nom::character::complete::char(' ')),
    //     |opt_space| {
    //         nom::combinator::map_res(nom::character::complete::digit1, str::parse::<i64>)(opt_space)
    //     },
    // )(input)?;

    let (input, rsslim) =
        nom::combinator::map_res(nom::character::complete::digit1, str::parse::<u64>)(input)?;

    Ok((
        input,
        ProcessStat {
            pid,
            name: name.to_owned(),
            state,
            ppid,
            pgrp,
            session,
            tty_nr,
            tpgid,
            utime,
            stime,
            cutime,
            cstime,
            priority,
            nice,
            num_threads,
            starttime,
            vsize,
            rss,
            rsslim,
        },
    ))
}

#[test]
fn test_stat_parser() {
    let mut f = File::open("/proc/14863/stat").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let stat = parse_stat(buf.as_str()).expect("Error parsing stat.");
}
