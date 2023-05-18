extern crate sys_info_rs;

fn main() {
    //println!("Hello, world!");
    //let mut hostname: String;
    //std::fs::File::open("/proc/sys/kernel/hostname").expect("Err reading proc").read_to_string(&mut hostname).expect("Err reading hostname");
    let info: SysInfo = SysInfo {
        loadavg: sys_info_rs::loadavg().expect("Err getting loadavg"),
        disk_info: sys_info_rs::disk_info().expect("Err getting disk info"),
        mem_info: sys_info_rs::mem_info().expect("Err getting mem_info"),
        boottime_sec: sys_info_rs::boottime()
            .expect("Err getting boottime")
            .tv_sec,
        hostname: sys_info_rs::hostname().expect("Err getting hostname)"),
        //hostname: hostname,
        time: std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Err getting unix time.")
            .as_secs(),
    };

    //println!("info {:#?} ", info);
    // println!("disk_info {:?} sec", sys_info_rs::disk_info().unwrap());
    // println!("loadavg {:?}", sys_info_rs::loadavg().unwrap());
    let info_json = serde_json::json!(info); //.expect("Err not json.");
    println!("{}", info_json);

    //println!("OS={}", std::env::consts::OS);
}

#[derive(Debug, serde::Serialize)]
struct SysInfo {
    loadavg: sys_info_rs::LoadAvg,
    disk_info: sys_info_rs::DiskInfo,
    mem_info: sys_info_rs::MemInfo,
    boottime_sec: i64,
    hostname: String,
    time: u64,
}
