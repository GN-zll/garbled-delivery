use mpz_circuits::{Circuit, CircuitBuilder, Node, Feed, Gate};
use std::sync::Arc;

pub fn add_delivery_layer(base_circuit: Arc<Circuit>) -> Arc<Circuit> {
    let mut builder = CircuitBuilder::new();
    
    let base_inputs: Vec<_> = (0..base_circuit.inputs().len())
    .map(|_| builder.add_input())
    .collect();
    
    let base_outputs = apply_circuit(&mut builder, &base_inputs, &base_circuit);

    let evaluator_share = compute_evaluator_share(&mut builder, &base_outputs);

    for out in evaluator_share {
        builder.add_output(out);
    }

    Arc::new(builder.build().unwrap())
}

pub fn compute_evaluator_share(
    builder: &mut CircuitBuilder,
    base_outputs: &[Node<Feed>],
) ->  Vec<Node<Feed>> {
    let garbler_share: Vec<_> = (0..base_outputs.len())
        .map(|_| builder.add_input())
        .collect();

    garbler_share
    .into_iter()
    .zip(base_outputs.iter().cloned())
    .map(|(a, b)| builder.add_xor_gate(a, b))
    .collect()
}

pub fn apply_circuit(
    builder: &mut CircuitBuilder,
    inputs: &[Node<Feed>],
    circuit: &Circuit,
) -> Vec<Node<Feed>> {
    let mut node_map = vec![builder.get_const_zero(); circuit.feed_count()];

    for (i, input) in circuit.inputs().clone().enumerate() {
        node_map[input] = inputs[i];
    }

    for gate in circuit.gates() {
        match gate {
            Gate::Xor { x, y, z } => {
                let out = builder.add_xor_gate(node_map[x.id()], node_map[y.id()]);
                node_map[z.id()] = out;
            }
            Gate::And { x, y, z } => {
                let out = builder.add_and_gate(node_map[x.id()], node_map[y.id()]);
                node_map[z.id()] = out;
            }
            Gate::Inv { x, z } => {
                let out = builder.add_inv_gate(node_map[x.id()]);
                node_map[z.id()] = out;
            }
            Gate::Id { x, z } => {
                let out = builder.add_id_gate(node_map[x.id()]);
                node_map[z.id()] = out;
            }
        }
    }

    circuit.outputs().clone().map(|o| node_map[o]).collect()
}