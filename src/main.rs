
/// The Register type.
type Reg = u8;
/// The Literal type.
type Lit = i64;
/// The memory size.
const MEMORY_SIZE: usize = 32;

/// Represents the opcodes.
#[derive(Debug)]
enum Opcode {
    Loadi(Reg, Lit), // Loadi rx l1
    Addi(Reg, Reg, Lit), // Addi rx ra l1
    Compare(Reg, Reg, Reg), // Compare rx ra rb
    Jump(Lit), // Jump l1
    Branch(Reg, Lit), // Branch ra l1
    Exit // Exit
}

/// Use all the opcodes without the prefix.
use Opcode::{ * };

/// Entry point.
fn main() {
    // Program memory (Registers)
    let mut memory  = [0; MEMORY_SIZE];
    // Program code to execute.
    let code = [
        Loadi(0, 20000), // r0 = 20000;
        Loadi(1, 0), // r1 = 0;
        Compare(2, 0, 1), // r2 = r0 == r1;
        Branch(2, 2), // if (r2 == 0) goto +2;
        Addi(1, 1, 1), // r0 = r0 + 1;
        Jump(-4), // goto -4;
        Exit
    ];
    // Program counter.
    let mut pc = 0usize;
    // The VM itself.
    loop {
        let op = &code[pc];
        // println!("({}) Kind = {:?}\n\tr0={}\tr1={}\tr2={}", pc, op, memory[0], memory[1], memory[2]);
        match op {
            Loadi(rx, l1) => memory[*rx as usize] = *l1,
            Addi(rx, ra, l1) => memory[*rx as usize] = memory[*ra as usize] + l1,
            Compare(rx, ra, rb) => memory[*rx as usize] = (memory[*ra as usize] == memory[*rb as usize]) as i64,
            Jump(l1) => pc = pc.wrapping_add(*l1 as usize),
            Branch(ra, l1) => if memory[*ra as usize] != 0 { pc = pc.wrapping_add(*l1 as usize); },
            Exit => break,
        }
        pc += 1;
    }
}
