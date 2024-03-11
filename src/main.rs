#![no_std]
#![no_main]

mod stepper;

use arduino_hal::{delay_ms, hal::delay, prelude::*};
use panic_halt as _;
use stepper::{StepDir, Stepper};
use ufmt::{uwrite, uwriteln};

struct RxBuffer {
    in_pos: usize,
    buffer: [u8; 256],
    has_command: bool,
}

impl RxBuffer {
    pub fn new() -> Self {
        RxBuffer {
            in_pos: 0, buffer: [b'\0'; 256], has_command: false,
        }
    }
    
    pub fn rx(&mut self, c: u8) {
        self.buffer[self.in_pos] = c;
        self.in_pos += 1;
        
        if c == b'\n' {
            self.has_command = true;
            // for i in 0..self.in_pos {
            //     uwrite!(&mut serial, "{}", self.buffer[i]).unwrap();
            // }
            // uwriteln!(&mut serial, "").unwrap();
            // self.in_pos = 0;
        }
    }

    pub fn get_command(&mut self) -> Option<&str> {
        if self.has_command {
            let end = self.in_pos;
            self.in_pos = 0;
            self.has_command = false;
            return core::str::from_utf8(&self.buffer[..end - 1]).ok();
        }
        
        None
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut rx_buffer = RxBuffer::new();
    
    uwriteln!(&mut serial, "Starting...").unwrap();

    let mut led = pins.d13.into_output();
    led.set_low();

    let mut stepper0 = Stepper::new(
        pins.d2.into_output(),
        pins.d3.into_output(),
        pins.d5.into_output(),
        pins.d6.into_output()
    );

    let mut stepper1 = Stepper::new(
        pins.d7.into_output(),
        pins.d8.into_output(),
        pins.d9.into_output(),
        pins.d10.into_output()
    );
    
    loop {        
        // let b = nb::block!(serial.read()).unwrap();
        // rx_buffer.rx(b);
        
        // if let Some(command) = rx_buffer.get_command() {
        //     uwriteln!(&mut serial, "{}", command).unwrap();

        //     match command {
        //         "on" => stepper0.step(),
        //         "off" => stepper0.release(),
        //         _ => {}
        //     }
        // }

        stepper0.step(StepDir::CCW);
        stepper1.step(StepDir::CW);
        delay_ms(5);
    }
}
