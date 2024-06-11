use std::fmt::Debug;
use std::collections::HashSet;
use marker::Usize1;
use proconio::*;

#[derive(Debug, PartialEq)]
enum NodeType {
    InLoop(usize),
    BranchRoot,
    Unknown
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    from: HashSet<usize>,
    to: usize
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::Unknown,
            from: HashSet::new(),
            to: 0
        }
    }
    pub fn is_in_loop(&self) -> bool {
        match self.node_type {
            NodeType::InLoop(_) => true,
            _ => false
        }
    }
    pub fn is_branch_root(&self) -> bool {
        match self.node_type {
            NodeType::BranchRoot => true,
            _ => false
        }
    }
}

fn calc(ans: &mut Vec<usize>, root: usize, nodes: &Vec<Node>) {
    ans[root] = ans[nodes[root].to] + 1;
    for &f in &nodes[root].from {
        calc(ans, f, nodes);
    }
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n]
    };
    let mut nodes: Vec<Node> = vec![];
    for _ in 0..n {
        nodes.push(Node::new());
    }
    for i in 0..n {
        nodes[i].to = a[i];
        nodes[a[i]].from.insert(i);
    }
    let mut loop_idx = 1;
    'a: for i in 0..n {
        if nodes[i].is_in_loop() {
            continue;
        }
        let mut work: Vec<usize> = nodes.iter().map(|node| {
            match node.node_type {
                NodeType::InLoop(x) => x,
                _ => 0
            }
        }).collect();
        let mut cur = i;
        work[cur] = loop_idx;
        'b: loop {
            let nex = nodes[cur].to;
            match work[nex] {
                0 => {
                    cur = nex;
                    work[cur] = loop_idx;
                },
                x if x < loop_idx => {
                    nodes[cur].node_type = NodeType::BranchRoot;
                    continue 'a;
                },
                _ => {
                    // find loop
                    let goal = cur;
                    let mut cur = nex;
                    nodes[goal].node_type = NodeType::InLoop(loop_idx);
                    while cur != goal {
                        nodes[cur].node_type = NodeType::InLoop(loop_idx);
                        cur = nodes[cur].to;
                    }
                    break 'b;
                }
            }
        }
        loop_idx += 1;
    }
    let nodes = nodes;
    let mut loop_size = vec![0usize; loop_idx - 1];
    for node in &nodes {
        match node.node_type {
            NodeType::InLoop(x) => {
                loop_size[x - 1] += 1;
            },
            _ => {}
        }
    }
    let mut ans: Vec<usize> = nodes.iter().map(|node| {
        match node.node_type {
            NodeType::InLoop(x) => {
                loop_size[x - 1]
            },
            _ => 0
        }
    }).collect();
    for i in 0..n {
        if nodes[i].is_branch_root() {
            calc(&mut ans, i, &nodes);
        }
    }
    let ans: usize = ans.iter().sum();
    println!("{}", ans);
}

#[allow(unused)]
fn print_vec<T: Debug>(v: &Vec<T>) {
    if v.len() == 0 {
        return;
    }
    print!("{:?}", v[0]);
    for e in &v[1..] {
        print!(" {:?}", e);
    }
    println!();
}
