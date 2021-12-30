use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
                false
            },
            Opcode::HLT => {
                println!("HLT encountered");
                true
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                false
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                false
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                false
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                false
            },
            _ => {
                println!("Illegal opcode. Terminating.");
                true
            }
        }
    } 

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        /*
         * LOAD $0 #10
         * LOAD $1 #15
         * ADD $0 $1 $2
         */
        let instruction1 = vec![1, 0, 0, 10];
        let instruction2 = vec![1, 1, 0, 15];
        let instruction3 = vec![2, 0, 1, 2];
        test_vm.program = vec![];
        test_vm.program.extend(&instruction1);
        test_vm.program.extend(&instruction2);
        test_vm.program.extend(&instruction3);
        test_vm.run();
        assert_eq!(test_vm.registers[0], 10);
        assert_eq!(test_vm.registers[1], 15);
        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    fn test_opcode_sub() {
        /*
        LOAD $0 #10
        LOAD $1 #2
        SUB $0 $1 $2
        */
        let mut test_vm = VM::new();
        let instruction1 = vec![1, 0, 0, 10];
        let instruction2 = vec![1, 1, 0, 2];
        let instruction3 = vec![3, 0, 1, 2];
        test_vm.program = vec![];
        test_vm.program.extend(&instruction1);
        test_vm.program.extend(&instruction2);
        test_vm.program.extend(&instruction3);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 8);
    }

    #[test]
    fn test_opcode_mul() {
        /*
        LOAD $0 #2
        LOAD $0 #4
        MUL $0 $1 $2
        */
        let mut test_vm = VM::new();
        let instruction1 = vec![1, 0, 0, 2];
        let instruction2 = vec![1, 1, 0, 4];
        let instruction3 = vec![4, 0, 1, 2];
        test_vm.program = vec![];
        test_vm.program.extend(&instruction1);
        test_vm.program.extend(&instruction2);
        test_vm.program.extend(&instruction3);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 8);
    }

    #[test]
    fn test_opcode_div() {
        /*
        LOAD $0 #8
        LOAD $1 #5
        DIV $0 $1 $2
        */
        let mut test_vm = VM::new();
        let instruction1 = vec![1, 0, 0, 8];
        let instruction2 = vec![1, 1, 0, 5];
        let instruction3 = vec![5, 0, 1, 2];
        test_vm.program = vec![];
        test_vm.program.extend(&instruction1);
        test_vm.program.extend(&instruction2);
        test_vm.program.extend(&instruction3);
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);

    }
}