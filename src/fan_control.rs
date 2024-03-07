use libc::ioperm;
use std::arch::asm;
use std::thread::sleep;
use std::time::Duration;

// const TEMP: u16 = 0x9E;
const INPUT_BYTE_FLAG: u16 = 1;
// const OUTPUT_BYTE_FLAG: u16 = 0;
const EC_DATA_PORT: u16 = 0x62;
const EC_COMMAND_PORT: u16 = 0x66;
const SET_RPM_COMMAND: u16 = 0x99;

fn outb(port: u16, value: u8) {
    unsafe { asm!("outb %al, %dx", in("al") value, in("dx") port, options(att_syntax)) }
}

fn inb(port: u16) -> u8 {
    unsafe {
        let ret: u8;
        asm!("inb %dx, %al", in("dx") port, out("al") ret, options(att_syntax));
        ret
    }
}
/*
fn outw(port: u16, value: u16) {
    unsafe { asm!("outw %ax, %dx", in("ax") value, in("dx") port, options(att_syntax)) }
}

fn inw(port: u16) -> u16 {
    unsafe {
        let ret: u16;
        asm!("inw %dx, %ax", in("dx") port, out("ax") ret, options(att_syntax));
        ret
    }
}
 */
fn fan_control_init() -> Result<(), String> {
    unsafe {
        if ioperm(EC_COMMAND_PORT.into(), 1, 1) != 0 {
            return Err("Error: sysio_init can't r/w ec registers. \
                       Try to run as root"
                .to_string());
        }
        if ioperm(EC_DATA_PORT.into(), 1, 1) != 0 {
            return Err("Error: sysio_init can't r/w ec registers. \
                       Try to run as root"
                .to_string());
        }
    }
    Ok(())
}

fn system_io_wait(port: u16, flag: u16, value: u8) -> Result<(), String> {
    let mut data;

    data = inb(port);

    for i in 0..100 {
        if ((data >> flag) & 0x1) != value {
            sleep(Duration::from_millis(1));
            data = inb(port);
        } else {
            break;
        }

        if i >= 99 {
            return Err(format!(
                "Error: sysio_wait runtime \
                               exeption on port: {}, data: {}, flag: {}, value: {}",
                port, data, flag, value
            ));
        }
    }
    Ok(())
}

fn system_io_write(cmd: u16, port: u16, value: u16) -> Result<(), String> {
    if let Err(err) = system_io_wait(EC_COMMAND_PORT, INPUT_BYTE_FLAG, 0) {
        return Err(err);
    } else {
        outb(EC_COMMAND_PORT, cmd as u8);
    }

    if let Err(err) = system_io_wait(EC_COMMAND_PORT, INPUT_BYTE_FLAG, 0) {
        return Err(err);
    } else {
        outb(EC_DATA_PORT, port as u8);
    }

    if let Err(err) = system_io_wait(EC_COMMAND_PORT, INPUT_BYTE_FLAG, 0) {
        return Err(err);
    } else {
        outb(EC_DATA_PORT, value as u8);
    }

    Ok(())
}

pub fn set_fan_speed(speed: u8) -> Result<(), String> {
    if speed > 100 {
        return Err(format!(
            "Wrong fan speed value: {}\n\tFan duty must be in range 0 to 100!",
            speed
        ));
    }

    let speed_hex = ((speed as f64 / 100.0) * 255.0) as u16;

    if let Err(err) = fan_control_init() {
        return Err(err);
    }

    if let Err(err) = system_io_write(SET_RPM_COMMAND, INPUT_BYTE_FLAG, speed_hex) {
        return Err(err);
    }

    Ok(())
}
