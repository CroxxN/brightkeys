use std::collections::HashMap;
use std::{fs, io, io::BufRead, io::Write};

// List out all the devices with led capabilites
fn get_devices(path: &str, map: &mut HashMap<usize, String>) -> io::Result<()> {
    for (i, entry) in fs::read_dir(path)?.enumerate() {
        if let Ok(devices) = entry?.file_name().into_string() {
            println!("{i}>\t{devices}");
            map.insert(i, devices);
        }
    }
    println!();
    Ok(())
}

// weiredly sexualize
fn toogle_device(path: &str, device: &str, mode: u8) {
    if device == "NULL" {
        return;
    }
    let full_path = std::path::Path::new(path).join(device).join("brightness");
    // println!("{}", full_path.display());
    let mut file;
    if let Ok(f) = std::fs::File::create(&full_path) {
        file = f;
    } else {
        println!("Error Opening file");
        return;
    }
    println!("{}", &full_path.display());
    if let Err(e) = file.write_fmt(format_args!("{}", mode)) {
        eprintln!("Error: {e}");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut device_map: HashMap<usize, String> = HashMap::new();
    if let Err(e) = get_devices("/sys/class/leds", &mut device_map) {
        // On linux, led capable devices are listed in /sys/class/led
        println!("Error occured: {e}");
        return;
    }
    print!("> ");
    let mut device = "NULL";
    'outer: loop {
        let mut user_input = String::new();
        print!("> ");
        _ = stdin.lock().read_line(&mut user_input);
        let user_input = user_input.trim();
        if user_input == "quit" {
            break 'outer;
        }
        if user_input == "on" {
            toogle_device("/sys/class/leds", &device, 1);
        } else if user_input.trim() == "off" {
            toogle_device("/sys/class/leds", &device, 0);
        } else if user_input == "list" {
            _ = get_devices("/sys/class/leds", &mut device_map.clone())
        } else if let Ok(user_sel) = user_input.parse::<usize>() {
            if let Some(dev) = device_map.get(&user_sel) {
                println!("Selected: {} which is {}\n", user_sel, dev);
                device = dev;
            }
        }
        print!("> ");
    }
}
