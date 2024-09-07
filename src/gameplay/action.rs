use super::tile::Tile;

#[derive(Default)]
pub struct Action {
    pub player: u8,
    pub action: EnumAction,
    pub act_tile: Tile,
    pub actee_tiles: Vec<Tile>,
}

#[derive(Default)]
pub enum EnumAction {
    Chi,
    Pon,
    Kang,
    Ron,
    Tsumo,
    Dahai,
    Reach,
    Ryuukyoku,
    #[default]
    None,
}
