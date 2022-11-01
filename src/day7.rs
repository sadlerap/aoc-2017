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
use petgraph::{
    graph::NodeIndex,
    visit::{DfsPostOrder, Walker},
    Direction, Graph,
};

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

struct Tree<'a> {
    graph: Graph<(&'a str, u32), ()>,
    nodes: FxHashMap<&'a str, NodeIndex>,
}

impl<'a> Tree<'a> {
    fn parse(input: &'a str) -> anyhow::Result<Self> {
        let mut graph = Graph::default();
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
                NodeType::Leaf => {}
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

        Ok(Tree { graph, nodes })
    }

    pub fn root(&self) -> anyhow::Result<&str> {
        let mut root = self.nodes.iter().map(|x| *x.1).next().unwrap();
        while let Some(neigh) = self
            .graph
            .neighbors_directed(root, Direction::Incoming)
            .next()
        {
            root = neigh;
        }
        self.graph
            .node_weight(root)
            .map(|w| Ok(w.0))
            .unwrap_or_else(|| Err(anyhow!("unable to find root of tree!")))
    }

    pub fn outlier(&self) -> anyhow::Result<u32> {
        let mut subtree_weights: FxHashMap<NodeIndex, u32> = FxHashMap::default();
        let root_index = self.nodes[self.root()?];
        for node in DfsPostOrder::new(&self.graph, root_index).iter(&self.graph) {
            let outbound_neighbors = self.graph.neighbors_directed(node, Direction::Outgoing);
            let mut weights = FxHashMap::default();
            let total_children_weight: u32 = outbound_neighbors
                .into_iter()
                .map(|idx| {
                    let weight = subtree_weights[&idx];
                    weights.entry(weight).and_modify(|x| *x += 1).or_insert(1);
                    weight
                })
                .sum();

            if let Some(outlier_weight) = weights
                .iter()
                .find(|(_, &count)| count == 1)
                .map(|(weight, _)| weight)
            {
                // since we're doing a depth-first traversal, we know that the children of the
                // node witn an outlier weight are already balanced, so that node must have a
                // bad weight.
                let child_weight = subtree_weights
                    .iter()
                    .find(|(_, &v)| v == *outlier_weight)
                    .map(|(&k, _)| self.graph.node_weight(k).map(|(_, x)| x).unwrap())
                    .unwrap();
                let common_weight = weights
                    .iter()
                    .find(|(_, &count)| count > 1)
                    .map(|(&x, _)| x)
                    .unwrap();
                return Ok(child_weight + common_weight - outlier_weight);
            }
            subtree_weights.insert(
                node,
                total_children_weight + self.graph.node_weight(node).unwrap().1,
            );
        }

        Err(anyhow!("No weight outlier found!"))
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> anyhow::Result<String> {
    let tree = Tree::parse(input)?;
    tree.root().map(|x| String::from(x))
}

#[aoc(day7, part2)]
fn part2(input: &str) -> anyhow::Result<u32> {
    let tree = Tree::parse(input)?;
    tree.outlier()
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

    #[test]
    fn given_part2() {
        assert_eq!(part2(&GIVEN).unwrap(), 60)
    }
}
