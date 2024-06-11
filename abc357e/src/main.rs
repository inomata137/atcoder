use std::fmt::Debug;
use std::collections::{HashSet, VecDeque};
use proconio::*;
use marker::Usize1;

#[derive(Debug, PartialEq)]
enum NodeType {
    InLoop,
    Branch,
    Unknown
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    from: HashSet<usize>,
    to: usize,
    group: usize
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::Unknown,
            from: HashSet::new(),
            to: 0,
            group: 0
        }
    }
    pub fn visited(&self) -> bool {
        match self.node_type {
            NodeType::Unknown => false,
            _ => true
        }
    }
}

fn dfs(root: usize, ans: &mut Vec<usize>, nodes: &Vec<Node>) -> usize {
    if ans[root] == 0 {
        ans[root] = dfs(nodes[root].to, ans, nodes) + 1;
    }
    ans[root]
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n]
    };
    let mut nodes: Vec<Node> = (0..n).map(|_| Node::new()).collect();
    for i in 0..n {
        nodes[i].to = a[i];
        nodes[a[i]].from.insert(i);
    }
    let mut group_id = 1usize;
    for i in 0..n {
        if nodes[i].visited() {
            continue;
        }
        // forward
        let mut cur = i;
        loop {
            match nodes[cur].node_type {
                NodeType::Unknown => {
                    nodes[cur].node_type = NodeType::Branch;
                    nodes[cur].group = group_id;
                    cur = nodes[cur].to;
                },
                NodeType::Branch => {
                    assert_eq!(nodes[cur].group, group_id);
                    break;
                },
                _ => panic!()
            }
        }
        // loop
        let mut queue = VecDeque::<usize>::new();
        loop {
            for &p in &nodes[cur].from {
                queue.push_back(p);
            }
            match nodes[cur].node_type {
                NodeType::Branch => {
                    nodes[cur].node_type = NodeType::InLoop;
                    cur = nodes[cur].to;
                },
                NodeType::InLoop => break,
                _ => panic!()
            }
        }
        // backward
        while let Some(x) = queue.pop_back() {
            match nodes[x].node_type {
                NodeType::InLoop => {},
                _ => {
                    nodes[x].node_type = NodeType::Branch;
                    nodes[x].group = group_id;
                    for &p in &nodes[x].from {
                        queue.push_back(p);
                    }
                }
            }
        }
        group_id += 1;
    }
    let nodes = nodes;
    let mut loop_size = vec![0usize; group_id - 1];
    for i in 0..n {
        match nodes[i].node_type {
            NodeType::InLoop => {
                loop_size[nodes[i].group - 1] += 1;
            },
            _ => {}
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        match nodes[i].node_type {
            NodeType::InLoop => {
                ans[i] = loop_size[nodes[i].group - 1]
            },
            _ => {}
        }
    }
    for i in 0..n {
        dfs(i, &mut ans, &nodes);
    }
    println!("{}", ans.iter().sum::<usize>());
}

#[allow(unused)]
fn print_vec<T: Debug>(v: &Vec<T>) {
    if v.len() == 0 {
        return;
    }
    println!("----------");
    for e in v {
        println!(" {:?}", e);
    }
    println!("----------");
}
