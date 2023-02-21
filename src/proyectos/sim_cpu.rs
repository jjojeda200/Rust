#[allow(dead_code)]
#[derive(Debug)]
struct Register8 {
    value: u8,
    ac: u16,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
}

#[allow(dead_code)]
impl Register8 {
    fn new() -> Register8 {
        Register8 {
            value: 0,
            ac: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
        }
    }
    fn load(&mut self, value: u8) {
        self.value = value;
    }
    fn load_ac(&mut self, ac: u16) {
        self.ac = ac;
    }

    fn store(&self) -> u8 {
        self.value
    }

    fn store_ac(&self) -> u16 {
        self.ac
    }
}

#[allow(dead_code)]
impl Register8 {
    fn add(&mut self, value: u8) {
        self.value = self.value.wrapping_add(value);
    }

    fn sub(&mut self, value: u8) {
        self.value = self.value.wrapping_sub(value);
    }

    fn mul(&mut self, value: u8) {
        self.value = self.value.wrapping_mul(value);
    }

    fn div(&mut self, value: u8) {
        if value != 0 {
            self.value = self.value.wrapping_div(value);
        }
    }
}

#[allow(dead_code)]
impl Register8 {
    fn and(&mut self, value: u8) {
        self.value &= value;
    }

    fn or(&mut self, value: u8) {
        self.value |= value;
    }

    fn xor(&mut self, value: u8) {
        self.value ^= value;
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sim_cpu() {
    // let mut cpu = Register8{value:0, b:0, c:0, e:0, f:0, ac:0, pc:0};
    let mut cpu = Register8::new();
    cpu.b = 0xf;
    cpu.load_ac(0xff);
    println!("{:?}", cpu);
    println!("{:x}", cpu.ac);

    println!("{:x}", cpu.store_ac());
}
