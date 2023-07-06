use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Component,
    Container,
    System,
    ExternalSystem,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub model_type: ModelType,
}

impl Element {
    pub fn new(name: &str, element_type: ModelType) -> Element {
        Element {
            name: name.to_owned(),
            model_type: element_type,
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
}

pub struct Project {
    pub name: String,
    pub graph: DiGraph<Element, Edge>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: name.to_owned(),
            graph: DiGraph::new(),
        }
    }

    pub fn add_node(&mut self, component: Element) -> NodeIndex {
        self.graph.add_node(component)
    }

    pub fn uses(&mut self, user_index: NodeIndex, usee_index: NodeIndex, description: &str) {
        self.graph.add_edge(
            user_index,
            usee_index,
            Edge {
                edge_type: EdgeType::Uses,
                description: description.to_owned(),
            },
        );
    }

    pub fn add_child(&mut self, parent: NodeIndex, child: NodeIndex) {
        self.graph.add_edge(
            parent,
            child,
            Edge {
                edge_type: EdgeType::IsParentOf,
                description: "is parent of".to_owned(),
            },
        );
        self.graph.add_edge(
            child,
            parent,
            Edge {
                edge_type: EdgeType::IsChildOf,
                description: "is child of".to_owned(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
