extern crate procfs;

use std::collections::HashMap;

fn main() {
    let mut host = HashMap::new();

    let cpu_keys = vec![
        "model name",
        "cpu cores",
        "cache size",
    ];

    let _mem_keys = vec![
        "mem_total",
        "swap_total",
    ];

    let mut cpu_details = HashMap::new();
    let mut mem_details = HashMap::new();

    let cpu = procfs::CpuInfo::new();
    match cpu {
        Ok(x) => cpu_fields(&mut cpu_details, &cpu_keys, &x.fields),
        Err(_e) => println!("{:?}", "Couldn't fetch CPU info!")
    }

    let mem = procfs::Meminfo::new();
    match mem {
        Ok(x) => mem_info(&mut mem_details, &x),
        Err(_e) => println!("{:?}", "Couldn't fetch CPU info!")
    }

    host.insert("cpu", cpu_details);
    host.insert("mem", mem_details);

    println!("{:#?}", host);

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://0.0.0.0:9090")
        .json(&host)
        .send();
    println!("{:?}", res);
}


fn cpu_fields(host_info: &mut HashMap<String, String>, keys: &Vec<&str>, cf: &HashMap<String, String>) {
    for key in keys.iter() {
        if cf.contains_key(&key.to_string()) {
            let kv = cf.get(&key.to_string());
            match kv {
                Some(x) => host_info.insert(key.to_string(), x.to_string()),
                None => host_info.insert(key.to_string(), "".to_string()),
            };
        } else {
            println!("{:?}", "No matching keys found!");
        }
    }
}

fn mem_info(host_info: &mut HashMap<String, String>, cf: &procfs::Meminfo) {
    host_info.insert("mem_total".to_string(), cf.mem_total.to_string());
    host_info.insert("swap_total".to_string(), cf.swap_total.to_string());
}
