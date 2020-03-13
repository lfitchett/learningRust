use byte_unit::*;
use chrono::*;
use std::convert::*;
use std::time::*;
use std::*;
use sysinfo::*;

pub fn test_main() {
    println!("{:#?}", SystemInfo::new());
}

#[derive(Clone, Debug, Default)]
struct SystemInfo {
    host_uptime: i64,
    process_uptime: u64,

    used_ram: String,
    total_ram: String,
    used_swap: String,
    total_swap: String,

    disks: Vec<DiskInfo>,
}

impl SystemInfo {
    fn new() -> Self {
        {
            let mut system = sysinfo::System::new();
            system.refresh_all();

            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - system.get_process_list()[&process::id().try_into().unwrap()].start_time();

            SystemInfo {
                host_uptime: uptime_lib::get().unwrap().num_seconds(),
                process_uptime: start_time,

                total_ram: pretty_kbyte(system.get_total_memory()),
                used_ram: pretty_kbyte(system.get_used_memory()),
                total_swap: pretty_kbyte(system.get_total_swap()),
                used_swap: pretty_kbyte(system.get_used_swap()),

                disks: system.get_disks().iter().map(DiskInfo::new).collect(),
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct DiskInfo {
    name: String,
    percent_free: String,
    available_space: String,
    total_space: String,
    file_system: String,
    file_type: String,
}

impl DiskInfo {
    fn new<T>(disk: &T) -> Self
    where
        T: DiskExt,
    {
        let available_space = disk.get_available_space();
        let total_space = disk.get_total_space();
        #[allow(clippy::cast_precision_loss)]
        let percent_free = format!(
            "{:.1}%",
            available_space as f64 / total_space as f64 * 100.0
        );

        DiskInfo {
            name: disk.get_name().to_string_lossy().into_owned(),
            percent_free,
            available_space: Byte::from_bytes(u128::from(available_space))
                .get_appropriate_unit(true)
                .format(2),
            total_space: Byte::from_bytes(u128::from(total_space))
                .get_appropriate_unit(true)
                .format(2),
            file_system: String::from_utf8_lossy(disk.get_file_system()).into_owned(),
            file_type: format!("{:?}", disk.get_type()),
        }
    }
}

fn pretty_kbyte(bytes: u64) -> String {
    #[allow(clippy::cast_precision_loss)]
    match Byte::from_unit(bytes as f64, ByteUnit::KiB) {
        Ok(b) => b.get_appropriate_unit(true).format(2),
        Err(err) => format!("could not parse bytes value: {:?}", err),
    }
}
