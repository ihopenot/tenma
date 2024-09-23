mod action;
mod baserules;
mod effect;
mod enums;
mod rules;
pub mod state;
mod tile;

use baserules::BasePreGameStartInitScores;
use bevy::prelude::*;
use rules::*;

pub fn gameplay_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    setup_rules(&mut commands);
}

fn setup_rules(commands: &mut Commands) {
    let mut rule_dict = RuleDict::default();
    rule_dict.set_rule(Box::new(BasePreGameStartInitScores {
        name: "set_scores",
        scores: [25000, 25000, 25000, 25000],
    }));
    commands.insert_resource(rule_dict);
}

#[cfg(test)]
mod tests {
    use baserules::*;
    use enums::EnumGameState;

    use crate::tu8;
    use crate::tuz;

    use super::action::*;
    use super::state::*;
    use super::tile::*;
    use super::*;

    fn default_ruleset() -> RuleSet {
        let mut ruleset = RuleSet::default();
        ruleset
            .pre_gamestart_rules
            .push(Box::new(BasePreGameStartInitScores {
                name: "gamestart_init_scores",
                scores: [25000, 25000, 25000, 25000],
            }));

        ruleset
            .post_kyokustart_rules
            .push(Box::new(BasePostKyokuStartEnterTsumo {
                name: "kyokustart_enter_tsumo",
            }));

        ruleset
            .pre_tsumo_rules
            .push(Box::new(BasePreTsumoRandomTsumo {
                name: "pre_tsumo_random_tsumo",
            }));

        ruleset
            .pre_tsumo_rules
            .push(Box::new(BasePreTsumoStateCheck {
                name: "pre_tsumo_state_check",
            }));

        let mut tiles = Vec::new();
        for tile in 0..tu8!(all) {
            if tile == tu8!(5pr) || tile == tu8!(5sr) || tile == tu8!(5mr) {
                continue;
            }
            tiles.push(Tile(tile));
        }
        ruleset
            .pre_kyokustart_rules
            .push(Box::new(BasePreKyokuStartSetYama {
                name: "pre_kyoku_start_set_yama",
                tiles,
            }));

        ruleset
    }

    #[test]
    fn test_gamestate() {
        let mut game = GameState::new();
        let rules = default_ruleset();
        assert!(game.step_action(Action::start_game(), &rules).is_ok());
        assert!(game.players[0].score == 25000);

        assert!(game.step_action(Action::start_kyoku(), &rules).is_ok());
        assert!(game.yama.len() == 136);
        assert!(game.current_player == 0);
        assert!(game.kyoku == 0);
        assert!(game.state == EnumGameState::Dahai);
        assert!(game.players[0].tehai.len() == 13);
    }
}
