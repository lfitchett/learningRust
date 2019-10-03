use chrono::*;
use std::*;
use sysinfo::*;

pub fn test_main() {
    let mut system = sysinfo::System::new();

    // First we update all information of our system struct.
    system.refresh_all();

    // // Now let's print every process' id and name:
    // for (pid, proc_) in system.get_process_list() {
    //     println!("{}:{} => status: {:?}", pid, proc_.name, proc_.status);
    // }

    // Then let's print the temperature of the different components:
    for component in system.get_components_list() {
        println!("{:?}", component);
    }

    // And then all disks' information:
    for disk in system.get_disks() {
        println!("{:?}", disk);
    }

    // And finally the RAM and SWAP information:
    println!("total memory: {} kB", system.get_total_memory());
    println!("used memory : {} kB", system.get_used_memory());
    println!("total swap  : {} kB", system.get_total_swap());
    println!("used swap   : {} kB", system.get_used_swap());

    system.refresh_all();
    system.refresh_all();
    system.refresh_all();


    for processor in system.get_processor_list() {
        println!("Processor {} usage: {}", processor.get_name(), processor.get_cpu_usage());
    }
}

//     use sha2::{Sha256, Digest};
//     use std::io;

// fn hash_file(path: &str) -> String {
//         let mut file = File::open(path).unwrap();
//         let mut hasher = Sha256::new();
//         io::copy(&mut file, &mut hasher).unwrap();
//         hasher.result().into_iter().map(|c| format!("{:02x?}", c)).collect()
//     }
