use anyhow::anyhow;
use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::{map, map_res, opt},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::{Graph, dot::{Dot, Config}};

#[derive(PartialEq, Eq, Debug)]
struct Entry<'a> {
    name: &'a str,
    weight: u32,
    node_type: NodeType<'a>,
}

#[derive(PartialEq, Eq, Debug)]
enum NodeType<'a> {
    Leaf,
    Branch(Vec<&'a str>),
}

impl<'a> Entry<'a> {
    fn parse(input: &'a str) -> anyhow::Result<Entry<'a>> {
        let command: IResult<&str, Entry> = map(
            tuple((
                alpha1,
                space1,
                tag("("),
                map_res(digit1, |x| str::parse::<u32>(x)),
                tag(")"),
                opt(map(
                    tuple((
                        tag(" -> "),
                        separated_list0(tuple((tag(","), space1)), alpha1),
                    )),
                    |(_, values)| values,
                )),
            )),
            |parsed_input| {
                let name = parsed_input.0;
                let weight = parsed_input.3;
                let node_type = if let Some(values) = parsed_input.5 {
                    NodeType::Branch(values)
                } else {
                    NodeType::Leaf
                };
                Entry {
                    name,
                    weight,
                    node_type,
                }
            },
        )(input);
        command
            .map(|x| x.1)
            .map_err(|_| anyhow!("Unable to parse command!"))
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> anyhow::Result<String> {
    let mut graph: Graph<_, ()> = Graph::default();
    let mut nodes = FxHashMap::default();
    for line in input.lines() {
        let e = Entry::parse(line)?;
        let idx = if let Some(idx) = nodes.get(e.name) {
            *graph.node_weight_mut(*idx).unwrap() = (e.name, e.weight);
            *idx
        } else {
            let idx = graph.add_node((e.name, e.weight));
            nodes.insert(e.name, idx);
            idx
        };
        match e.node_type {
            NodeType::Leaf => {},
            NodeType::Branch(neighbors) => {
                for neighbor in neighbors {
                    let neighbor_idx = if let Some(idx) = nodes.get(neighbor) {
                        *idx
                    } else {
                        let idx = graph.add_node((neighbor, 0));
                        nodes.insert(neighbor, idx);
                        idx
                    };
                    graph.add_edge(idx, neighbor_idx, ());
                }
            }
        }
    }
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut root = nodes.iter().map(|x| *x.1).next().unwrap();
    while let Some(neigh) = graph.neighbors_directed(root, petgraph::Direction::Incoming).next() {
        root = neigh;
    }
    graph.node_weight(root)
        .map(|w| Ok(String::from(w.0)))
        .unwrap_or_else(|| Err(anyhow!("unable to find root of tree!")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_leaf() {
        assert_eq!(
            Entry::parse("abcdef (42)").unwrap(),
            Entry {
                name: "abcdef",
                weight: 42,
                node_type: NodeType::Leaf,
            }
        )
    }

    #[test]
    fn parse_branch() {
        assert_eq!(
            Entry::parse("abcdef (42) -> bcd, cde, def").unwrap(),
            Entry {
                name: "abcdef",
                weight: 42,
                node_type: NodeType::Branch(vec!["bcd", "cde", "def"])
            }
        )
    }

    static GIVEN: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn given_part1() {
        assert_eq!(part1(&GIVEN).unwrap(), "tknk")
    }
}
