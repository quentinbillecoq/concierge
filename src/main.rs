use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::thread;

use systemstat::{System, Platform};
use systemstat::platform::PlatformImpl;


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
                let mut response: String = "".to_string();
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
        Ok(mem) => reponse_msg(true, format!("{:?}", mem.platform_memory)),
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}

fn get_mounts(_sys: PlatformImpl) -> String {
    match _sys.mounts() {
        Ok(mounts) => {
            let mut data: String = "{".to_string();
            for mount in mounts.iter() {
                data = format!("{}{:?},", data,mount);
            }
            data = format!("{} }}", data);

            reponse_msg(true, format!("{:?}", data))
        },
        Err(x) => reponse_msg(false, format!(r#""{}""#, x))
    }
}