use derivative::Derivative;

pub mod game;
pub mod loading;

#[derive(Clone, Debug, PartialEq, Derivative)]
#[derivative(Default)]
pub enum RuntimeSystemState {
    #[derivative(Default)]
    Paused,
    Running,
}
