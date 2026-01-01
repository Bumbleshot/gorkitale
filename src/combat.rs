#[derive(PartialEq, Clone, Copy)]
pub enum CombatTurn {
    Menu,
    Fighting,
    Acting,
    Mercy,
    SansTurn,
}

pub struct CombatData {
    pub player_hp: i32,
    pub player_max_hp: i32,
    #[allow(dead_code)]
    pub sans_hp: i32,
    #[allow(dead_code)]
    pub sans_max_hp: i32,
    pub turn: CombatTurn,
    pub menu_selection: usize, // 0: Fight, 1: Act, 2: Mercy
    #[allow(dead_code)]
    pub sub_menu_selection: usize,
    pub dialogue_text: String,
    pub action_text: String,
    pub timer: f32,
    pub sans_shake: f32,
}

impl CombatData {
    pub fn new() -> Self {
        CombatData {
            player_hp: 20,
            player_max_hp: 20,
            sans_hp: 1,
            sans_max_hp: 1,
            turn: CombatTurn::Menu,
            menu_selection: 0,
            sub_menu_selection: 0,
            dialogue_text: "You feel like you're gonna have a bad time.".to_string(),
            action_text: String::new(),
            timer: 0.0,
            sans_shake: 0.0,
        }
    }
}
