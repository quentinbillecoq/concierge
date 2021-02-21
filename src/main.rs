use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::collections::HashMap;

use systemstat::{System, Platform};
use systemstat::platform::PlatformImpl;
use regex::Regex;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4444").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peerip = match stream.peer_addr() {
                    Ok(addr) => format!("{}", addr),
                    Err(_) => "inconnue".to_owned()
                };
                println!("+ Client connecté: {:?}", stream.peer_addr().unwrap().ip());
                thread::spawn(move|| {
                    handle_connection(stream, &*peerip);
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}

fn handle_connection(mut stream: TcpStream, peerip: &str) {
    let _msg: Vec<u8> = Vec::new();
    loop  {
        let buffer = &mut [0; 64];
        match stream.read(buffer) {
            Ok(received) => {

                if received < 1 {
                    println!("- Client déconnecté {}", peerip);
                    break;
                }

                let val = String::from_utf8_lossy(&buffer[..]);
                let mut command = val.trim_matches(char::from(0)).to_string();
                let len = command.trim_end_matches(&['\r', '\n'][..]).len();
                #[allow(unused_mut)]
                let mut response: String;
                let sys = System::new();

                command.truncate(len);

                if command == "uptime" {
                    response = get_uptime(sys);
                    println!("[{}] Uptime: {} secondes", peerip, response);
                } else if command == "load" {
                    response = get_load(sys);
                    println!("[{}] Load: {}", peerip, response);
                } else if command == "memory" {
                    response = get_memory(sys);
                    println!("[{}] Memory: {}", peerip, response);
                } else if command == "mounts" {
                    response = get_mounts(sys);
                    println!("[{}] Mounts: {}", peerip, response);
                } else if  command == "exit" {
                    println!("Client déconnecté {}", peerip);
                    break;
                } else {
                    println!("[{}] La commande '{}' n'existe pas", peerip, command);
                    response = format!("La commande '{}' n'existe pas", command);
                }

                stream.write(format!("{}\n", response).as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(_) => {
                println!("- Client déconnecté {}", peerip);
                break;
            }
        }
    }
    

}

fn reponse_msg(status: bool, data: String) -> String {
    if status {
        format!(r#"{{ "status": "ok", "data": {} }}"#, data)
    }else{
        format!(r#"{{ "status": "error", "data": {} }}"#, data)
    }
}


fn get_uptime(_sys: PlatformImpl) -> String {
    match _sys.uptime() {
        Ok(uptime) => reponse_msg(true, format!(r#""{}""#, uptime.as_secs())),
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}

fn get_load(_sys: PlatformImpl) -> String {
    match _sys.load_average() {
        Ok(loadavg) => reponse_msg(true, format!(r#""{} {} {}""#, loadavg.one, loadavg.five, loadavg.fifteen)),
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}

fn get_memory(_sys: PlatformImpl) -> String {
    match _sys.memory() {
        Ok(mem) => {

            if cfg!(windows) {
                #[allow(unused_mut)]
                let mut _patformmemory: String = "".to_string();
                #[cfg(target_os = "windows")]
                let _patformmemory = format!(r#"{{ "total_phys":{},"avail_phys":{},"total_pagefile":{},"avail_pagefile":{},"total_virt":{},"avail_virt":{},"avail_ext":{} }}"#, 
                    mem.platform_memory.total_phys.as_u64(),
                    mem.platform_memory.avail_phys.as_u64(),
                    mem.platform_memory.total_pagefile.as_u64(),
                    mem.platform_memory.avail_pagefile.as_u64(),
                    mem.platform_memory.total_virt.as_u64(),
                    mem.platform_memory.avail_virt.as_u64(),
                    mem.platform_memory.avail_ext.as_u64()
                );
                reponse_msg(true, format!(r#"{{ "type":"windows", "free":{}, "total":{}, "patformmemory":{} }}"#, mem.free.as_u64(), mem.total.as_u64(), _patformmemory))
            } else if cfg!(unix) {
                #[allow(unused_mut)]
                let mut patformmemory: HashMap<&str, u64> = HashMap::new();
                #[cfg(target_os = "linux")]
                for (key, value) in &mem.platform_memory.meminfo {
                    patformmemory.insert(key, value.as_u64());
                }
                match serde_json::to_string(&patformmemory) {
                    Ok(data) => reponse_msg(true, format!(r#"{{ "type":"unix", "free":{}, "total":{}, "patformmemory":{} }}"#, mem.free.as_u64(), mem.total.as_u64(), data)),
                    Err(x) => reponse_msg(false, format!(r#""{}""#, x))
                }
            } else{
                reponse_msg(false, format!("Not supported"))
            }

        },
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}

fn get_mounts(_sys: PlatformImpl) -> String {
    match _sys.mounts() {
        Ok(mounts) => {
            let mut mountpoint = "".to_string();
            let re = Regex::new(r"\\").unwrap();
            for mount in mounts.iter() {
                let data = format!(r#""{}": {{ "files":{},"files_total":{},"files_avail":{},"free":{},"avail":{},"total":{},"name_max":{},"fs_type":"{}","fs_mounted_from":"{}","fs_mounted_on":"{}" }}"#, 
                    re.replace_all(&mount.fs_mounted_on, "\\\\"),
                    mount.files,
                    mount.files_total,
                    mount.files_avail,
                    mount.free.as_u64(),
                    mount.avail.as_u64(),
                    mount.total.as_u64(),
                    mount.name_max,
                    mount.fs_type,
                    re.replace_all(&mount.fs_mounted_from, "\\\\"),
                    re.replace_all(&mount.fs_mounted_on, "\\\\")
                );
                mountpoint = format!("{} {},", mountpoint, data);
            }

            let mut refdata: String = mountpoint.to_string();
            refdata.pop();
            
            reponse_msg(true, format!(r#"{{ {} }}"#, refdata.to_string()))
        },
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}