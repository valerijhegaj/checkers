pub trait Node<Edge, RecoverState, TerminalState> {
    fn terminal(&self, edges: &Vec<Edge>) -> (bool, TerminalState);

    fn edge_to_children(&self) -> Vec<Edge>;
    fn apply(&mut self, edge: &Edge) -> RecoverState;
    fn unapply(&mut self, edge: &Edge, state: RecoverState);
}

pub trait Score: PartialOrd + Copy {
    fn max() -> Self;
    fn min() -> Self;
}

pub trait Evaluator<TS, S: Score, Player> {
    fn evaluate(&self, player: &Player) -> S;
    fn evaluate_terminal(&self, terminal_state: TS, player: &Player) -> S;
}

mod alphabeta;
pub use alphabeta::MinMaxAlphaBeta;

#[inline(always)]
fn min<S: Score>(left: S, right: S) -> S {
    if left < right {
        return left;
    }
    right
}

#[inline(always)]
fn max<S: Score>(left: S, right: S) -> S {
    if left > right {
        return left;
    }
    right
}
