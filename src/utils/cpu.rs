
enum Instruction {
    ADD(AritmaticTarget),
    ADDHL(AritmaticTarget),
    ADC(AritmaticTarget),
    SUB(AritmaticTarget),
    SBC(AritmaticTarget),
    AND(AritmaticTarget),
    OR(AritmaticTarget),
    XOR(AritmaticTarget),
    CP(AritmaticTarget),
    INC(AritmaticTarget),
    DEC(AritmaticTarget),

    // unimplemented instructions for future use
    CCF(AritmaticTarget),
    SCF(AritmaticTarget),
    RRA(AritmaticTarget),
    RLA(AritmaticTarget),
    RRCA(AritmaticTarget),
    RRLA(AritmaticTarget),
    CPL(AritmaticTarget),
    BIT(AritmaticTarget),
    RESET(AritmaticTarget),
    SET(AritmaticTarget),
    SRL(AritmaticTarget),
    RR(AritmaticTarget),
    RL(AritmaticTarget),
    RRC(AritmaticTarget),
    RLC(AritmaticTarget),
    SRA(AritmaticTarget),
    SLA(AritmaticTarget),
    SWAP(AritmaticTarget),
}

enum AritmaticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister) -> u8 {
        (if flags.zero          { 1 } else { 0 }) |
        (if flags.subtract      { 1 } else { 0 }) |
        (if flags.half_carry    { 1 } else { 0 }) |
        (if flags.carry         { 1 } else { 0 })
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero =          (byte >> ZERO_FLAG_BYTE_POSSITION) & 0b1 != 0;
        let subtract =      (byte >> SUBTRACT_FLAG_BYTE_POSSITION) & 0b1 != 0;
        let half_carry =    (byte >> HALF_CARRY_FLAG_BYTE_POSSITION) & 0b1 != 0;
        let carry =         (byte >> CARRY_FLAG_BYTE_POSSITION) & 0b1 != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

impl Registers {

    // AF Register Pair Accessors
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    // BC Register Pair Accessors
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // DE Register Pair Accessors
    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    // HL Register Pair Accessors
    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {

            Instruction::ADD(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::ADDHL(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l as u16;
                        let new_value = self.add_hl(value);
                        self.registers.set_hl(new_value);
                    }
                }
            }

            Instruction::ADC(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add_c(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::SUB(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::SBC(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub_c(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::AND(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.and(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::OR(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.or(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::XOR(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.xor(value);
                        self.registers.a = new_value;
                    }
                }
            }

            Instruction::CP(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        self.cp(value);
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        self.cp(value);
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        self.cp(value);
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        self.cp(value);
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        self.cp(value);
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        self.cp(value);
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        self.cp(value);
                    }
                }
            }

            Instruction::INC(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.inc(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.inc(value);
                        self.registers.b = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.inc(value);
                        self.registers.c = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.inc(value);
                        self.registers.d = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.inc(value);
                        self.registers.e = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.inc(value);
                        self.registers.h = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.inc(value);
                        self.registers.l = new_value;
                    }
                }
            }

            Instruction::DEC(target) => {
                match target {
                    AritmaticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.dec(value);
                        self.registers.a = new_value;
                    }
                    AritmaticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.dec(value);
                        self.registers.b = new_value;
                    }
                    AritmaticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.dec(value);
                        self.registers.c = new_value;
                    }
                    AritmaticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.dec(value);
                        self.registers.d = new_value;
                    }
                    AritmaticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.dec(value);
                        self.registers.e = new_value;
                    }
                    AritmaticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.dec(value);
                        self.registers.h = new_value;
                    }
                    AritmaticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.dec(value);
                        self.registers.l = new_value;
                    }
                }
            }
        }
    }

    fn add(&mut self, value: u8) -> u8{
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) > 0xF;
        self.registers.f.carry = did_overflow;
        
        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (new_hl, did_overflow) = hl.overflowing_add(value);
        
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
        self.registers.f.carry = did_overflow;
        
        new_hl
    }

    fn add_c(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (intermediate_value, did_overflow1) = self.registers.a.overflowing_add(value);
        let (new_value, did_overflow2) = intermediate_value.overflowing_add(carry);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + carry) > 0xF;
        self.registers.f.carry = did_overflow1 || did_overflow2;
        
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
        
        new_value
    }

    fn sub_c(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (intermediate_value, did_overflow1) = self.registers.a.overflowing_sub(value);
        let (new_value, did_overflow2) = intermediate_value.overflowing_sub(carry);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < ((value & 0xF) + carry);
        self.registers.f.carry = did_overflow1 || did_overflow2;
        
        new_value
    }

    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        
        new_value
    }

    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        
        new_value
    }

    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        
        new_value
    }

    fn cp(&mut self, value: u8) {
        let (result, did_overflow) = self.registers.a.overflowing_sub(value);
        
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;
    }

    fn inc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_add(1);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) + 1 > 0xF;
        
        new_value
    }

    fn dec(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_sub(1);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value & 0xF) == 0;
        
        new_value
    }
}