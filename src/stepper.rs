use arduino_hal::port::{mode::Output, Pin, PinOps};

pub struct Stepper<M1, M2, M3, M4> {
    m1: Pin<Output, M1>,
    m2: Pin<Output, M2>,
    m3: Pin<Output, M3>,
    m4: Pin<Output, M4>,
    state: i8,
}

pub enum StepDir {
    CW,
    CCW
}

impl <M1, M2, M3, M4> Stepper<M1, M2, M3, M4>
where
    M1: PinOps,
    M2: PinOps,
    M3: PinOps,
    M4: PinOps,
{
    pub fn new(
        m1: Pin<Output, M1>,
        m2: Pin<Output, M2>,
        m3: Pin<Output, M3>,
        m4: Pin<Output, M4>
    ) -> Self {
        let state = 0;
        let mut stepper = Self {m1, m2, m3, m4, state};
        stepper.release();
        stepper
    }

    pub fn release(&mut self) {
        self.m1.set_low();
        self.m2.set_low();
        self.m3.set_low();
        self.m4.set_low();
    }

    pub fn step(&mut self, dir: StepDir) {
        self.drive_state();
        
        match dir {
            StepDir::CW => self.state += 1,
            StepDir::CCW => self.state -= 1,
        }

        if self.state > 3 {
            self.state = 0;
        } else if self.state < 0 {
            self.state = 3;
        }
    }

    fn drive_state(&mut self) {
        match self.state {
            0 => {
                self.m1.set_high();
                self.m2.set_low();
                self.m3.set_low();
                self.m4.set_low();
            },
            1 => {
                self.m1.set_low();
                self.m2.set_high();
                self.m3.set_low();
                self.m4.set_low();
            },
            2 => {
                self.m1.set_low();
                self.m2.set_low();
                self.m3.set_high();
                self.m4.set_low();
            },
            3 => {
                self.m1.set_low();
                self.m2.set_low();
                self.m3.set_low();
                self.m4.set_high();
            },
            _ => self.release(),
        }
    }
}