#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TurnPhase {
    AwaitingPlayer,
    ResolvingActors,
    ResolvingEnvironment,
    RunEnded,
}
