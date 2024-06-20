use std::env;
mod fan_control;
mod auto_control;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_arg = args[0].as_str();
    let default_temps_config_path = "temps_config.json".to_string();

    match args.len() {
        1 => print_help(cmd_arg),
        2 => {
            let first = args[1].as_str();

            match first {
                "-h" => print_help(cmd_arg),
                "-a" => {
                    auto_control::auto_control(default_temps_config_path);
                },
                "-t" => {
                    let res = fan_control::get_cpu_temp();
                    match res {
                        Err(err) => eprintln!("{}", err),
                        Ok(cpu_temp) => println!("INFO: CPU temp is {}°C", cpu_temp),
                    }
                }
                "-d" => {
                    println!("ERROR: No value specified for argument -d\n");
                    print_help(cmd_arg);
                }
                _ => {
                    println!("ERROR: Unexpected argument {}\n", first);
                    print_help(cmd_arg);
                }
            }
        },
        3 => {
            let first = args[1].as_str();

            match first {
                "-d" => {
                    let duty = args[2]
                        .trim()
                        .parse::<u8>()
                        .expect(&format!("ERROR: Wrong duty percentage {}", args[2]));

                    let res = fan_control::set_fan_speed(duty);
                    match res {
                        Err(err) => eprintln!("ERROR: {}", err),
                        Ok(()) => println!("INFO: Set fan speed to {}%", duty),
                    }
                }
                _ => {
                    println!("ERROR: Unexpected argument {}\n", first);
                    print_help(cmd_arg);
                }
            }
        },
        _ => {
            println!("ERROR: Too many arguments\n");
            print_help(cmd_arg);
        }
    }
}

fn print_help(cmd: &str) {
    println!("Clefan is a fan control utility for Clevo laptops.\n");
    println!("Usage: {} -d [fan_duty_percentage]", cmd);
    println!("Arguments:");
    println!("\t-h\t\t\tPrint this help and exit");
    println!("\t-d [percentage]\t\tSet fan duty percentage manually: from 0 up to 100");
    println!("\t-t\t\t\tPrint CPU temperature");
    println!("\t-a\t\t\tControl fan speed automaticaly");
    println!("To use without sudo:");
    println!("\tsudo chown root [path/to/clefan]");
    println!("\tsudo chmod u+s [path/to/clefan]");
    println!("DO NOT MANIPULATE OR QUERY EC I/O PORTS WHILE THIS PROGRAM IS RUNNING.");
}
