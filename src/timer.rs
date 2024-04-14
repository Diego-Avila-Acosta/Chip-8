pub struct Timer{
    pub number: u8,
    flag: bool,
    acc: f64,
    cycle: f64
}

impl Timer {
    pub fn new(hertz: u8) -> Timer{
        Timer {
            number: 0,
            flag: false,
            acc: 0.0,
            cycle: 1.0 / (hertz as f64)
        }
    }

    pub fn set(&mut self, number: u8){ 
        self.number = number;
        self.flag = true;
    }

    pub fn get(&self) -> u8 { self.number }

    pub fn check(&mut self, delta_time: f64) {
        if self.flag {
            self.acc += delta_time;

            while self.acc >= self.cycle{
                self.decrement();
                self.acc -= self.cycle;
            }
        }
    }

    fn decrement(&mut self) {
        self.number -= 1;
        if self.number == 0 { self.flag = false; }
    }
}