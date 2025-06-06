pub mod mem{
    pub struct Memory{
        registers: [u8; 16],
        I: u8, //16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
        delay: u8,
        timer: u8,
        PC: u16,
        SP: [u8; 16],
        RAM: [u8; 4096],
        INSTRUCTION_START: usize,
    }

    impl Memory {
        pub fn new() -> Self{
            Self {
                I: 0,
                delay: 0,
                timer: 0,
                PC: 0,
                SP: [0; 16],
                RAM: [0; 4096],
                registers: [0; 16],
                INSTRUCTION_START: 512,
            }
        }
        pub fn load_program_in_ram(&mut self, program: Vec<u8>){
            for (index,instruction) in program.iter().enumerate(){
                self.RAM[self.INSTRUCTION_START + index] = *instruction;
            }
        }
    }
}