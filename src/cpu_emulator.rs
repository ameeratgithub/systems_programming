struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    /// Reads opcode at current position
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // Both op_byte1 and op_byte2 are 16 bit values
        // We left shift op_byte1 by 8 to create space for op_byte2
        // Then we merge it via bitwise OR (|)
        op_byte1 << 8 | op_byte2
    }

    /// Assumes that all instructions are loaded into memory, and ready for execution
    fn run(&mut self) {
        // Continuous Execution
        loop {
            let opcode = self.read_opcode();
            // Reading opcode means two bytes have been read. So incrementing position
            // by 
            self.position_in_memory += 2;

            // Extracting first nibble, high byte, high nibble
            let c = ((opcode & 0xF000) >> 12) as u8;
            // Extracting second nibble, high byte, low nibble
            let x = ((opcode & 0x0F00) >> 8) as u8;
            // Extracting third nibble, low byte, high nibble
            let y = ((opcode & 0x00F0) >> 4) as u8;
            // Extracting forth nibble, low byte, low nibble
            let d = (opcode & 0x000F) as u8;
            // Extract address of 12 bits (3 nibbles)
            let addr = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    // Exit the main loop
                    return;
                }
                // 0x00EE is opcode to return from current function
                (0, 0, 0xE, 0xE) => self.ret(),
                // 0x2 indicates that a function should be called, at address `addr`
                (0x2, _, _, _) => self.call(addr),
                // Just add value at register `x` to value at register `y`
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                // Other opcodes need implementation
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    /// Calls a function and starts executing function instructions at given address
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        // Storing the current position in memory in stack. When returning from a function,
        // This helps to track where we left things before calling function
        stack[sp] = self.position_in_memory as u16;
        // Stack pointer is increased. This prevents from position_in_memory being overwritten
        // in stack if a nested function is called
        self.stack_pointer += 1;
        // Setting `position_in_memory` to start execution of function's instructions
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack overflow!");
        }
        // Decrement stack pointer, BEFORE getting the return address, because it was 
        // incremented AFTER storing the position in memory
        self.stack_pointer -= 1;
        // Get the address where we started things
        let call_addr = self.stack[self.stack_pointer];
        // Set position in memory to the address
        self.position_in_memory = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // It will add value, but will return boolean if addition
        // caused overflow
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // Last register is used as carry flag. When set, it means overflow has been occured
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

}

#[allow(dead_code)]
pub fn load_and_run() {
    call_user_defined_functions();
}

fn call_user_defined_functions() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // Set opcode to 0x2100 to call the function at 0x100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    // Set opcode to 0x2100 to call the function at 0x100
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    // This isn't strictly neccessary to halt the function because cpu.memory is
    // initialized with null bytes.
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    // Set the opcode to 0x8014: ADD register 1's value to register 0
    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    // Set the opcode to 0x8014: ADD register 1's value to register 0
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    // Set the opcode to 0x00EE: RETURN
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    let result = cpu.registers[0];
    assert_eq!(result, 45);
    println!("5 + (10 * 2) + (10 * 2) = {result}");
}

#[allow(dead_code)]
fn add_numbers() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    // Loads opcode 0x8014, which adds register 1 to register 0
    mem[0] = 0x80;
    mem[1] = 0x14;
    // Loads opcode 0x8024, which adds register 2 to register 0
    mem[2] = 0x80;
    mem[3] = 0x24;
    // Loads opcode 0x8034, which adds register 3 to register 0
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
