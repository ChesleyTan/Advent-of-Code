use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::hash_map::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;

fn a() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("7.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut graph = HashMap::new();
    for line in contents.lines() {
        let tokens: Vec<_> = line.split(" ").collect();
        let dependency = tokens[1];
        let dependent = tokens[7];
        graph
            .entry(dependent)
            .or_insert(RefCell::new(Rc::new(Node::new(String::from(dependent)))));
        graph
            .entry(dependency)
            .or_insert(RefCell::new(Rc::new(Node::new(String::from(dependency)))));
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
    println!("{}", toposort(&graph));
    Ok(())
}

fn b() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("7.input")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let mut graph = HashMap::new();
    for line in contents.lines() {
        let tokens: Vec<_> = line.split(" ").collect();
        let dependency = tokens[1];
        let dependent = tokens[7];
        graph
            .entry(dependent)
            .or_insert(RefCell::new(Rc::new(Node::new(String::from(dependent)))));
        graph
            .entry(dependency)
            .or_insert(RefCell::new(Rc::new(Node::new(String::from(dependency)))));
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
    let mut workers = vec![
        Worker::new(),
        Worker::new(),
        Worker::new(),
        Worker::new(),
        Worker::new(),
    ];
    let num_tasks = graph.keys().len();
    let mut completed_tasks = String::new();
    let mut ready_tasks = BinaryHeap::new();
    for node_ref_cell in graph.values() {
        let node = node_ref_cell.borrow();
        if node.children.len() == 0 {
            ready_tasks.push(OrderedString::new(&node.name));
        }
    }
    let mut time_elapsed = 0;
    while completed_tasks.len() != num_tasks {
        // while there are tasks in the ready queue and there are workers available, assign tasks to workers
        while !ready_tasks.is_empty() && available_worker(&mut workers).is_some() {
            let mut worker = available_worker(&mut workers).unwrap();
            let task = ready_tasks.pop().unwrap();
            worker.task = Some(task.to_char());
            worker.time_remaining = task_duration(task.to_char());
        }
        // simulate a second
        let completed = simulate(&mut workers);
        time_elapsed += 1;
        // if any tasks are completed, make the worker available and add the task to the completed list
        for completed_task in completed {
            let mut completed_task_name = String::new();
            completed_task_name.push(completed_task);
            completed_tasks.push_str(&completed_task_name);
            for other in graph.values().filter(|o| o.borrow().name != completed_task_name) {
                let other_rc = &mut *(other.borrow_mut());
                let other_node = Rc::get_mut(other_rc).unwrap();
                let has_dependency = other_node
                    .children
                    .iter()
                    .any(|c| c.borrow().name == completed_task_name);
                if !has_dependency {
                    continue;
                }
                other_node
                    .children
                    .retain(|child| child.borrow().name != completed_task_name);
                // if the node has no more dependencies, then add it the the queue
                if other_node.children.len() == 0 {
                    ready_tasks.push(OrderedString::new(&other_node.name));
                }
            }
        }
    }
    println!("{}", time_elapsed);
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct OrderedString {
    str: String,
}

impl OrderedString {
    fn to_char(&self) -> char {
        self.str.chars().next().unwrap()
    }
}

#[derive(Debug)]
struct Worker {
    task: Option<char>,
    time_remaining: usize,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            task: None,
            time_remaining: 0,
        }
    }
}

impl OrderedString {
    fn new(s: &String) -> OrderedString {
        OrderedString {
            str: String::clone(s),
        }
    }
}

impl Ord for OrderedString {
    fn cmp(&self, other: &OrderedString) -> Ordering {
        // Increasing order
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for OrderedString {
    fn partial_cmp(&self, other: &OrderedString) -> Option<Ordering> {
        other.str.partial_cmp(&self.str)
    }
}

fn toposort(graph: &HashMap<&str, RefCell<Rc<Node>>>) -> String {
    let mut queue: BinaryHeap<OrderedString> = BinaryHeap::new();
    let mut deps_order = String::new();
    for node_ref_cell in graph.values() {
        let node = node_ref_cell.borrow();
        if node.children.len() == 0 {
            queue.push(OrderedString::new(&node.name));
        }
    }
    // loop through nodes in queue
    while let Some(node_name) = queue.pop() {
        let node = graph.get(node_name.str.as_str()).unwrap().borrow();
        deps_order.push_str(&node_name.str);
        // for every node that depends on the node, remove the child
        for other in graph.values().filter(|o| o.borrow().name != node_name.str) {
            let other_rc = &mut *(other.borrow_mut());
            let other_node = Rc::get_mut(other_rc).unwrap();
            let has_dependency = other_node
                .children
                .iter()
                .any(|c| c.borrow().name == node.name);
            if !has_dependency {
                continue;
            }
            other_node
                .children
                .retain(|child| child.borrow().name != node_name.str);
            // if the node has no more dependencies, then add it the the queue
            if other_node.children.len() == 0 {
                queue.push(OrderedString::new(&other_node.name));
            }
        }
    }
    deps_order
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
        self.children
            .sort_by_key(|&child| String::clone(&(*child.borrow()).name));
    }
}

fn available_worker(workers: &mut Vec<Worker>) -> Option<&mut Worker> {
    workers.iter_mut().find(|w| w.task.is_none())
}

fn task_duration(task: char) -> usize {
    ((task as u8) - ('A' as u8) + 61) as usize
}

fn simulate(workers: &mut Vec<Worker>) -> Vec<char> {
    let mut completed = vec![];
    for worker in workers.iter_mut().filter(|w| w.task.is_some()) {
        worker.time_remaining -= 1;
        if worker.time_remaining == 0 {
            completed.push(worker.task.unwrap());
            worker.task = None;
        }
    }
    completed
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
