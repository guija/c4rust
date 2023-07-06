use std::collections::HashSet;

use petgraph::{dot::Dot, stable_graph::NodeIndex, Graph};

use crate::core::{self, Edge, EdgeType, Element, ModelType, Project};

pub fn default_dot(project: &core::Project) -> String {
    format!("{:?}", Dot::with_config(&project.graph, &[]))
}

type C4Graph = Graph<Element, Edge>;
type C4Index = NodeIndex;

pub fn dot_container_view(project: &Project) {
    // Gehe ueber alle Knoten welche container sind und fuege sie zum graph hinzu

    println!("digraph {{");

    let graph = &project.graph;

    let containers: HashSet<NodeIndex> = graph
        .node_indices()
        .filter(|idx| graph.node_weight(*idx).unwrap().model_type == ModelType::Container)
        .collect();

    // add nodes to dot

    for container_index in &containers {
        let container = graph.node_weight(*container_index).unwrap();
        // TODO GJA move this into a method because we will need it in a lot of places.
        println!(
            "\t{} [label=\"{}\"]",
            container_index.index(),
            container.name
        );
    }

    // get all the edges that go either from a container to another container or from a container into an external system
    let edges = graph.edge_indices().filter(|idx| {
        let (start_idx, end_idx) = graph.edge_endpoints(*idx).unwrap();
        let start_node = graph.node_weight(start_idx).unwrap();
        let end_node = graph.node_weight(end_idx).unwrap();
        // TODO GJA what about containers in external system (get parent and check whether parent is an external system)
        // TODO GJA research hierarchical enum Container(db,queue,service,job), system(external,internal)
        let container_uses_container =
            containers.contains(&start_idx) && containers.contains(&end_idx);
        let container_to_external =
            containers.contains(&start_idx) && end_node.model_type == ModelType::ExternalSystem;
        let external_uses_container =
            start_node.model_type == ModelType::ExternalSystem && containers.contains(&end_idx);
        container_uses_container || container_to_external || external_uses_container
    });

    // Add edges to graph
    for idx in edges {
        let (start_idx, end_idx) = graph.edge_endpoints(idx).unwrap();
        let edge = graph.edge_weight(idx).unwrap();
        println!(
            "\t{} -> {} [label=\"{}\"]",
            start_idx.index(),
            end_idx.index(),
            edge.description
        );
    }

    println!("}}");
}

fn background_color(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Container => "438DD5",
        ModelType::Component => "438DD5",
        ModelType::ExternalSystem => "999999",
        ModelType::System => "1168BD",
    }
}

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
