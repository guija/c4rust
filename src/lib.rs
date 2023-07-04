use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;

#[derive(Debug, Clone, Copy)]
pub enum ModelType {
    Component,
    Container,
    System,
}

#[derive(Debug, Clone)]
pub struct Element {
    name: String,
    model_type: ModelType,
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
    edge_type: EdgeType,
    description: String,
}

pub struct Project {
    name: String,
    graph: DiGraph<Element, Edge>,
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

    pub fn default_dot(&self) -> String {
        format!("{:?}", Dot::with_config(&self.graph, &[]))
    }

    pub fn dot(&self, element_index: NodeIndex) {
        let element = self.graph.node_weight(element_index);

        let children: Vec<NodeIndex> = self
            .graph
            .neighbors(element_index)
            .map(|neighbour| {
                println!("neighbour {:?}", neighbour);
                neighbour
            })
            .filter(|neighbour| {
                self.graph
                    .edges_connecting(element_index, *neighbour)
                    .any(|x| x.weight().edge_type == EdgeType::IsParentOf)
            })
            .collect();

        for child_index in children {
            let child = self.graph.node_weight(child_index);
            println!("{:?} is child of {:?}", child, element)
        }
    }

    pub fn lookup_node(&self, node_index: NodeIndex) -> Option<&Element> {
        self.graph.node_weight(node_index)
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
