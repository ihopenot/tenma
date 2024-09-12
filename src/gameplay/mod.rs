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

    use super::action::*;
    use super::state::*;
    use super::*;

    fn default_ruleset() -> RuleSet {
        let mut ruleset = RuleSet::default();
        ruleset
            .gamestart_rules
            .push(Box::new(BasePreGameStartInitScores {
                name: "gamestart_init_scores",
                scores: [25000, 25000, 25000, 25000],
            }));
        ruleset
            .gamestart_rules
            .push(Box::new(BasePostGameStartEnterTsumo {
                name: "gamestart_enter_tsumo",
            }));

        ruleset
    }

    #[test]
    fn test_gamestate() {
        let mut game = GameState::new(Some(default_ruleset()));
        let action = Action::start_game();
        assert!(game.step_action(action).is_ok());
        assert!(game.players[0].score == 25000);
    }
}
