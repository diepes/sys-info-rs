extern crate sys_info_rs;

fn main() {
    println!("Hello, world!");
    let info: sysinfo = sysinfo {
        loadavg: sys_info_rs::loadavg().expect("Err getting loadavg"),
        disk_info: sys_info_rs::disk_info().expect("Err getting disk info"),
        mem_info: sys_info_rs::mem_info().expect("Err getting mem_info"),
        boottime_sec: sys_info_rs::boottime()
            .expect("Err getting boottime")
            .tv_sec,
        hostname: sys_info_rs::hostname().expect("Err getting hostname)"),
    };

    println!("info {:#?} ", info);
    // println!("disk_info {:?} sec", sys_info_rs::disk_info().unwrap());
    // println!("loadavg {:?}", sys_info_rs::loadavg().unwrap());
}

#[derive(Debug, serde::Deserialize)]
struct sysinfo {
    loadavg: sys_info_rs::LoadAvg,
    disk_info: sys_info_rs::DiskInfo,
    mem_info: sys_info_rs::MemInfo,
    boottime_sec: i64,
    hostname: String,
}
