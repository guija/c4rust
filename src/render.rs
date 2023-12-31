use std::collections::HashSet;

use petgraph::dot::Dot;

use crate::core::{self, C4Graph, C4Index, EdgeType, ModelType, Project};

pub fn debug_dot(project: &core::Project) -> String {
    format!("{:?}", Dot::with_config(&project.graph, &[]))
}

pub fn dot_container_view(project: &Project) {
    println!("digraph {{");
    println!("graph [pad=\"0.5\", nodesep=\"2\", ranksep=\"2\"];");
    println!("splines=\"true\";");
    println!("splines=ortho;");

    let graph = &project.graph;

    let containers: HashSet<C4Index> = graph
        .node_indices()
        .filter(|idx| graph.node_weight(*idx).unwrap().model_type == ModelType::Container)
        .collect();

    for container_index in &containers {
        let container = graph.node_weight(*container_index).unwrap();
        // TODO GJA move registering nodes in dot into a method because we will need it in a lot of places.
        let shape = "box"; // TODO GJA introduce method, but before that we need group of containers
        let href = ""; // TODO GJA reference to container view
        let description = split_into_multiple_lines_max_n_words_per_line(&container.description, 3);
        let dot_node_string = format!("\t\"{}\" [label=<<B>{}</B><BR/>[{}]<BR/><BR/>{}> shape={} fontname=Helvetica fontsize=12 margin=\"0.3,0.1\" fillcolor=\"#{}\" color=\"#{}\" fontcolor=white style=filled {}]", container_index.index(), container.name, container.model_type.to_string(), description, shape, background_color(container.model_type), border_color(container.model_type), href);
        println!("{}", dot_node_string);
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
        let edge_style = "solid"; // TODO GJA introduce notion of async with "dashed" style
        let dot_edge = format!("\"{}\" -> \"{}\" [xlabel=\"{}\" style={} fontname=Helvetica fontsize=11 color=\"#707070\"]", start_idx.index(), end_idx.index(), edge.description, edge_style);
        println!("{}", dot_edge);
    }

    println!("}}");
}

fn split_into_multiple_lines_max_n_words_per_line(text: &str, max_words_per_line: usize) -> String {
    let mut description = String::new();
    for (i, word) in text.split_whitespace().enumerate() {
        description.push_str(word);
        if (i + 1) % max_words_per_line == 0 {
            description.push_str("<BR/>");
        } else {
            description.push_str(" ");
        }
    }
    description
}

fn background_color(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Container => "438DD5",
        ModelType::Component => "438DD5",
        ModelType::ExternalSystem => "999999",
        ModelType::System => "1168BD",
    }
}

fn border_color(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Container => "3C7FC0",
        ModelType::Component => "3C7FC0",
        ModelType::ExternalSystem => "8A8A8A",
        ModelType::System => "0F5EAA",
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
