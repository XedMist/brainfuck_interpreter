use std::io::{BufWriter, Read, Write};

const MEMORY_SIZE: usize = 3000;


pub struct Interpreter {
    source: Vec<u8>,
    dp: usize,
    ip: usize,
    memory: [u8; MEMORY_SIZE],
    stack: Vec<usize>,
}

impl Interpreter {
    pub fn new(source: &str) -> Self{
        Self{
            source: source.into(),
            dp: 0,
            ip: 0,
            memory: [0; MEMORY_SIZE],
            stack : Vec::with_capacity(16)
        }
    }

    pub fn next_instruction(&mut self) -> Option<u8> {
        if self.ip < self.source.len() {
            let c = self.source[self.ip];
            self.ip += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn run(&mut self, input: &mut impl Read, output: &mut impl Write) -> Result<(), String> {
        while let Some(c) = self.next_instruction() {
            match c {
                b'>' => {
                    self.dp = (self.dp + 1) % MEMORY_SIZE;
                },
                b'<' => {
                    self.dp = (self.dp + MEMORY_SIZE - 1) % MEMORY_SIZE;
                },
                b'+' => {
                    self.memory[self.dp] = self.memory[self.dp].wrapping_add(1);
                },
                b'-' => {
                    self.memory[self.dp] = self.memory[self.dp].wrapping_sub(1);
                },
                b'.' => {
                    output.write_all(&self.memory[self.dp..=self.dp]).map_err(|e| format!("Output error - {e}"))?;
                    output.flush().unwrap();
                }
                b',' => {
                    let mut byte = [0];
                    loop {
                        input.read_exact(&mut byte).map_err(|e| format!("Input error - {e}"))?;
                        if byte[0] != b'\n' {
                            break;
                        }
                    }
                    self.memory[self.dp] = byte[0];
                },
                b'[' => {
                    if self.memory[self.dp] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            if let Some(n) = self.next_instruction() {
                                match n {
                                    b'[' => depth += 1,
                                    b']' => depth -= 1,
                                    _ => {}
                                }
                            } else {
                                return Err(format!("Unexpected end of loop"))
                            }
                        }
                    } else {
                        self.stack.push(self.ip);
                    }

                },
                b']' => {
                    if let Some(&j) = self.stack.last() {
                        if self.memory[self.dp] != 0 {
                            self.ip = j;
                        } else {
                            self.stack.pop();
                        }
                    } else {
                        return Err(format!("Malformed loop"))
                    }
                },
                _ => {} 
            }
        }
        Ok(())
    }
}


pub struct Transpiler {
    source: Vec<u8>,
    current: usize,
}

impl Transpiler {
    pub fn new(source: &str) -> Self {
        Self{
            source: source.into(),
            current: 0
        }
    }
    pub fn next_char(&mut self) -> Option<u8> {
        if self.current < self.source.len() {
            let c = self.source[self.current];
            self.current += 1;
            Some(c)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<u8> {
        if self.current < self.source.len() {
            Some(self.source[self.current])
        } else {
            None
        }
    }

    pub fn get_repetition(&mut self, r: u8) -> usize {
        let mut i = 1;
        while let Some(c) = self.peek(){
            if c == r {
                i += 1;
                self.next_char();
            } else {
                break;
            }
        }
        return i;
    }

    pub fn transpile(&mut self, file: &mut impl Write) -> Result<(), String> {
        let mut writer = BufWriter::new(file);
        const INTRO: &str = r#"
#include <stdio.h>
#define MAXMEM 30000
int main(void) {
    char memory[MAXMEM];
    char *ptr = memory;
"#;
        let buf = INTRO.bytes().collect::<Vec<u8>>();
        writer.write_all(&buf).map_err(|e| format!("Output error - {e}"))?;

        let mut indent = 1;
        while let Some(c) = self.next_char() {

            for _ in 0..if c == b']' {indent - 1} else {indent} {
                write!(writer, "    ").map_err(|e| format!("Output error - {e}"))?;
            }

            match c {
                b'>' => {
                    write!(writer, "ptr += {};\n", self.get_repetition(b'>')).map_err(|e| format!("Output error - {e}"))?;
                },
                b'<' => {
                    write!(writer, "ptr -= {};\n", self.get_repetition(b'<')).map_err(|e| format!("Output error - {e}"))?;
                },
                b'+' => {
                    write!(writer, "*ptr += {};\n", self.get_repetition(b'+')).map_err(|e| format!("Output error - {e}"))?;
                },
                b'-' => {
                    write!(writer, "*ptr -= {};\n", self.get_repetition(b'-')).map_err(|e| format!("Output error - {e}"))?;
                },
                b'[' => {
                    indent += 1;
                    write!(writer, "while (*ptr) {{ \n").map_err(|e| format!("Output error - {e}"))?;
                },
                b']' => {
                    write!(writer, "}}\n").map_err(|e| format!("Output error - {e}"))?;
                    indent -= 1;
                },
                b'.' => {
                    write!(writer, "putchar(*ptr);\n").map_err(|e| format!("Output error - {e}"))?;
                },
                b',' => {
                    write!(writer, "*ptr = getchar();\n").map_err(|e| format!("Output error - {e}"))?;
                },
                _ => {}
            }
        }
        write!(writer, "}}\n").map_err(|e| format!("Output error - {e}"))?;
        Ok(())
    }

}



