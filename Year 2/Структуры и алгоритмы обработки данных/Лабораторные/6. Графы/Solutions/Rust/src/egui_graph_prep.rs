use egui_graphs::*;
use petgraph::graph::NodeIndex;
use eframe::{App, CreationContext, Frame};
use egui::Context;
use petgraph::prelude::StableGraph;
use petgraph::{Directed, Undirected};
use petgraph::visit::IntoEdges;
use crate::structs::Graph as MyGraph;

pub struct BasicApp {
    g: Graph<(), ()>
}
impl BasicApp {
    pub(crate) fn new(_: &CreationContext<'_>, my_graph: MyGraph) -> Self {
        let g = generate_graph(my_graph);
        let mut draw_g = Graph::from(&g);
        for idx in g.edge_indices() {
            let n = draw_g.edge_mut(idx).unwrap();
            n.set_label("".to_string());
        }
        Self { g: draw_g }
    }
}

fn generate_graph(graph: MyGraph) -> StableGraph<(), ()> {
    let mut g: StableGraph<(), ()> = StableGraph::new();
    let mut node_vec: Vec<NodeIndex> = vec![];
    for _ in 0..graph.vertex_count {
        let j = g.add_node(());
        node_vec.push(j);
    }
    for (idx, node) in node_vec.iter().enumerate(){
        for i_con in &graph.adjacency[idx] {
            let lhs = node;
            let rhs = node_vec[*i_con];
            g.add_edge(*lhs, rhs, ());
        }
    }
    g
}
impl App for BasicApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let interaction_settings = &SettingsInteraction::new()
                .with_dragging_enabled(true)
                .with_node_clicking_enabled(true)
                .with_node_selection_enabled(true)
                .with_node_selection_multi_enabled(true)
                .with_edge_clicking_enabled(true)
                .with_edge_selection_enabled(true)
                .with_edge_selection_multi_enabled(true);
            let style_settings = &SettingsStyle::new().with_labels_always(true);
            ui.add(
                &mut GraphView::<_, _, _, _, DefaultNodeShape, DefaultEdgeShape>::new(&mut self.g)
                    .with_styles(style_settings)
                    .with_interactions(interaction_settings),
            );
        });
    }
}