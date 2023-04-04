use sysinfo::{CpuExt, NetworkExt, System, SystemExt};
use battery::{Manager, Battery};
use battery::units::power::watt;
use battery::units::ratio::percent;
use std::io::{stdout, Write};
use std::fs;

const RED :&str ="#[fg=red]";
const GREEN :&str = "#[fg=green]";
const YELLOW :&str = "#[fg=yellow]";
const BLU :&str = "#[fg=brightblue]";
const MAGENTA :&str = "#[fg=magenta]";
const CYAN :&str = "#[fg=cyan]";
const HOT :&str = "#[fg=red]#[bg=yellow]";
const CRITICAL :&str = "#[fg=black]#[bg=red]";
const NORMAL_BG :&str = "#[bg=black]";

fn format_data(data :u64) -> String {
    let mut formated :f32 = data as f32;
    let mut suffix :&str = "";
    let mut offset :String = String::new();
    if formated >= 1024.0 {formated = formated/1024.0; suffix = "K"}
    if formated >= 1024.0 {formated = formated/1024.0; suffix = "M"}
    let str_f = format!("{:.1}", formated);
    while (str_f.len() + offset.len()) < 5 { offset.push(' ') }
    format!("{}{}{}", offset, str_f, suffix)
}

fn main() {
    let mut sys =System::new();
    let batman = Manager::new().expect("couldn't start battery manager");
    let mut bat :Battery = match batman.batteries().unwrap().next() {
        Some(Ok(battery)) => battery,
        Some(Err(_e)) => {println!("Unable to access battery info"); return;}
        None => {println!("No batteries found"); return;}
    };
    for battery in batman.batteries().unwrap() {
        bat = battery.unwrap();
    };

    let mut stdout = stdout();
    let path = "/sys/class/thermal/thermal_zone0/hwmon1/temp1_input";
    sys.refresh_networks_list();
    loop {
        batman.refresh(&mut bat).unwrap();
        sys.refresh_networks();
        sys.refresh_memory();
        sys.refresh_cpu();

        //CPU
        print!("{}{:.1}%   ", RED, sys.global_cpu_info().cpu_usage());
        

        //RAM
        print!("{}{}M   ", BLU, sys.used_memory()/1048576);
        
        //Battery
        print!("{}{:.0}% {}{:.2}W   ", GREEN, bat.state_of_charge().get::<percent>(), CYAN, bat.energy_rate().get::<watt>());

        //temperature
        let temp :u16 = fs::read_to_string(path).expect("unable to get thermal info")[0..2].parse::<u16>().unwrap();
        let color = match temp {
            0..=25 => BLU,
            26..=34 => CYAN,
            35..=64 => GREEN,
            65..=74 => YELLOW,
            75..=84 => HOT,
            85_u16..=u16::MAX => CRITICAL,
        };
        print!("{}{}'C{}   ", color, temp, NORMAL_BG);

        //network
        for (interface_name, data) in sys.networks() {
            if interface_name == "wlan0" {
                print!("{}{}B/s | {}B/s   \r", MAGENTA, format_data(data.received()), format_data(data.transmitted()));  
            }
        }
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}
