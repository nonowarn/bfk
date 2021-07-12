use std::io::{Read, Write};

/// Language to parse and execute.
pub struct Language {
    inc: char,
    dec: char,
    inc_ptr: char,
    dec_ptr: char,
    put_char: char,
    get_char: char,
    loop_start: char,
    loop_end: char,
}

impl Language {
    pub fn is_token(&self, ch: char) -> bool {
        self.inc == ch ||
            self.dec == ch ||
            self.inc_ptr == ch ||
            self.dec_ptr == ch ||
            self.put_char == ch ||
            self.get_char == ch ||
            self.loop_start == ch ||
            self.loop_end == ch
    }

    /// Make from string. The length of string must be 8
    pub fn make_from_string(s: &String) -> Option<Language> {
        if s.chars().count() != 8 {
            return None;
        }

        let chars = s.chars().collect::<Vec<char>>();

        Some(
            Language {
                inc: chars[0],
                dec: chars[1],
                inc_ptr: chars[2],
                dec_ptr: chars[3],
                get_char: chars[4],
                put_char: chars[5],
                loop_start: chars[6],
                loop_end: chars[7],
            }
        )
    }
}

/// Provides default brainfuck language
impl Default for Language {
    /// Default brainfuck language
    fn default() -> Self {
        Language {
            inc: '+',
            dec: '-',
            inc_ptr: '>',
            dec_ptr: '<',
            get_char: ',',
            put_char: '.',
            loop_start: '[',
            loop_end: ']'
        }
    }
}

/// Operations
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Op {
    /// Increment data.
    Inc,
    /// Decrement data.
    Dec,
    /// Increment pointer.
    IncPtr,
    /// Decrement pointer.
    DecPtr,
    /// Get and put character of data under pointer.
    PutChar,
    /// Read character of data under pointer to stdout.
    GetChar,
    /// Start of loop.
    LoopStart,
    /// End of loop.
    LoopEnd,
}

/// Compressed operations
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompressedOp {
    /// Add to data
    Add(u8),
    /// Subtract from data
    Sub(u8),
    /// Move back pointer
    Back(usize),
    /// Move forward pointer
    Forward(usize),
    /// Get and put character of data under pointer.
    PutChar,
    /// Read character of data under pointer to stdout.
    GetChar,
    /// Start of loop.
    LoopStart,
    /// End of loop.
    LoopEnd,
}

/// Execution environment.
pub struct Environment<'a, R, W> {
    data: &'a mut [u8],
    pc: usize,
    pointer: usize,
    reader: &'a mut R,
    writer: &'a mut W,
}

impl<'a, R: Read, W: Write> Environment<'a, R, W> {
    /// Add to data
    pub fn add(&mut self, n: u8) {
        self.data[self.pointer] = self.data[self.pointer].wrapping_add(n);
    }

    /// Sub from data
    pub fn sub(&mut self, n: u8) {
        self.data[self.pointer] = self.data[self.pointer].wrapping_sub(n);
    }

    /// Add to pointer
    pub fn add_ptr(&mut self, n: usize) {
        self.pointer += n;
    }

    /// Sub from pointer
    pub fn sub_ptr(&mut self, n: usize) {
        self.pointer -= n;
    }

    /// Print data under the pointer as a character
    pub fn put_char(&mut self) {
        write!(self.writer, "{}", self.data[self.pointer] as char).unwrap();
        self.writer.flush().unwrap();
    }

    /// Read a character into data
    pub fn read_char(&mut self) {
        let char = self.reader
            .bytes()
            .next()
            .and_then(|result| result.ok());
        self.data[self.pointer] = char.unwrap_or(0);
    }

    /// Increment program pointer
    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    /// Set program counter
    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    /// Read data under the pointer
    pub fn read_data(&self) -> u8 {
        self.data[self.pointer]
    }

    pub fn new(data: &'a mut [u8], reader: &'a mut R, writer: &'a mut W) -> Self {
        Environment {
            data,
            writer,
            reader,
            pointer: 0,
            pc: 0
        }
    }
}

