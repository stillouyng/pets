use std::fs::File;
use std::result::Result as StandardResult;
use std::io::{BufWriter, Write, BufReader};
use sysinfo::{System, Process};
use configparser::ini::{Ini};
use serde_json::{Value, to_writer, from_reader, to_value, to_writer_pretty};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct DateInfo {
    date: String,
    time: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct JsonData {
    datetime: DateInfo,
    fields: ProcessData,
    counter: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ProcessData {
    name: String,
    pid: Vec<String>,
    memory: String,
    cpu: String,
}


const CONFIG_FILE: &str = "config/config.ini";


fn read_config(config_name: &str) -> String {
    let mut config: Ini = Ini::new();
    let _map = config.load(CONFIG_FILE);
    let process_name: String = config.get("DEFAULT", config_name).unwrap();
    return process_name;
}

fn get_date() -> DateInfo {
    let datetime: DateTime<Local> = Local::now();
    let date: String = datetime.date_naive().to_string();
    let time: String = datetime.time().to_string();
    let date_info = DateInfo{date, time};
    return date_info;
}

fn collect_json(data: &Process, datetime: DateInfo) -> JsonData {
    let json_data: JsonData = JsonData {
        datetime,
        fields: ProcessData {
            pid: vec![format!("{}", data.pid())],
            name: data.name().to_string(),
            memory: format!("{} MB", data.memory() / (2_u64.pow(20))),
            cpu: format!("{0:.2}%", data.cpu_usage()),
        },
        counter: 1
    };
    return json_data
}

fn open_file(process_name: String) -> std::io::Result<File> {
    let filename: String = format!("config/{process_name}.json");
    let json_file_result = File::open(filename.clone());
    return json_file_result
}

fn read_json(process_name: String) -> StandardResult<JsonData, String> {
    let file_result = open_file(process_name);
    return match file_result {
        Ok(file) => {
            let reader = BufReader::new(file);
            let process_result = from_reader(reader);
            return match process_result {
                Ok(process) => Ok(process),
                Err(error) => Err(error.to_string())
            }
        }
        Err(error) => {
            Err(error.to_string())
        }
    }
}

fn write_to_file(data: JsonData, process_name: String, directory: &str, is_pretty: bool) -> std::io::Result<()> {
    let filename: String = format!("{directory}/{process_name}.json");
    println!("{filename}");
    let file: File = File::create(filename)?;
    let mut writer: BufWriter<File> = BufWriter::new(file);
    let data_valued: Value = to_value(data)?;
    if is_pretty {
        to_writer_pretty(&mut writer, &data_valued)?;
    } else {
        to_writer(&mut writer, &data_valued)?;
    }
    writer.flush()?;
    Ok(())
}

fn count_and_save_data(data: JsonData, process_name: String) {
    let saved_result = read_json(process_name.clone());
    match saved_result {
        Ok(mut json_data) => {
            println!("Read JsonData: {json_data:?}");
            let mut new_data: JsonData = data.clone();
            if json_data.datetime.date == new_data.datetime.date {
                new_data.counter = json_data.counter + 1;
                new_data.fields.pid.append(&mut json_data.fields.pid);
                new_data.fields.memory = format!("{} MB",
                    (
                        data.fields.memory.split(" ")
                            .next().unwrap()
                            .parse::<u16>().unwrap()
                        + json_data.fields.memory.split(" ")
                            .next().unwrap()
                            .parse::<u16>().unwrap()
                    ).to_string());
                new_data.fields.cpu = format!("{:.4}%",
                    (data.fields.cpu.replace("%", "").as_str()
                            .parse::<f64>().unwrap()
                        + json_data.fields.cpu.replace("%", "").as_str()
                            .parse::<f64>().unwrap()
                    ).to_string()
                );
            }
            write_to_file(new_data.clone(), process_name.clone(), "config", false).expect("ERROR: info is not saved!");
        }
        Err(error_string) => {
            write_to_file(data.clone(), process_name.clone(), "config", false).expect("ERROR: info is not saved!");
            println!("Error while reading file: {error_string}")
        }
    }
}

fn save_result(mut filename: String) -> StandardResult<bool, String> {
    let save_directory: String = read_config("save_directory");
    let saved_result: JsonData = read_json(filename.clone())?;
    let counted_data: JsonData = JsonData {
        datetime: DateInfo {
            date: saved_result.datetime.date,
            time: saved_result.datetime.time,
        },
        fields: ProcessData {
            cpu: format!("{:.4}%",
                (
                    saved_result.fields.cpu.replace("%", "").as_str()
                        .parse::<f64>().unwrap()
                        %
                        saved_result.counter as f64
                ).to_string()
            ),
            memory: format!("{} MB",
                (
                    saved_result.fields.memory.split(" ")
                        .next().unwrap()
                        .parse::<u16>().unwrap()
                        %
                        saved_result.counter
                ).to_string()
            ),
            pid: saved_result.fields.pid,
            name: saved_result.fields.name,
        },
        counter: saved_result.counter
    };
    filename = format!(
        "{filename}_{0}",
        counted_data.datetime.date.replace(":", "-")
    );
    write_to_file(counted_data, filename, &*save_directory, true).expect("ERROR: can't save the result");
    Ok(true)
}

fn main() -> () {
    let process_name: String = read_config("process_name");
    let sys: System = System::new_all();
    let mut processes = sys.processes_by_name(&process_name);
    if processes.next().is_some() {
        let processes = sys.processes_by_name(&process_name);
        let mut iter_count: i32 = 1;
        for process in processes {
            let datetime: DateInfo = get_date();
            let data: JsonData = collect_json(process, datetime);
            println!("Get data: {:?}", data);
            count_and_save_data(data, process_name.clone());
            println!("Cycle No. {iter_count} ended.");
            iter_count += 1;
            println!("––––––––––––––––––––");
        }
        save_result(process_name.clone()).expect("ERROR: While saving result");
    } else {
        println!("No process named {process_name} found");
    }
}
