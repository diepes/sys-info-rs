//! #Introduction
//! This crate focuses on geting system information.
//!
//! For now it supports Linux, Mac OS X.
//! And now it can get information of cpu/memory/disk/load/hostname.
//!

extern crate nix;

extern crate serde;

use serde::{Deserialize, Serialize};

use std::fmt;
use std::fs::File;
use std::io::{self, Read};

use std::collections::HashMap;

/// System load average value.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadAvg {
    /// Average load within one minutes.
    pub one: f64,
    /// Average load within five minutes.
    pub five: f64,
    /// Average load within fifteen minutes.
    pub fifteen: f64,
}

/// System memory information.
#[derive(Debug, Deserialize, Serialize)]
pub struct MemInfo {
    /// Total physical memory.
    pub total: u64,
    pub free: u64,
    pub avail: u64,

    pub buffers: u64,
    pub cached: u64,

    /// Total swap memory.
    pub swap_total: u64,
    pub swap_free: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeVal {
    pub tv_sec: u64,
    pub tv_usec: u64,
}
/// Error types
#[derive(Debug)]
pub enum Error {
    UnsupportedSystem,
    ExecFailed(io::Error),
    IO(io::Error),
    SystemTime(std::time::SystemTimeError),
    General(String),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnsupportedSystem => write!(fmt, "System is not supported"),
            ExecFailed(ref e) => write!(fmt, "Execution failed: {}", e),
            IO(ref e) => write!(fmt, "IO error: {}", e),
            SystemTime(ref e) => write!(fmt, "System time error: {}", e),
            General(ref e) => write!(fmt, "Error: {}", e),
            Unknown => write!(fmt, "An unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnsupportedSystem => "unsupported system",
            ExecFailed(_) => "execution failed",
            IO(_) => "io error",
            SystemTime(_) => "system time",
            General(_) => "general error",
            Unknown => "unknown error",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        use self::Error::*;
        match *self {
            UnsupportedSystem => None,
            ExecFailed(ref e) => Some(e),
            IO(ref e) => Some(e),
            SystemTime(ref e) => Some(e),
            General(_) => None,
            Unknown => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

//#impl From<std::time::SystemTimeError> for Error {
//#    fn from(e: std::time::SystemTimeError) -> Error {
//#        Error::SystemTime(e)
//#    }
//#}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Error {
        Error::General(e.to_string())
    }
}

/// Get system load average value.
///
/// Notice, on windows, one/five/fifteen of the LoadAvg returned are the current load.
pub fn loadavg() -> Result<LoadAvg, Error> {
    {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        let loads = s
            .trim()
            //.split(' ')
            .split_whitespace()
            .take(3)
            .map(|val| val.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        Ok(LoadAvg {
            one: loads[0],
            five: loads[1],
            fifteen: loads[2],
        })
    }
}

/// Get current processes quantity.
pub fn proc_total() -> Result<u64, Error> {
    {
        let mut s = String::new();
        File::open("/proc/loadavg")?.read_to_string(&mut s)?;
        s.split_whitespace()
            .nth(3)
            .and_then(|val| val.split('/').last())
            .and_then(|val| val.parse::<u64>().ok())
            .ok_or(Error::Unknown)
    }
}

/// Get memory information.
///
/// On Mac OS X and Windows, the buffers and cached variables of the MemInfo returned are zero.
pub fn mem_info() -> Result<MemInfo, Error> {
    {
        let mut s = String::new();
        File::open("/proc/meminfo")?.read_to_string(&mut s)?;
        let mut meminfo_hashmap = HashMap::new();
        for line in s.lines() {
            let mut split_line = line.split_whitespace();
            let label = split_line.next();
            let value = split_line.next();
            if value.is_some() && label.is_some() {
                let label = label.unwrap().split(':').nth(0).ok_or(Error::Unknown)?;
                let value = value.unwrap().parse::<u64>().ok().ok_or(Error::Unknown)?;
                meminfo_hashmap.insert(label, value);
            }
        }
        let total = *meminfo_hashmap.get("MemTotal").ok_or(Error::Unknown)?;
        let free = *meminfo_hashmap.get("MemFree").ok_or(Error::Unknown)?;
        let buffers = *meminfo_hashmap.get("Buffers").ok_or(Error::Unknown)?;
        let cached = *meminfo_hashmap.get("Cached").ok_or(Error::Unknown)?;
        let avail = meminfo_hashmap
            .get("MemAvailable")
            .copied()
            //.map(|v| v.clone())
            .or_else(|| {
                let sreclaimable = *meminfo_hashmap.get("SReclaimable")?;
                let shmem = *meminfo_hashmap.get("Shmem")?;
                Some(free + buffers + cached + sreclaimable - shmem)
            })
            .ok_or(Error::Unknown)?;
        let swap_total = *meminfo_hashmap.get("SwapTotal").ok_or(Error::Unknown)?;
        let swap_free = *meminfo_hashmap.get("SwapFree").ok_or(Error::Unknown)?;
        Ok(MemInfo {
            total,
            free,
            avail,
            buffers,
            cached,
            swap_total,
            swap_free,
        })
    }
}

/// Disk information.
#[derive(Debug, Deserialize, Serialize)]
pub struct DiskInfo {
    //pub total: u64,
    //pub free: u64,
    pub name: String,
}
/// Get disk information.
///
/// Notice, it just calculate current disk on Windows.
/// PES use du -P
pub fn disk_info_filtered() -> Result<Vec<DiskUsage>, Error> {
    use std::process::Command;
    let output = Command::new("/bin/df")
        .arg("-Pk") //# -P Posix format, -k block-size=1K
        .output() // wait collect output
        .expect("failed to execute /bin/df");

    assert!(output.status.success());
    //println!("output.stdout: {}", String::from_utf8(&output.stdout).unwrap());
    let a: Vec<DiskUsage> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|s| !{
            ["Filesystem ", "tmpfs ", "shm ", "grpcfuse "]
                .iter()
                .any(|&x| s.contains(x))
        })
        .map(df_split)
        .collect();
    Ok(a)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiskUsage {
    path: String,
    mb_used: u64,
    mb_available: u64,
    used_pct: u8,
    fs: String,
}
fn df_split(s: &str) -> DiskUsage {
    //vec![String,"a".to_string(),"b".to_string()]
    //String { vec!["a".to_string(),"b".to_string()] }
    let split: Vec<&str> = s
        .split_whitespace()
        //.map(|v| v.to_string())
        .collect();

    DiskUsage {
        path: split[5].into(),
        mb_used: split[2]
            .parse::<u64>()
            .expect("df 3rd column not a number ?")
            / 1000,
        mb_available: split[3]
            .parse::<u64>()
            .expect("df 4th column non a number ?")
            / 1000,
        used_pct: split[4]
            .to_string()
            .strip_suffix("%")
            .expect("df 5th column no % sign ?")
            .parse::<u8>()
            .expect("df 5th column non number% ?"),
        fs: split[0].into(),
    }
}

/// Get hostname.
pub fn hostname() -> Result<String, Error> {
    use std::process::Command;
    Command::new("hostname")
        .output()
        .map_err(Error::ExecFailed)
        .map(|output| String::from_utf8(output.stdout).unwrap().trim().to_string())
}

/// Get system boottime
pub fn uptime() -> Result<u64, Error> {
    let mut s = String::new();
    //# Get 2 numbers system uptime and system idle time
    File::open("/proc/uptime")?.read_to_string(&mut s)?;
    let secs = s
        .trim()
        .split(' ')
        .take(1)
        .map(|val| val.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    Ok(secs[0] as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_loadavg() {
        let load = loadavg().unwrap();
        println!("loadavg(): {:?}", load);
    }

    #[test]
    pub fn test_proc_total() {
        let procs = proc_total().unwrap();
        assert!(procs > 0);
        println!("proc_total(): {}", procs);
    }

    #[test]
    pub fn test_mem_info() {
        let mem = mem_info().unwrap();
        assert!(mem.total > 0);
        println!("mem_info(): {:?}", mem);
    }

    #[test]
    pub fn test_hostname() {
        let host = hostname().unwrap();
        assert!(host.len() > 0);
        println!("hostname(): {}", host);
    }

    #[test]
    pub fn test_uptime() {
        let bt = uptime().unwrap();
        println!("uptime(): {}", bt);
        assert!(bt > 0);
    }
}