/// Executable brainfuck operations
pub struct Code<T> {
    ops: Vec<T>,
    jump_table: Vec<usize>
}

/// Parse source code into the operations
pub fn parse(source: &String, language: &Language) -> Code<Op> {
    let token_chars = source.chars().filter(|&c| language.is_token(c));

    let mut ops = Vec::new();
    let mut jump_table = vec![0; token_chars.clone().count()];
    let mut map_stack = Vec::new();

    for (pc, char) in token_chars.enumerate() {
        if language.inc == char {
            ops.push(Op::Inc);
        } else if language.dec == char {
            ops.push(Op::Dec);
        } else if language.inc_ptr == char {
            ops.push(Op::IncPtr);
        } else if language.dec_ptr == char {
            ops.push(Op::DecPtr);
        } else if language.put_char == char {
            ops.push(Op::PutChar);
        } else if language.get_char == char {
            ops.push(Op::GetChar);
        } else if language.loop_start == char {
            ops.push(Op::LoopStart);
            map_stack.push(pc);
        } else if language.loop_end == char {
            ops.push(Op::LoopEnd);
            let begin = map_stack.pop().expect("Unmatched loop end");
            jump_table[begin] = pc + 1;
            jump_table[pc] = begin + 1;
        }
    }

    Code { ops, jump_table }
}

/// Compress operations
pub fn compress(code: &Code<Op>) -> Code<CompressedOp> {
    let mut compressed_ops = Vec::new();

    let mut last_op: Option<Op> = None;
    let mut count: usize = 1;
    let mut pc = 0;
    let mut map_stack = Vec::new();
    let mut op_groups: Vec<(Op, usize)> = Vec::new();

    for op in code.ops.iter() {
        if let Some(last_op_) = last_op {
            if last_op_ == *op && (last_op_ == Op::Inc || last_op_ == Op::Dec || last_op_ == Op::IncPtr || last_op_ == Op::DecPtr) {
                count += 1;
                continue;
            } else {
                op_groups.push((last_op_, count));

                last_op = Some(op.clone());
                count = 1;
            }
        } else {
            last_op = Some(op.clone());
        }
    }

    if last_op.is_some() {
        op_groups.push((last_op.unwrap(), count));
    }

    let mut jump_table = vec![0; op_groups.len()];

    for (op, count) in op_groups {
        match op {
            Op::Inc => {
                compressed_ops.push(CompressedOp::Add(count as u8));
                pc += 1;
            }
            Op::Dec => {
                compressed_ops.push(CompressedOp::Sub(count as u8));
                pc += 1;
            }
            Op::IncPtr => {
                compressed_ops.push(CompressedOp::Forward(count));
                pc += 1;
            }
            Op::DecPtr => {
                compressed_ops.push(CompressedOp::Back(count));
                pc += 1;
            }
            Op::PutChar => {
                compressed_ops.push(CompressedOp::PutChar);
                pc += 1;
            }
            Op::GetChar => {
                compressed_ops.push(CompressedOp::GetChar);
                pc += 1;
            }
            Op::LoopStart => {
                compressed_ops.push(CompressedOp::LoopStart);
                map_stack.push(pc);
                pc += 1;
            }
            Op::LoopEnd => {
                compressed_ops.push(CompressedOp::LoopEnd);
                let begin = map_stack.pop().expect("Unmatched loop end");
                jump_table[begin] = pc + 1;
                jump_table[pc] = begin + 1;
                pc += 1;
            }
        }
    }

    Code { ops: compressed_ops, jump_table }
}

/// Represents runnable operations
pub trait Runnable {
    /// Run the operation over code and environment
    fn run<R: Read, W: Write>(&self, code: &Code<Self>, env: &mut Environment<R, W>) where Self: Sized;

