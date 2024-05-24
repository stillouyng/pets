use std::fs::File;
use std::io::{BufWriter, Write};
use sysinfo::{System, Process};
use configparser::ini::{Ini};
use serde_json::{Value, json, to_writer_pretty};
use chrono::prelude::*;


struct DateInfo {
    date: NaiveDate,
    time: NaiveTime
}

const CONFIG_FILE: &str = "config/config.ini";


fn read_config() -> String {
    let mut config: Ini = Ini::new();
    let _map = config.load(CONFIG_FILE);
    let process_name: String = config.get("DEFAULT", "process_name").unwrap();
    return process_name;
}

fn get_date() -> DateInfo {
    let datetime = Local::now();
    let date = datetime.date_naive();
    let time = datetime.time();
    let date_info = DateInfo{date, time};
    return date_info;
}

fn collect_json(data: &Process, datetime: DateInfo) -> Value {
    let json_data = json!({
        "date": format!("{}", datetime.date),
        "time": format!("{}", datetime.time),
        "fields": {
            "pid": format!("{}", data.pid()),
            "name": data.name(),
            "memory": format!("{} MB", data.memory() / (2_u64.pow(20))),
            "cpu": format!("{0:.2}%", data.cpu_usage()),
        }
    });
    return json_data
}

fn write_to_file(data: Value, process_name: String) -> std::io::Result<()> {
    let filename = format!("config/{}.json", process_name);
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    to_writer_pretty(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}


fn main() {
    let process_name = read_config();
    let sys: System = System::new_all();
    let mut processes = sys.processes_by_name(&process_name);
    if processes.next().is_some() {
        let processes = sys.processes_by_name(&process_name);
        for process in processes {
            let datetime: DateInfo = get_date();
            let data = collect_json(process, datetime);
            println!("Information saved!");
            write_to_file(data, process_name.clone()).expect("ERROR: info is not saved!");
        }
    } else {
        println!("No process named {process_name} found");
    }
}