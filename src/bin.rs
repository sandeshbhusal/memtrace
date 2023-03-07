//! The binary runner interface
//!
//! Kills the Automatic address randomization on the binary
//! and makes it traceable from "traceme()" call.
//!
//! Forks over the main process, and returns a PID to the caller
//! that can then be traced like a normal PID.

// TODO: Initialize signals to cleanup so that we don't have orphan PIDs.
// LATER: Going to implement this later.
