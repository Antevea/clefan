use std::env;
mod fan_control;

enum EParsedArgs {
    Help,
    Duty(u8),
    Temperature,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Usage: {} [fan_duty_percentage]", args[0]);
        return;
    }

    if let Ok(parsed_arg) = parse_args(args) {
        match parsed_arg {
            EParsedArgs::Help => print_help(),
            EParsedArgs::Duty(duty) => {
                let res = fan_control::set_fan_speed(duty);
                match res {
                    Err(err) => eprintln!("ERROR: {}", err),
                    Ok(()) => println!("INFO: Set fan speed to {}%", duty),
                }
            }
            EParsedArgs::Temperature => {
                let res = fan_control::get_cpu_temp();
                match res {
                    Err(err) => eprintln!("{}", err),
                    Ok(cpu_temp) => println!("INFO: CPU temp is {}°C", cpu_temp),
                }
            }
        };
    } else {
        eprintln!("ERROR: Wrong arguments!\n");
        print_help();
    }
}

fn parse_args(args: Vec<String>) -> Result<EParsedArgs, ()> {
    if args[1].contains("-h") {
        return Ok(EParsedArgs::Help);
    } else if args[1].contains("-d") {
        return Ok(EParsedArgs::Duty(
            args[2]
                .trim()
                .parse::<u8>()
                .expect(&format!("ERROR: Wrong duty percentage {}", args[2])),
        ));
    } else if args[1].contains("-t") {
        return Ok(EParsedArgs::Temperature);
    } else {
        return Err(())
    }
}

fn print_help() {
    println!("Clefan is a fan control utility for Clevo laptops.\n");
    println!("Usage: clefan -d [fan_duty_percentage]");
    println!("Arguments:");
    println!("\t-h\t\t\tPrint this help and exit");
    println!("\t-d [percentage]\t\tSet fan duty percentage manually: from 0 up to 100");
    println!("\t-t\t\t\tPrint CPU temperature");
    println!("To use without sudo:");
    println!("\tsudo chown root [path/to/clefan]");
    println!("\tsudo chmod u+s [path/to/clefan]");
    println!("DO NOT MANIPULATE OR QUERY EC I/O PORTS WHILE THIS PROGRAM IS RUNNING.");
}
