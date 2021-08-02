use crate::intcode::{Computer, Program};

use super::packet::Packet;

pub struct Node {
    computer: Computer,
}

impl Node {
    pub fn new(program: &Program, address: usize) -> Self {
        let mut computer = program.launch();
        computer.run(&[address as i64]);

        Node { computer }
    }

    pub fn run(&mut self, input: &[Packet]) -> Vec<Packet> {
        let mut input: Vec<i64> = input
            .iter()
            .flat_map(|packet| vec![packet.x(), packet.y()])
            .collect();

        input.push(-1);

        let output = self.computer.run(&input);

        output.chunks(3).map(|chunk| Packet::from(chunk)).collect()
    }
}
