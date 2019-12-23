mod node;
mod packet;

use std::collections::HashMap;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::intcode::Program;
use crate::problem::Problem;

use node::Node;
use packet::Packet;

const NUM_NODES: usize = 50;

#[derive(Debug, StructOpt)]
pub struct Day23 {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

impl Problem for Day23 {
    type Output = i64;

    fn part_one(&self) -> i64 {
        let program = Program::read(&self.input);
        let mut nodes: Vec<Node> = (0..NUM_NODES)
            .map(|address| Node::new(&program, address))
            .collect();

        let mut packets: HashMap<usize, Vec<Packet>> = HashMap::new();

        loop {
            for (address, node) in nodes.iter_mut().enumerate() {
                let input = packets.remove(&address).unwrap_or_else(|| vec![]);

                for packet in node.run(&input) {
                    if packet.address() == 255 {
                        return packet.y();
                    }

                    packets
                        .entry(packet.address())
                        .or_insert_with(|| vec![])
                        .push(packet);
                }
            }
        }
    }

    fn part_two(&self) -> i64 {
        unimplemented!();
    }
}
