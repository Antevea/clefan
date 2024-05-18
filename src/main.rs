use std::env;
mod fan_control;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_arg = args[0].as_str();

    match args.len() {
        1 => print_help(cmd_arg),
        2 => {
            let first = args[1].as_str();

            match first {
                "-h" => print_help(cmd_arg),
                "-t" => {
                    let res = fan_control::get_cpu_temp();
                    match res {
                        Err(err) => eprintln!("{}", err),
                        Ok(cpu_temp) => println!("INFO: CPU temp is {}°C", cpu_temp),
                    }
                }
                _ => {
                    println!("ERROR: Unexpected argument {}", first);
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
                    println!("ERROR: Unexpected argument {}", first);
                    print_help(cmd_arg);
                }
            }
        },
        _ => {
            println!("ERROR: Too many arguments");
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
    println!("To use without sudo:");
    println!("\tsudo chown root [path/to/clefan]");
    println!("\tsudo chmod u+s [path/to/clefan]");
    println!("DO NOT MANIPULATE OR QUERY EC I/O PORTS WHILE THIS PROGRAM IS RUNNING.");
}
