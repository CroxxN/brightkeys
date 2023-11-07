use std::{fs, io};


// List out all the devices with led capabilites
fn get_devices(path: &str) -> io::Result<()>{
    for entry in fs::read_dir(path)? {
	if let Ok(devices) = entry?.file_name().into_string(){
            println!("{}", devices);            
	}
    }
    Ok(())
}

fn main() {
    if let Err(e) = get_devices("/sys/class/leds"){ // On linux, led capable devices are listed in /sys/class/led
	println!("Error occured: {e}");
    }
}
