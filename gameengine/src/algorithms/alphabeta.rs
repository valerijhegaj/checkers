use super::*;

pub struct MinMaxAlphaBeta {
    depth: u8,
}

impl MinMaxAlphaBeta {
    pub fn predict<Edge, RecoverState, TerminalState, S: Score, Player>(
        &self,
        node: &mut (impl Node<Edge, RecoverState, TerminalState> + Evaluator<TerminalState, S, Player>),
        player: &Player,
    ) -> Option<Edge> {
        let mut best_edge = None;
        let mut best_score = S::min();

        for edge in node.edge_to_children() {
            let state = node.apply(&edge);
            let value = Self::alpha_beta(node, player, self.depth - 1, S::min(), S::max(), false);
            node.unapply(&edge, state);

            if value >= best_score {
                best_score = value;
                best_edge = Some(edge);
            }
        }

        best_edge
    }

    fn alpha_beta<Edge, State, TerminalState, S: Score, Player>(
        node: &mut (impl Node<Edge, State, TerminalState> + Evaluator<TerminalState, S, Player>),
        player: &Player,
        depth: u8,
        mut alpha: S,
        mut beta: S,
        is_maximizing: bool,
    ) -> S {
        let edges = node.edge_to_children();

        let (terminal, terminal_state) = node.terminal(&edges);
        if terminal {
            return node.evaluate_terminal(terminal_state, player);
        }

        if depth == 0 {
            return node.evaluate(player);
        }

        if is_maximizing {
            let mut value = S::min();
            for edge in edges {
                let state = node.apply(&edge);
                let new_value = Self::alpha_beta(node, &player, depth - 1, alpha, beta, false);
                node.unapply(&edge, state);

                if new_value > value {
                    value = new_value;
                }

                if value > alpha {
                    alpha = value;
                }

                if beta <= alpha {
                    break;
                }
            }

            return value;
        }

        let mut value = S::max();
        for edge in edges {
            let state = node.apply(&edge);
            let new_value = Self::alpha_beta(node, &player, depth - 1, alpha, beta, true);
            node.unapply(&edge, state);

            if new_value < value {
                value = new_value;
            }

            if value < beta {
                beta = value;
            }

            if beta <= alpha {
                break;
            }
        }

        return value;
    }
}
