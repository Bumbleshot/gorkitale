pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

#[derive(PartialEq, Clone, Copy)]
pub enum Scene {
    Boot,
    LoginUsername,
    LoginPassword,
    Menu,
    TransitionToDesktop,
    Desktop,
    CombatTransition,
    Combat,
    Config,
    KernelPanic,
    AyasofyaInside,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Language {
    English,
    Turkish,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Front,
    Left,
    Right,
}
