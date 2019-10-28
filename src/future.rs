use byte_unit::*;
use chrono::*;
use number_prefix::*;
use serde_json;
use std::process::Command;
use std::*;
use sysinfo::{DiskExt, SystemExt};

pub fn test_main() {
    println!("{}", Utc::now().format("%F %T").to_string());
    let since = Utc::now();

    let inspect = Command::new("Get-WinEvent")
        .args(&[
            "-ea",
            "SilentlyContinue",
            "-FilterHashtable",
            &format!(
                "@{{ProviderName='iotedged';LogName='application';StartTime={}}}",
                since
            ),
            "Select",
            "TimeCreated",
            "Message",
            "Sort-Object",
            "@{Expression='TimeCreated';Descending=$false}",
            "Format-Table",
            "-AutoSize",
            "-Wrap",
        ])
        .output().unwrap();


}
