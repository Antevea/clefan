use crate::fan_control;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::{thread, time};
use tinyjson::*;

#[derive(Debug)]
struct FanState {
    fan: u8,
    end_temp: u8,
    begin_temp: u8,
}

fn parse_config(path: String) -> Result<(Vec<FanState>, Option<f64>), ()> {
    let temps_file_str =
        read_to_string(&path).expect(&format!("ERROR: could not read file {}", path));

    let mut sleep_time: f64 = f64::MIN;

    let mut fan_states: Vec<FanState> = Vec::new();

    let parsed_temps_config: JsonValue = temps_file_str.parse().map_err(|err| {
        eprintln!("ERROR: {path}: could not parse json temps config file\nERR MESSAGE: {err}");
    })?;
    let hashmap_temps_config: &HashMap<String, _> = parsed_temps_config.get().unwrap();

    for (label, object) in hashmap_temps_config.iter() {
        if label == "sleep_ms" {
            sleep_time = *object.get::<f64>().ok_or_else(|| {
                eprintln!(
                    "ERROR: could not get sleep time from line: {}{:?}\nConfig file {}",
                    label, object, path
                );
            })?;
            continue;
        }

        let fan_state: &HashMap<_, _> = object.get().unwrap();

        let tmp_fan = fan_state["fan"]
            .get::<f64>()
            .ok_or_else(|| {
                eprintln!(
                    "ERROR: could not get fan speed value from line: {}{:?}\nConfig file {}",
                    label, object, path
                );
            })?
            .to_owned() as u8;
        let tmp_end_temp = fan_state["end_temp"].get::<f64>()
            .ok_or_else(|| {
                eprintln!(
                    "ERROR: could not get fan speed value from line: {}{:?}\nConfig file {}",
                    label, object, path
                );
            })?
            .to_owned() as u8;
        let tmp_begin_temp = fan_state["begin_temp"].get::<f64>()
            .ok_or_else(|| {
                eprintln!(
                    "ERROR: could not get fan speed value from line: {}{:?}\nConfig file {}",
                    label, object, path
                );
            })?
            .to_owned() as u8;

        let tmp_state = FanState {
            fan: tmp_fan,
            end_temp: tmp_end_temp,
            begin_temp: tmp_begin_temp,
        };

        fan_states.push(tmp_state);
    }

    if sleep_time != f64::MIN {
        return Ok((fan_states, Some(sleep_time)));
    }

    return Ok((fan_states, None));
}

fn control_fan_speed(fan_states: Vec<FanState>, sleep_val_opt: Option<f64>) {
    let mut state_idx: usize = 0;
    let mut sleep_value: f64 = 3000.0;

    if sleep_val_opt.is_some() {
        sleep_value = sleep_val_opt.unwrap();
    }

    loop {
        if let Ok(cpu_temp) = fan_control::get_cpu_temp() {
            for (idx, state) in fan_states.iter().enumerate() {
                if (cpu_temp > state.begin_temp) && (cpu_temp < state.end_temp) {
                    let target_state_idx = idx;

                    if state_idx != target_state_idx {
                        let target_fan_speed = fan_states.get(target_state_idx).unwrap().fan;
                        fan_control::set_fan_speed(target_fan_speed).unwrap();
                        println!(
                            "[INFO] Set fan speed to: {}\tCPU temp: {}",
                            target_fan_speed, cpu_temp
                        );
                        state_idx = target_state_idx;
                    }
                }
            }

            let sleep_time = time::Duration::from_millis(sleep_value as u64);
            thread::sleep(sleep_time);
        } else {
            println!("ERROR: Can't r/w ec registers. Try to run as root!");
            break;
        }
    }
}

pub fn auto_control(path: String) -> Result<(), ()> {
    let (mut fan_states, sleep_time) = parse_config(path)?;
    fan_states.sort_by(|a, b| a.fan.cmp(&b.fan));

    control_fan_speed(fan_states, sleep_time);
    Ok(())
}
