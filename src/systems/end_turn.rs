use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    // TODO: Can refactor this so TurnState implements an advance() method. Validity of it
    // is done in struct instead of in the system, which just controls the calling of it
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}
