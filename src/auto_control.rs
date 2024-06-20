use std::{thread, time};
use std::fs::read_to_string;
use std::collections::HashMap;
use tinyjson::*;
use crate::fan_control;


#[derive(Debug)]
struct FanState {
    fan: u8,
    end_temp: u8,
    begin_temp: u8
}

fn parse_config(path: String) -> Vec<FanState> {
    let temps_file_str = read_to_string(path.clone())
        .expect(&format!("ERROR: File {} not found!", path));

    let mut fan_states: Vec<FanState> = Vec::new();

    let parsed_temps_config: JsonValue = temps_file_str.parse().unwrap();
    let hashmap_temps_config: &HashMap<String, _> = parsed_temps_config.get().unwrap();

    for (_, object) in hashmap_temps_config.iter() {
        let fan_state: &HashMap<_, _> = object.get().unwrap();

        let tmp_fan = fan_state["fan"].get::<f64>().unwrap().to_owned() as u8;
        let tmp_end_temp = fan_state["end_temp"].get::<f64>().unwrap().to_owned() as u8;
        let tmp_begin_temp = fan_state["begin_temp"].get::<f64>().unwrap().to_owned() as u8;

        let tmp_state = FanState {
            fan: tmp_fan,
            end_temp: tmp_end_temp,
            begin_temp: tmp_begin_temp 
        };

        fan_states.push(tmp_state);
    }

    fan_states
}

fn control_fan_speed(fan_states: Vec<FanState>) {
    let mut state_idx: usize = 0;

    loop {
        let cpu_temp = fan_control::get_cpu_temp().unwrap();
        for (idx, state) in fan_states.iter().enumerate() {
            if (cpu_temp > state.begin_temp) && (cpu_temp < state.end_temp) {
                let target_state_idx = idx;

                if state_idx != target_state_idx {
                    let target_fan_speed = fan_states.get(target_state_idx).unwrap().fan;
                    fan_control::set_fan_speed(target_fan_speed).unwrap();
                    println!("[INFO] Set fan speed to: {}", target_fan_speed);
                    state_idx = target_state_idx;
                }
            }
        }

        let sleep_time = time::Duration::from_millis(3000);
        thread::sleep(sleep_time);
    }
}

pub fn auto_control(path: String) {
    let mut fan_states = parse_config(path);
    fan_states.sort_by(|a, b| a.fan.cmp(&b.fan));

    control_fan_speed(fan_states);
}