    fn process_loop_start<R: Read, W: Write>(code: &Code<Self>, env: &mut Environment<R, W>) where Self: Sized {
        if env.read_data() == 0 {
            env.set_pc(code.jump_table[env.pc]);
        } else {
            env.advance_pc();
        };
    }

    fn process_loop_end<R: Read, W: Write>(code: &Code<Self>, env: &mut Environment<R, W>) where Self: Sized {
        if env.read_data() != 0 {
            env.set_pc(code.jump_table[env.pc]);
        } else {
            env.advance_pc();
        }
    }
}

impl Runnable for Op {
    fn run<R: Read, W: Write>(&self, code: &Code<Self>, env: &mut Environment<R, W>) {
        match self {
            Op::Inc => { env.add(1); env.advance_pc(); }
            Op::Dec => { env.sub(1); env.advance_pc(); }
            Op::IncPtr => { env.add_ptr(1); env.advance_pc(); }
            Op::DecPtr => { env.sub_ptr(1); env.advance_pc(); }
            Op::PutChar => { env.put_char(); env.advance_pc(); }
            Op::GetChar => { env.read_char(); env.advance_pc(); }
            Op::LoopStart => {
                Runnable::process_loop_start(code, env);
            }
            Op::LoopEnd => {
                Runnable::process_loop_end(code, env);
            }
        }
    }
}

impl Runnable for CompressedOp {
    fn run<R: Read, W: Write>(&self, code: &Code<Self>, env: &mut Environment<R, W>) where Self: Sized {
        match self {
            CompressedOp::Add(n) => { env.add(*n); env.advance_pc(); }
            CompressedOp::Sub(n) => { env.sub(*n); env.advance_pc(); }
            CompressedOp::Back(n) => { env.sub_ptr(*n); env.advance_pc(); }
            CompressedOp::Forward(n) => { env.add_ptr(*n); env.advance_pc(); }
            CompressedOp::PutChar => { env.put_char(); env.advance_pc(); }
            CompressedOp::GetChar => { env.read_char(); env.advance_pc(); }
            CompressedOp::LoopStart => {
                Runnable::process_loop_start(code, env);
            }
            CompressedOp::LoopEnd => {
                Runnable::process_loop_end(code, env);
            }
        }
    }
}

