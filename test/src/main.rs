
extern crate sys_info_rs;

use sys_info_rs::*;

fn main() {

    let load = LoadAvg().unwrap();
    println!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = mem_info().unwrap();
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
             mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
    println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
    #[cfg(not(target_os = "solaris"))] {
        let disk = disk_info().unwrap();
        println!("disk: total {} KB, free {} KB", disk.total, disk.free);
    }
    println!("hostname: {}", hostname().unwrap());
    #[cfg(not(target_os = "windows"))] {
        let t = boottime().unwrap();
        println!("boottime {} sec, {} usec", t.tv_sec, t.tv_usec);
    }
}
