use std::collections::HashMap;
use std::iter::Cycle;
use std::rc::Rc;
use std::vec::IntoIter;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node(String);

impl Node {
    fn new(name: &str) -> Rc<Node> {
        return Rc::new(Node(name.to_string()));
    }
}

struct Network {
    steps: Vec<char>,
    step_cycle: Cycle<IntoIter<char>>,
    current_step: char,
    current_node: Rc<Node>,
    network: HashMap<Node, (Rc<Node>, Rc<Node>)>,
    num_of_steps: usize,
}

impl Network {
    fn new(input: &str) -> Self {
        let network_input: Vec<&str> = input.split("\n").collect();
        let steps: Vec<char> = network_input[0].chars().collect();
        let mut cycler = steps.clone().into_iter().cycle();
        let first_step = cycler.next().unwrap();

        let initial_node = Node::new("AAA");

        let mut new_network = Network {
            network: HashMap::new(),
            steps,
            step_cycle: cycler,
            current_step: first_step,
            current_node: initial_node,
            num_of_steps: 0,
        };

        for i in 2..network_input.len() {
            let node = network_input[i];
            let clean = node.replace("(", "").replace(")", "");
            let array: Vec<&str> = clean.split("=").map(|s| s.trim()).collect();
            let pair: Vec<&str> = array[1].split(", ").collect();
            let nodes = (Node::new(pair[0]), Node::new(pair[1]));

            new_network
                .network
                .insert(Node(array[0].to_string()), nodes);
        }

        return new_network;
    }

    fn next_step(&mut self) {
        self.current_step = self.step_cycle.next().unwrap();
    }

    fn move_to_next_node(&mut self) {
        let node = self.network.get(&self.current_node).unwrap();
        if self.current_step == 'L' {
            self.current_node = Rc::clone(&node.0);
        } else {
            self.current_node = Rc::clone(&node.1);
        }
        self.num_of_steps += 1;
        self.next_step();
    }

    fn get_to_final_node(&mut self) {
        loop {
            self.move_to_next_node();
            if self.current_node == Node::new("ZZZ") {
                break;
            }
        }
    }
}
fn main() {
    let input = include_str!("input.txt");
    let mut network = Network::new(input);
    network.get_to_final_node();

    println!("Number of steps: {}", network.num_of_steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_step() {
        let input = "RL\n\n\
            AAA = (BBB, CCC)\n \
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)";

        let mut network = Network::new(input);
        for _ in 0..3 {}
        assert_eq!(network.current_step, 'R');
    }

    #[test]
    fn move_to_the_correct_node() {
        let input = "RL\n\n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)";

        let mut network = Network::new(input);
        network.move_to_next_node();
        assert_eq!(network.current_node, Node::new("CCC"));
    }

    #[test]
    fn correct_number_of_steps() {
        let tests = vec![
            (
                "RL\n\n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)",
                2,
            ),
            (
                "LLR\n\n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)",
                6,
            ),
        ];

        for (input, expected) in tests {
            let mut network = Network::new(input);
            network.get_to_final_node();
            assert_eq!(network.num_of_steps, expected);
        }
    }
}
