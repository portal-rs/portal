use std::collections::HashMap;

use thiserror::Error;

use crate::{Label, Name, Record};

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("No such parent")]
    NoSuchParent,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node<Record>>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: vec![Node {
                index: 0,
                parent: None,
                nodes: HashMap::new(),
                records: Vec::new(),
            }],
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> &Node<Record> {
        self.nodes.first().unwrap()
    }

    /// Inserts multiple resource `records` for a domain `name` into the tree.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// let mut tree = Tree::new();
    /// tree.insert_multi(Name::from("example.com"), vec![Record {}]);
    /// ```
    pub fn insert_multi(&mut self, name: Name, records: &mut Vec<Record>) -> Result<(), TreeError> {
        let labels = name.labels_rev();
        let mut current = 0;

        for label in labels {
            match self.nodes[current].nodes.get(&label) {
                Some(index) => current = *index,
                None => self.add_new_child_node(label, &mut current),
            }
        }

        let node = self.find_node_by_index_mut(current).unwrap();
        node.add_records(records);

        Ok(())
    }

    /// Inserts a resource `record` for a domain `name` into the tree.
    ///
    /// ### Example
    ///
    /// ```ignore
    /// let mut tree = Tree::new();
    /// tree.insert(Name::from("example.com"), Record {});
    /// ```
    pub fn insert(&mut self, name: Name, record: Record) -> Result<(), TreeError> {
        let labels = name.labels_rev();
        let mut current = 0;

        for label in labels {
            match self.nodes[current].nodes.get(&label) {
                Some(index) => current = *index,
                None => self.add_new_child_node(label, &mut current),
            }
        }

        let node = self.find_node_by_index_mut(current).unwrap();
        node.add_record(record);

        Ok(())
    }

    /// Finds the index of the domain `name`'s node.
    pub fn find_index(&self, name: Name) -> Option<usize> {
        let labels = name.labels_rev();
        let mut current = 0;

        for label in labels {
            let index = self.nodes[current].nodes.get(&label)?;
            current = *index
        }

        Some(current)
    }

    /// Finds a node by domain `name` and returns a reference to it.
    pub fn find_node(&self, name: Name) -> Option<&Node<Record>> {
        let index = self.find_index(name)?;
        self.find_node_by_index(index)
    }

    /// Finds a node by `index` and returns a reference to it.
    pub fn find_node_by_index(&self, index: usize) -> Option<&Node<Record>> {
        self.nodes.get(index)
    }

    /// Finds a node by `index` and returns a mutable reference to it.
    pub fn find_node_by_index_mut(&mut self, index: usize) -> Option<&mut Node<Record>> {
        self.nodes.get_mut(index)
    }

    fn add_new_node(&mut self, parent_index: usize) -> usize {
        let index = self.nodes.len();

        self.nodes.push(Node {
            index,
            parent: Some(parent_index),
            nodes: HashMap::new(),
            records: Vec::new(),
        });

        index
    }

    fn add_new_child_node(&mut self, label: Label, current: &mut usize) {
        // If there is no node for the current label, we need to
        // create a new one.
        let child_node_index = self.add_new_node(*current);

        // Create the child node and set the current node index
        // to the child node index.
        let node = self.find_node_by_index_mut(*current).unwrap();
        node.add_child_node(label, child_node_index);
        *current = child_node_index
    }
}

#[derive(Debug)]
pub struct Node<T> {
    nodes: HashMap<Label, usize>,
    parent: Option<usize>,
    records: Vec<T>,
    index: usize,
}

impl<T> Node<T> {
    pub fn records(&self) -> &Vec<T> {
        &self.records
    }

    pub fn children(&self) {}

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn has_records(&self) -> bool {
        !self.records.is_empty()
    }

    pub fn has_children(&self) -> bool {
        !self.nodes.is_empty()
    }

    fn add_child_node(&mut self, label: Label, index: usize) {
        self.nodes.insert(label, index);
    }

    fn add_records(&mut self, records: &mut Vec<T>) {
        self.records.append(records)
    }

    fn add_record(&mut self, record: T) {
        self.records.push(record)
    }
}
