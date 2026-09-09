#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnPhase {
    AwaitingPlayer,
    ResolvingActors,
    ResolvingEnvironment,
    RunEnded,
}
