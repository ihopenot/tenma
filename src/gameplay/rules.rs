use super::tile::Tile;

#[derive(Clone)]
pub struct RuleSet {
    pub init_rule: InitRule,       // init score, or sth.
    pub play_rules: Vec<PlayRule>, // 食替, 番缚 or sth.
    pub ron_rules: Vec<RonRule>,   // 胡牌方式
}

#[derive(Clone)]
pub struct InitRule {
    tile_set: Vec<Tile>,
    init_score: [i32; 4],
}

#[derive(Clone)]
pub struct PlayRule {}

#[derive(Clone)]
pub struct RonRule {}

impl InitRule {
    pub fn get_tile_set(&self) -> &Vec<Tile> {
        &self.tile_set
    }

    pub fn get_init_score(&self) -> &[i32; 4] {
        &self.init_score
    }
}
