use super::tile::Tile;

#[derive(Default, Clone)]
pub struct Action {
    pub player: u8,
    pub act_type: EnumAction,
    pub act_tile: Tile,
    pub actee_tiles: Vec<Tile>,
}

#[derive(Default, PartialEq, Clone)]
pub enum EnumAction {
    Chi,
    Pon,
    Kang,
    Ron,
    Tsumo,
    Dahai,
    Reach,
    Ryuukyoku,
    GameStart,
    KyokuStart,
    #[default]
    None,
}

impl Action {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn new(player: u8, act_type: EnumAction, act_tile: Tile, actee_tiles: Vec<Tile>) -> Self {
        Self {
            player,
            act_type,
            act_tile,
            actee_tiles,
        }
    }

    pub fn start_game() -> Self {
        Self {
            act_type: EnumAction::GameStart,
            ..Default::default()
        }
    }
}
