use std::fs::File;
use std::error::Error;
use std::io::{BufWriter, Write, BufReader};
use sysinfo::{System, Process};
use configparser::ini::{Ini};
use serde_json::{Value, json, to_writer, from_reader};
use serde::{Deserialize};
use chrono::prelude::*;


struct DateInfo {
    date: NaiveDate,
    time: NaiveTime
}

#[derive(Debug, Deserialize)]
struct JsonData {
    date: String,
    time: String,
    fields: ProcessData,
}

#[derive(Debug, Deserialize)]
struct ProcessData {
    name: String,
    pid: String,
    memory: String,
    cpu: String,
}


const CONFIG_FILE: &str = "config/config.ini";


fn read_config() -> String {
    let mut config: Ini = Ini::new();
    let _map = config.load(CONFIG_FILE);
    let process_name: String = config.get("DEFAULT", "process_name").unwrap();
    return process_name;
}

fn get_date() -> DateInfo {
    let datetime: DateTime<Local> = Local::now();
    let date: NaiveDate = datetime.date_naive();
    let time: NaiveTime = datetime.time();
    let date_info = DateInfo{date, time};
    return date_info;
}

fn collect_json(data: &Process, datetime: DateInfo) -> Value {
    let json_data: Value = json!({
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

fn read_json(process_name: String) -> std::result::Result<JsonData, Box<dyn Error>> {
    let filename = format!("config/{}.json", process_name);
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let process = from_reader(reader)?;
    Ok(process)
}

fn write_to_file(data: Value, process_name: String) -> std::io::Result<()> {
    let filename: String = format!("config/{}.json", process_name);
    let file: File = File::create(filename)?;
    let mut writer: BufWriter<File> = BufWriter::new(file);
    to_writer(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}


fn main() -> std::result::Result<(), Box<dyn Error>>{
    let process_name: String = read_config();
    let sys: System = System::new_all();
    let test = read_json(process_name.clone())?;
    println!("{:?}", test);
    let mut processes = sys.processes_by_name(&process_name);
    if processes.next().is_some() {
        let processes = sys.processes_by_name(&process_name);
        for process in processes {
            let datetime: DateInfo = get_date();
            let data: Value = collect_json(process, datetime);
            // println!("Information saved!");
            println!("{:?}", data);
            write_to_file(data, process_name.clone()).expect("ERROR: info is not saved!");
        }
    } else {
        println!("No process named {process_name} found");
    }
    Ok(())
}