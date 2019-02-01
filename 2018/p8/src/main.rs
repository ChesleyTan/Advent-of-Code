use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("8.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut tree_desc: Vec<usize> = contents.trim().split(" ").map(parse_usize).collect();
    tree_desc = tree_desc.iter().cloned().rev().collect();
    let mut nodes = vec![Node::new(0)];
    parse(0, &mut tree_desc, &mut nodes);
    println!("{}", sum_tree(0, &nodes));
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("8.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut tree_desc: Vec<usize> = contents.trim().split(" ").map(parse_usize).collect();
    tree_desc = tree_desc.iter().cloned().rev().collect();
    let mut nodes = vec![Node::new(0)];
    parse(0, &mut tree_desc, &mut nodes);
    println!("{}", value_tree(0, &nodes));
    Ok(())
}

#[derive(Debug)]
struct Node {
    id: usize,
    children: Vec<usize>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(id: usize) -> Node {
        Node {
            id,
            children: vec![],
            metadata: vec![],
        }
    }
}

fn parse(root: usize, tree_desc: &mut Vec<usize>, nodes: &mut Vec<Node>) {
    let num_children = tree_desc.pop().unwrap();
    let num_metadata = tree_desc.pop().unwrap();
    let mut new_children = vec![];
    for _ in 0..num_children {
        nodes.push(Node::new(nodes.len()));
        let child = nodes.last().unwrap();
        new_children.push(child.id);
        parse(child.id, tree_desc, nodes);
    }
    let root_node = nodes.get_mut(root).unwrap();
    root_node.children = new_children;
    for _ in 0..num_metadata {
        root_node.metadata.push(tree_desc.pop().unwrap());
    }
}

fn sum_tree(root: usize, nodes: &Vec<Node>) -> usize {
    let root_node = nodes.get(root).unwrap();
    let mut sum = root_node.metadata.iter().sum();
    for &child_id in root_node.children.iter() {
        sum += sum_tree(child_id, nodes);
    }
    sum
}

fn value_tree(root: usize, nodes: &Vec<Node>) -> usize {
    let root_node = nodes.get(root).unwrap();
    if root_node.children.is_empty() {
        root_node.metadata.iter().sum()
    } else {
        let mut total = 0;
        for &child_idx in root_node.metadata.iter() {
            if child_idx != 0 && child_idx <= root_node.children.len() {
                let &child_id = root_node.children.get(child_idx - 1).unwrap();
                total += value_tree(child_id, nodes);
            }
        }
        total
    }
}

fn parse_usize(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let part = env::args()
        .nth(1)
        .expect("Expected argument specifying problem part `a` or `b`");
    if part == "a" {
        a()
    } else {
        b()
    }
}
