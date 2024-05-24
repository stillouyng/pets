use std::fs::File;
use std::io::{BufWriter, Write};
use sysinfo::{System, Process};
use serde_json::{Value, json, to_writer_pretty};
use chrono::prelude::*;


struct DateInfo {
    date: NaiveDate,
    time: NaiveTime
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

fn write_to_file(data: Value) -> std::io::Result<()> {
    let file = File::create("config.json")?;
    let mut writer = BufWriter::new(file);
    to_writer_pretty(&mut writer, &data)?;
    writer.flush()?;
    Ok(())
}


fn main() {
    let sys: System = System::new_all();
    for process in sys.processes_by_name("pycharm") {
        let datetime: DateInfo = get_date();
        let data = collect_json(process, datetime);
        println!("Information: {data}");
        write_to_file(data).expect("TODO: info is not saved!");
    }
}
