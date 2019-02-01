use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("7.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut graph: HashMap<&str, RefCell<Rc<Node>>> = HashMap::new();
    for line in contents.lines() {
        let tokens: Vec<_> = line.split(" ").collect();
        let dependency = tokens[1];
        let dependent = tokens[7];
        graph.entry(dependent).or_insert(RefCell::new(Rc::new(Node::new(String::from(dependent)))));
        graph.entry(dependency).or_insert(RefCell::new(Rc::new(Node::new(String::from(dependency)))));
    }
    for line in contents.lines() {
        let tokens: Vec<_> = line.split(" ").collect();
        let dependency = tokens[1];
        let dependent = tokens[7];
        let dependency_node = graph.get(dependency).unwrap();
        let dependent_node_rc = &mut *(graph.get(dependent).unwrap().borrow_mut());
        let dependent_node = Rc::get_mut(dependent_node_rc);
        dependent_node.unwrap().add_child(dependency_node);
    }
    let mut all_deps = String::new();
    for node in graph.values() {
        let mut deps_order = String::new();
        toposort(node.borrow(), &mut deps_order);
        println!("{}", deps_order);
        if deps_order.len() > all_deps.len() {
            all_deps = deps_order;
        }
    }
    println!("{:?}", all_deps);
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("7.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(())
}

fn toposort(start_node: Ref<Rc<Node>>, mut deps_order: &mut String) {
    for dep in start_node.children.iter() {
        let dep = dep.borrow();
        if deps_order.contains(&dep.name) {
            continue
        }
        toposort(dep, &mut deps_order);
    }
    deps_order.push_str(&start_node.name);
}

#[derive(Debug)]
struct Node<'a> {
    name: String,
    children: Vec<&'a RefCell<Rc<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn new(name: String) -> Node<'a> {
        Node {
            name,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: &'a RefCell<Rc<Node<'a>>>) {
        self.children.push(child);
        self.children.sort_by_key(|&child| String::clone(&(*child.borrow()).name));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part = env::args()
        .skip(1)
        .next()
        .expect("Expected argument specifying problem part `a` or `b`");
    if part == "a" {
        a()
    } else {
        b()
    }
}