/// Execute operations
pub fn run<R: Read, W: Write, O: Runnable>(code: &Code<O>, env: &mut Environment<R, W>) {
    let len_ops = code.ops.len();

    while len_ops > env.pc {
        code.ops[env.pc].run(&code, env);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;
    use std::io::Cursor;

    const BUF_SIZE: usize = 1024;
    const HELLO_BF: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    #[test]
    fn test_parse_ops() {
        let source = "+-.,[><]".to_string();
        let language = Language::default();

        let result = parse(&source, &language);

        assert_eq!(result.ops, vec![
            Op::Inc,
            Op::Dec,
            Op::PutChar,
            Op::GetChar,
            Op::LoopStart,
            Op::IncPtr,
            Op::DecPtr,
            Op::LoopEnd,
        ]);
    }

    #[test]
    fn test_parse_jumps() {
        let source = "[+++]--[+[+]+]".to_string();
        let language = Language::default();

        let result = parse(&source, &language);

        assert_eq!(result.jump_table[0], 5);
        assert_eq!(result.jump_table[4], 1);

        assert_eq!(result.jump_table[7], 14);
        assert_eq!(result.jump_table[13], 8);

        assert_eq!(result.jump_table[9], 12);
        assert_eq!(result.jump_table[11], 10);
    }

    #[test]
    fn test_run() {
        // hello.bf
        let language = Language::default();

        let ops = parse(&HELLO_BF.to_string(), &language);

        let mut data = [0; BUF_SIZE];
        let mut input = Cursor::new(vec![]);
        let mut output = Vec::new();

        let mut env = Environment::new(&mut data, &mut input, &mut output);

        run(&ops, &mut env);

        let output_string = from_utf8(&output[0..13]).expect("Encoding error");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn test_compress() {
        let source = "+++++[>>>----<<<[[..]],,]".to_string();
        let language = Language::default();

        let ops = parse(&source, &language);
        let compressed_ops = compress(&ops);

        assert_eq!(compressed_ops.ops, [
            CompressedOp::Add(5),
            CompressedOp::LoopStart,
            CompressedOp::Forward(3),
            CompressedOp::Sub(4),
            CompressedOp::Back(3),
            CompressedOp::LoopStart,
            CompressedOp::LoopStart,
            CompressedOp::PutChar,
            CompressedOp::PutChar,
            CompressedOp::LoopEnd,
            CompressedOp::LoopEnd,
            CompressedOp::GetChar,
            CompressedOp::GetChar,
            CompressedOp::LoopEnd,
        ]);

        assert_eq!(compressed_ops.jump_table[1], 14);
        assert_eq!(compressed_ops.jump_table[13], 2);

        assert_eq!(compressed_ops.jump_table[5], 11);
        assert_eq!(compressed_ops.jump_table[10], 6);

        assert_eq!(compressed_ops.jump_table[6], 10);
        assert_eq!(compressed_ops.jump_table[9], 7);
    }

    #[test]
    fn test_compress_run() {
        // hello.bf
        let language = Language::default();

        let ops = parse(&HELLO_BF.to_string(), &language);
        let compressed_ops = compress(&ops);

        let mut data = [0; BUF_SIZE];
        let mut input = Cursor::new(vec![]);
        let mut output = Vec::new();

        let mut env = Environment::new(&mut data, &mut input, &mut output);

        run(&compressed_ops, &mut env);

        let output_string = from_utf8(&output[0..13]).expect("Encoding error");
        assert_eq!(output_string, "Hello World!\n");
    }

    #[test]
    fn test_input() {
        // hello.bf
        let source = ",.,.,.".to_string();
        let language = Language::default();

        let ops = parse(&source, &language);
        let compressed_ops = compress(&ops);

        let mut data = [0; BUF_SIZE];
        let mut input = Cursor::new(vec![b'a', b'b', b'c']);
        let mut output = Vec::new();

        let mut env = Environment::new(&mut data, &mut input, &mut output);

        run(&compressed_ops, &mut env);

        let output_string = from_utf8(&output[0..3]).expect("Encoding error");
        assert_eq!(output_string, "abc");
    }

    #[test]
    fn test_language() {
        let lang = Language {
            inc: 'a',
            dec: 'b',
            inc_ptr: 'c',
            dec_ptr: 'd',
            put_char: 'e',
            get_char: 'f',
            loop_start: 'g',
            loop_end: 'h'
        };

        let source = "abcdefgh".to_string();
        let code = parse(&source, &lang);

        assert_eq!(code.ops, [
            Op::Inc,
            Op::Dec,
            Op::IncPtr,
            Op::DecPtr,
            Op::PutChar,
            Op::GetChar,
            Op::LoopStart,
            Op::LoopEnd,
        ]);
    }

    #[test]
    fn test_language_from_string() {
        let language = Language::make_from_string(&"abcdefgh".to_string());

        assert!(language.is_some());

        let language = language.unwrap();

        assert_eq!(language.inc, 'a');
        assert_eq!(language.dec, 'b');
        assert_eq!(language.inc_ptr, 'c');
        assert_eq!(language.dec_ptr, 'd');
        assert_eq!(language.get_char, 'e');
        assert_eq!(language.put_char, 'f');
        assert_eq!(language.loop_start, 'g');
        assert_eq!(language.loop_end, 'h');
    }

    #[test]
    fn test_environment_new() {
        let mut input = Cursor::new("");
        let mut output = vec![];
        let mut data = [0; 1024];

        let input = &mut input;
        let output = &mut output;

        let env = Environment::new(&mut data, input, output);

        assert_eq!(env.pointer, 0);
        assert_eq!(env.pc, 0);

        // TODO: Check reader, writer, and data
    }
}
