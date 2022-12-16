use std::collections::HashMap;

use thiserror::Error;

use crate::types::{
    dns::{Label, Name},
    rr::Record,
};

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("No such parent")]
    NoSuchParent,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                index: 0,
                parent: None,
                nodes: HashMap::new(),
                data: Vec::new(),
            }],
        }
    }

    fn add_new_node(&mut self, parent_index: usize) -> usize {
        let index = self.nodes.len();

        self.nodes.push(Node {
            index,
            parent: Some(parent_index),
            nodes: HashMap::new(),
            data: Vec::new(),
        });

        index
    }

    pub fn insert(&mut self, name: Name, data: &mut Vec<Record>) -> Result<(), TreeError> {
        let labels = name.labels_rev();
        let mut current = 0;

        for label in labels {
            match self.nodes[current].nodes.get(&label) {
                Some(index) => current = *index,
                None => {
                    let child_node_index = self.add_new_node(current);

                    let node = self.find_node_by_index_mut(current).unwrap();
                    node.add_child_node(label, child_node_index);
                    current = child_node_index
                }
            }
        }

        let node = self.find_node_by_index_mut(current).unwrap();
        node.add_data(data);

        Ok(())
    }

    pub fn find_index(&self, name: Name) -> Option<usize> {
        let labels = name.labels_rev();
        let mut current = 0;

        for label in labels {
            let index = self.nodes[current].nodes.get(&label)?;
            current = *index
        }

        Some(current)
    }

    pub fn find_node(&self, name: Name) -> Option<&Node> {
        let index = self.find_index(name)?;
        self.find_node_by_index(index)
    }

    pub fn find_node_by_index(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn find_node_by_index_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }
}

#[derive(Debug)]
pub struct Node {
    index: usize,
    parent: Option<usize>,
    nodes: HashMap<Label, usize>,
    data: Vec<Record>,
}

impl Node {
    fn add_child_node(&mut self, label: Label, index: usize) {
        self.nodes.insert(label, index);
    }

    fn add_data(&mut self, data: &mut Vec<Record>) {
        self.data.append(data)
    }
}
