use {Register, Instruction};

use nom::{digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Instruction {
    let result = instruction(line.as_bytes());

    match result {
        IResult::Done(_, instruction) => instruction,
        _                             => panic!("Invalid result {:?}", result)
    }
}

named!(a_register<Register>, map!(tag!("a"), |_| Register::A));
named!(b_register<Register>, map!(tag!("b"), |_| Register::B));
named!(register<Register>, alt!(a_register | b_register));

named!(number<usize>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(positive_offset<isize>, chain!(tag!("+") ~ value: number, || value as isize));
named!(negative_offset<isize>, chain!(tag!("-") ~ value: number, || 0 - value as isize));
named!(offset<isize>, alt!(positive_offset | negative_offset));

named!(half<Instruction>, chain!(
    tag!("hlf") ~ space ~ register: register,
    || Instruction::Half(register)
));

named!(triple<Instruction>, chain!(
    tag!("tpl") ~ space ~ register: register,
    || Instruction::Triple(register)
));

named!(increment<Instruction>, chain!(
    tag!("inc") ~ space ~ register: register,
    || Instruction::Increment(register)
));

named!(jump<Instruction>, chain!(
    tag!("jmp") ~ space ~ offset: offset,
    || Instruction::Jump(offset)
));

named!(jump_if_even<Instruction>, chain!(
    tag!("jie") ~ space ~ register: register ~ tag!(",") ~ space ~ offset: offset,
    || Instruction::JumpIfEven(register, offset)
));

named!(jump_if_one<Instruction>, chain!(
    tag!("jio") ~ space ~ register: register ~ tag!(",") ~ space ~ offset: offset,
    || Instruction::JumpIfOne(register, offset)
));

named!(instruction<Instruction>, alt!(half | triple | increment | jump | jump_if_even | jump_if_one));

#[cfg(test)]
mod tests {
    use super::instruction;

    use {Register, Instruction};
    use nom::IResult::Done;

    #[test]
    fn test_parse_half() {
        let result = instruction(b"hlf a");

        match result {
            Done(_, Instruction::Half(Register::A)) => (),
            _                                       => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_triple() {
        let result = instruction(b"tpl b");

        match result {
            Done(_, Instruction::Triple(Register::B)) => (),
            _                                         => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_increment() {
        let result = instruction(b"inc a");

        match result {
            Done(_, Instruction::Increment(Register::A)) => (),
            _                                            => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_jump() {
        let result = instruction(b"jmp +23");

        match result {
            Done(_, Instruction::Jump(23)) => (),
            _                              => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_jump_if_even() {
        let result = instruction(b"jie a, -23");

        match result {
            Done(_, Instruction::JumpIfEven(Register::A, -23)) => (),
            _                                                  => panic!("Incorrect result {:?}", result)
        };
    }

    #[test]
    fn test_parse_jump_if_one() {
        let result = instruction(b"jio b, +23");

        match result {
            Done(_, Instruction::JumpIfOne(Register::B, 23)) => (),
            _                                                => panic!("Incorrect result {:?}", result)
        };
    }
}
