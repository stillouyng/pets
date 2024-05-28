use std::fmt::Error;use std::fs::{File, create_dir, read_dir, remove_file};use std::result::Result as StandardResult;use std::io::{BufWriter, Write, BufReader};use std::path::Path;use sysinfo::{System, Process};use configparser::ini::{Ini};use serde_json::{Value, to_writer, from_reader, to_value, to_writer_pretty};use serde::{Deserialize, Serialize};use chrono::prelude::*;#[derive(Debug, Deserialize, Serialize, Clone)]struct DateInfo {    date: String,    time: String}#[derive(Debug, Deserialize, Serialize, Clone)]struct JsonData {    datetime: DateInfo,    fields: ProcessData,    counter: u16,}#[derive(Debug, Deserialize, Serialize, Clone)]struct ProcessData {    name: String,    pid: Vec<String>,    memory: String,    cpu: String,}#[derive(Debug)]enum FilenameData {    String(String),    Vector(Vec<String>)}const CONFIG_FILE: &str = "config/config.ini";fn read_config(config_name: &str) -> String {    let mut config: Ini = Ini::new();    let _map = config.load(CONFIG_FILE);    let process_name: String = config.get("DEFAULT", config_name).unwrap();    return process_name;}fn get_date() -> DateInfo {    let datetime: DateTime<Local> = Local::now();    let date: String = datetime.date_naive().to_string();    let time: String = datetime.time().to_string();    let date_info = DateInfo{date, time};    return date_info;}fn is_temp_directory_exist() -> StandardResult<bool, String> {    if Path::new("config/temp").exists() {        Ok(true)    } else {        create_dir("config/temp").expect("ERROR: while creating temp dir");        Ok(true)    }}fn count_cpu(value_1: String, value_2: String) -> String {     let result: String = format!("{:.4}%",        (            value_1.replace("%", "").as_str()                .parse::<f64>().unwrap()            +            value_2.replace("%", "").as_str()                .parse::<f64>().unwrap()        ).to_owned()    );    return result}fn count_memory(value_1: String, value_2: String) -> String {    let result: String = format!("{} MB",        (            value_1.split(" ")                .next().unwrap()                .parse::<u16>().unwrap()            + value_2.split(" ")                .next().unwrap()                .parse::<u16>().unwrap()        ).to_owned()    );    return result}fn count_files_in_temp() -> Vec<String> {    let files: Vec<String> = read_dir("config/temp/")        .unwrap()        .filter_map(|e| e.ok())        .map(|e| e.path().to_string_lossy().into_owned())        .collect::<Vec<_>>();    return files}fn collect_json(data: &Process, datetime: DateInfo) -> JsonData {    let json_data: JsonData = JsonData {        datetime,        fields: ProcessData {            pid: vec![format!("{}", data.pid())],            name: data.name().to_owned(),            memory: format!("{} MB", data.memory() / (2_u64.pow(20))),            cpu: format!("{0:.2}%", data.cpu_usage()),        },        counter: 1    };    return json_data}fn open_file(process_name: String) -> std::io::Result<File> {    let filename: String = format!("config/temp/{process_name}.json");    let json_file_result = File::open(filename.clone());    return json_file_result}fn read_json(process_name: String) -> StandardResult<JsonData, String> {    let file_result = open_file(process_name);    return match file_result {        Ok(file) => {            let reader = BufReader::new(file);            let process_result = from_reader(reader);            return match process_result {                Ok(process) => Ok(process),                Err(error) => Err(error.to_string())            }        }        Err(error) => {            Err(error.to_string())        }    }}fn write_to_file(data: JsonData, process_name: String, directory: &str, is_pretty: bool) -> std::io::Result<()> {    let filename: String;    if is_pretty {        let changed_filename: String = format!("{directory}/{process_name}.json");        filename = changed_filename;    } else {        let changed_filename: String = format!("{directory}/temp/{process_name}.json");        filename = changed_filename;    }    let file: File = File::create(filename)?;    let mut writer: BufWriter<File> = BufWriter::new(file);    let data_valued: Value = to_value(data)?;    if is_pretty {        to_writer_pretty(&mut writer, &data_valued)?;    } else {        to_writer(&mut writer, &data_valued)?;    }    writer.flush()?;    Ok(())}fn count_and_save_data(data: JsonData, process_name: String) {    let saved_result = read_json(process_name.clone());    match saved_result {        Ok(mut json_data) => {            println!("Read JsonData: {json_data:?}");            let mut new_data: JsonData = data.clone();            if json_data.datetime.date == new_data.datetime.date {                new_data.counter = json_data.counter + 1;                new_data.fields.pid.append(&mut json_data.fields.pid);                new_data.fields.memory = count_memory(data.fields.memory, json_data.fields.memory);                new_data.fields.cpu = count_cpu(data.fields.cpu, json_data.fields.cpu);            }            write_to_file(new_data.clone(), process_name.clone(), "config", false).expect("ERROR: info is not saved!");        }        Err(error_string) => {            write_to_file(data.clone(), process_name.clone(), "config", false).expect("ERROR: info is not saved!");            println!("Error while reading file: {error_string}");        }    }}fn save_result(filename_enum: FilenameData) -> StandardResult<bool, String> {    let save_directory: String = read_config("save_directory");    Ok(        match filename_enum {            FilenameData::String(filename) => {                let saved_result: JsonData = read_json(filename.clone())?;                let counted_data: JsonData = JsonData {                    datetime: DateInfo {                        date: saved_result.datetime.date,                        time: saved_result.datetime.time,                    },                    fields: ProcessData {                        cpu: saved_result.fields.cpu,                        memory: saved_result.fields.memory,                        pid: saved_result.fields.pid,                        name: saved_result.fields.name,                    },                    counter: saved_result.counter                };                let result_filename: String = format!(                    "{0}_{1}",                    filename.clone(),                    counted_data.datetime.date.replace(":", "-")                );                write_to_file(counted_data, result_filename, &*save_directory, true).expect("ERROR: can't save the result");                remove_file(format!("config/temp/{}.json", filename.clone().replace("\"", ""))).expect("ERROR: error while removing the file");                true            },            FilenameData::Vector(files_vector) => {                let mut counted_data: JsonData = JsonData {                    datetime: DateInfo { date: "".to_string(), time: "".to_string() },                    fields: ProcessData {                        name: "All processes".to_string(),                        pid: vec![],                        memory: "0 MB".to_string(),                        cpu: "0.00%".to_string(),                    },                    counter: 0,                };                for default_filename in files_vector {                    let filename = default_filename                        .split("/").collect::<Vec<_>>()[2]                        .split(".").collect::<Vec<_>>()[0]                        .to_owned();                    let mut saved_result: JsonData = read_json(filename.clone())?;                    counted_data.datetime.date = saved_result.datetime.date;                    counted_data.datetime.time = saved_result.datetime.time;                    counted_data.fields.pid.append(&mut saved_result.fields.pid);                    counted_data.fields.memory = count_memory(counted_data.fields.memory, saved_result.fields.memory);                    counted_data.fields.cpu = count_cpu(counted_data.fields.cpu, saved_result.fields.cpu);                    counted_data.counter += saved_result.counter;                    remove_file(default_filename.clone()).expect("ERROR: error while removing the file");                };                let result_filename: String = format!(                    "{0}_{1}",                    "all-processes",                    counted_data.datetime.date.replace(":", "-")                );                write_to_file(counted_data, result_filename, &*save_directory, true).expect("ERROR: can't save the result");                true            }        }    )}fn main() -> () {    let sys: System = System::new_all();    let is_all = read_config("is_parse_all");    if is_all == "true".to_owned() {        let processes = sys.processes();        if processes.values().next().is_some() {            let processes = sys.processes();            let mut iter_count: i32 = 1;            for process in processes {                let process_name = process.1.name().split(".").collect::<Vec<_>>()[0].to_owned();                let datetime: DateInfo = get_date();                let data: JsonData = collect_json(process.1, datetime);                println!("Get data: {:?}", data);                count_and_save_data(data, process_name.clone());                println!("Iteration No. {iter_count} ended.");                iter_count += 1;                println!("––––––––––––––––––––");            }            let files = count_files_in_temp();            save_result(FilenameData::Vector(files)).expect("ERROR: Trying to save as Vector");        } else {            println!("No processes found");        }    } else {        let process_name = read_config("process_name");        let mut processes = sys.processes_by_name(&process_name);        if processes.next().is_some() {            let processes = sys.processes_by_name(&process_name);            let mut iter_count: i32 = 1;            for process in processes {                let datetime: DateInfo = get_date();                let data: JsonData = collect_json(process, datetime);                println!("Get data: {:?}", data);                count_and_save_data(data, process_name.clone());                println!("Iteration No. {iter_count} ended.");                iter_count += 1;                println!("––––––––––––––––––––");            }            save_result(FilenameData::String(process_name.clone())).unwrap();        } else {            println!("No process names {process_name} found");        }    }}