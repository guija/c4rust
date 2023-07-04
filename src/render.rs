use petgraph::{dot::Dot, stable_graph::NodeIndex};

use crate::core::{self, EdgeType, Project};

pub fn default_dot(project: &core::Project) -> String {
    format!("{:?}", Dot::with_config(&project.graph, &[]))
}

pub fn dot(project: &Project, element_index: NodeIndex) {
    let element = project.graph.node_weight(element_index);

    let children: Vec<NodeIndex> = project
        .graph
        .neighbors(element_index)
        .map(|neighbour| {
            println!("neighbour {:?}", neighbour);
            neighbour
        })
        .filter(|neighbour| {
            project
                .graph
                .edges_connecting(element_index, *neighbour)
                .any(|x| x.weight().edge_type == EdgeType::IsParentOf)
        })
        .collect();

    for child_index in children {
        let child = project.graph.node_weight(child_index);
        println!("{:?} is child of {:?}", child, element)
    }
}
