use petgraph::{dot::Dot, graph, stable_graph::NodeIndex, Graph};

use crate::core::{self, Edge, EdgeType, Element, Project};

pub fn default_dot(project: &core::Project) -> String {
    format!("{:?}", Dot::with_config(&project.graph, &[]))
}

type C4Graph = Graph<Element, Edge>;
type C4Index = NodeIndex;

pub fn dot(project: &Project, element_index: C4Index) {
    let graph = &project.graph;
    let element = graph.node_weight(element_index);
    let children = children(graph, element_index);
    for child_index in children {
        let child = project.graph.node_weight(child_index);
        println!("{:?} is child of {:?}", child, element);
        let parent = parent(graph, child_index);
        println!("{:?} is parent of {:?}", parent, child);
    }
}

fn children(graph: &C4Graph, element_index: C4Index) -> Vec<C4Index> {
    neighbours_with_directed_edge_type(graph, element_index, EdgeType::IsChildOf)
}

fn parent(graph: &C4Graph, element_index: C4Index) -> Option<C4Index> {
    neighbours_with_directed_edge_type(graph, element_index, EdgeType::IsParentOf)
        .first()
        .cloned()
}

fn neighbours_with_directed_edge_type(
    graph: &C4Graph,
    from: C4Index,
    edge_type: EdgeType,
) -> Vec<C4Index> {
    graph
        .neighbors(from)
        .filter(|neighbour| {
            graph
                .edges_connecting(*neighbour, from)
                .any(|x| x.weight().edge_type == edge_type)
        })
        .collect()
}
