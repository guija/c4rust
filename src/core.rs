use std::fmt;

use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Graph;

pub type C4Graph = Graph<Element, Edge>;
pub type C4Index = NodeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Component,
    Container,
    System,
    ExternalSystem,
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub model_type: ModelType,
    pub description: String,
}

impl Element {
    pub fn new(name: &str, element_type: ModelType, description: &str) -> Element {
        Element {
            name: name.to_owned(),
            model_type: element_type,
            description: description.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeType {
    Uses,
    IsParentOf,
    IsChildOf,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub edge_type: EdgeType,
    pub description: String,
    pub asynchronous: bool,
}

pub struct Project {
    pub name: String,
    pub graph: C4Graph,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: name.to_owned(),
            graph: DiGraph::new(),
        }
    }

    pub fn container(&mut self, name: &str, description: &str) -> C4Index {
        self.add_node(Element::new(name, ModelType::Container, description))
    }

    pub fn add_node(&mut self, component: Element) -> C4Index {
        self.graph.add_node(component)
    }

    pub fn uses_sync(&mut self, user: C4Index, usee: C4Index, description: &str) {
        self.uses(user, usee, description, false)
    }

    pub fn uses_async(&mut self, user: C4Index, usee: C4Index, description: &str) {
        self.uses(user, usee, description, true)
    }

    fn uses(
        &mut self,
        user_index: C4Index,
        usee_index: C4Index,
        description: &str,
        asynchronous: bool,
    ) {
        self.graph.add_edge(
            user_index,
            usee_index,
            Edge {
                edge_type: EdgeType::Uses,
                description: description.to_owned(),
                asynchronous: asynchronous,
            },
        );
    }

    pub fn add_child(&mut self, parent: C4Index, child: C4Index) {
        self.graph.add_edge(
            parent,
            child,
            Edge {
                edge_type: EdgeType::IsParentOf,
                description: "is parent of".to_owned(),
                asynchronous: false,
            },
        );
        self.graph.add_edge(
            child,
            parent,
            Edge {
                edge_type: EdgeType::IsChildOf,
                description: "is child of".to_owned(),
                asynchronous: false,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
