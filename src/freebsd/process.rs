// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DiskUsage, Pid, ProcessExt, ProcessStatus, Signal};
use std::collections::HashMap;

use std::path::Path;

// FIXME: to be removed once freebsd 0.2.108 has been released.
pub const SIDL: ::c_char = 1;
pub const SRUN: ::c_char = 2;
pub const SSLEEP: ::c_char = 3;
pub const SSTOP: ::c_char = 4;
pub const SZOMB: ::c_char = 5;
pub const SWAIT: ::c_char = 6;
pub const SLOCK: ::c_char = 7;

#[doc(hidden)]
impl From<libc::c_char> for ProcessStatus {
    fn from(status: libc::c_char) -> ProcessStatus {
        match status {
            1 => ProcessStatus::Idle,
            2 => ProcessStatus::Run,
            3 => ProcessStatus::Sleep,
            4 => ProcessStatus::Stop,
            5 => ProcessStatus::Zombie,
            x => ProcessStatus::Unknown(x),
        }
    }
}

/// Struct containing a process' information.
#[derive(Clone)]
pub struct Process {
    pid: Pid,
    parent: Option<Pid>,
    status: ProcessStatus,
}

impl ProcessExt for Process {
    fn new(pid: Pid, parent: Option<Pid>, _start_time: u64) -> Process {
        Process { pid, parent }
    }

    fn kill(&self, _signal: Signal) -> bool {
        false
    }

    fn name(&self) -> &str {
        ""
    }

    fn cmd(&self) -> &[String] {
        &[]
    }

    fn exe(&self) -> &Path {
        &Path::new("")
    }

    fn pid(&self) -> Pid {
        self.pid
    }

    fn environ(&self) -> &[String] {
        &[]
    }

    fn cwd(&self) -> &Path {
        &Path::new("")
    }

    fn root(&self) -> &Path {
        &Path::new("")
    }

    fn memory(&self) -> u64 {
        0
    }

    fn virtual_memory(&self) -> u64 {
        0
    }

    fn parent(&self) -> Option<Pid> {
        self.parent
    }

    fn status(&self) -> ProcessStatus {
        self.status
    }

    fn start_time(&self) -> u64 {
        0
    }

    fn cpu_usage(&self) -> f32 {
        0.0
    }

    fn disk_usage(&self) -> DiskUsage {
        DiskUsage::default()
    }
}

pub fn get_process_data(kproc: &libc::kinfo_proc, proc_list: &mut HashMap<Pid, Process>) -> Option<Process> {
    if let Some(proc_) = proc_list.get_mut(&kproc.ki_pid) {
        ;
    } else {
        let mut proc_ = Process {
            pid: kproc.ki_pid,
            parent: if kproc->ki_ppid != 0 { Some(kproc->ki_ppid) } else { None },
        };
    }
}
