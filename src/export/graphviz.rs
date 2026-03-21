use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    analysis::semantic_model::{Arc, Place, SemanticModel, Transition},
    args::Engine,
    grammar::ast::{Program, Statement},
};

pub struct Graphviz;

impl Graphviz {
    pub fn export(model: &SemanticModel, output_path: PathBuf, engine: &Engine) {
        let dot = Self::render_net(model);

        let extension: &str = output_path
            .extension()
            .map_or("png", |extension| extension.to_str().unwrap());

        let mut child = Command::new(engine.to_string())
            .arg(format!("-T{extension}"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!(
                "Failed to run {}, make sure you have Graphviz installed",
                engine.to_string()
            ));

        child
            .stdin
            .take()
            .expect("Failed to open stdin")
            .write_all(dot.as_bytes())
            .expect("Failed to write to stdin");

        let output = child.wait_with_output().expect("Failed to read output");

        fs::write(output_path, output.stdout).expect("Failed to write output file");
    }

    fn render_places(places: impl Iterator<Item = &Place>) -> String {
        places
            .map(|place| {
                let xlabel = place.label().as_ref().map_or_else(
                    || place.name().to_owned(),
                    |label| format!("{}: {}", &place.name(), &label),
                );

                format!(
                    "{} [shape=circle, label=\"{}\", xlabel=\"{}\"];",
                    &place.name(),
                    &place.tokens(),
                    &xlabel
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_transitions(transitions: impl Iterator<Item = &Transition>) -> String {
        transitions
            .map(|transition| {
                let xlabel = transition.label().as_ref().map_or_else(
                    || transition.name().to_owned(),
                    |label| format!("{}: {}", &transition.name(), &label),
                );

                format!("{xlabel} [shape=box, width=0.15, height=0.8, fixedsize=true];",)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_arcs(arcs: impl Iterator<Item = &Arc>) -> String {
        arcs.map(|arc| {
            let label = if *arc.weight() == 1 {
                String::new()
            } else {
                format!("[label=\"{}\"]", arc.weight())
            };

            format!("{} -> {} {};", arc.left(), arc.right(), label)
        })
        .collect::<Vec<String>>()
        .join("\n")
    }

    fn render_net(model: &SemanticModel) -> String {
        let places = Self::render_places(model.places().iter());
        let transitions = Self::render_transitions(model.transitions().iter());
        let arcs = Self::render_arcs(model.arcs().iter());
        let graph = format!("{places}\n{transitions}\n{arcs}");

        format!(
            r"
            digraph Petri {{
                graph [
                dpi=300,
                splines=true,
                nodesep=0.6,
                ranksep=1.0
                ];
                rankdir=LR;
                {graph}
            }}"
        )
    }
}
