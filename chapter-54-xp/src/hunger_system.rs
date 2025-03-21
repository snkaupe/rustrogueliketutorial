use specs::prelude::*;
use super::{HungerClock, RunState, HungerState, SufferDamage, gamelog::GameLog};

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, HungerClock>,
                        ReadExpect<'a, Entity>, // The player
                        ReadExpect<'a, RunState>,
                        WriteStorage<'a, SufferDamage>,
                        WriteExpect<'a, GameLog>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut hunger_clock, player_entity, runstate, mut inflict_damage, mut log) = data;

        for (entity, mut clock) in (&entities, &mut hunger_clock).join() {
            let mut proceed = false;

            match *runstate {
                RunState::PlayerTurn => {
                    if entity == *player_entity {
                        proceed = true;
                    }
                }
                RunState::MonsterTurn => {
                    if entity != *player_entity {
                        proceed = false;
                    }
                }
                _ => proceed = false
            }

            if proceed {
                clock.duration -= 1;
                if clock.duration < 1 {
                    match clock.state {
                        HungerState::WellFed => {
                            clock.state = HungerState::Normal;
                            clock.duration = 200;
                            if entity == *player_entity {
                                log.entries.insert(0, "You are no longer well fed.".to_string());
                            }
                        }
                        HungerState::Normal => {
                            clock.state = HungerState::Hungry;
                            clock.duration = 200;
                            if entity == *player_entity {
                                log.entries.insert(0, "You are hungry.".to_string());
                            }
                        }
                        HungerState::Hungry => {
                            clock.state = HungerState::Starving;
                            clock.duration = 200;
                            if entity == *player_entity {
                                log.entries.insert(0, "You are starving!".to_string());
                            }
                        }
                        HungerState::Starving => {
                            // Inflict damage from hunger
                            if entity == *player_entity {
                                log.entries.insert(0, "Your hunger pangs are getting painful! You suffer 1 hp damage.".to_string());
                            }
                            inflict_damage.insert(entity, SufferDamage{ amount: 1, from_player: false }).expect("Unable to do damage");  
                        }
                    }
                }
            }
        }
    }
}