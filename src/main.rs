fn main() {
    let mut input = GameControllerInput::default();
    input.buttons.distinct.move_up.ended_down = true;
    input.buttons.distinct.move_down.ended_down = true;
    input.buttons.distinct.move_left.ended_down = true;
    input.buttons.distinct.move_right.ended_down = true;
    input.buttons.distinct.action_up.ended_down = true;
    input.buttons.distinct.action_down.ended_down = true;
}
#[derive(Default, Copy, Clone)]
pub struct GameButtonState {
    pub half_tansition_count: i32,
    pub ended_down: bool,
}

pub union GameControllerButtons {
    pub all: [GameButtonState; 12],
    pub distinct: GameControllerDistinctButtons,
}

impl Default for GameControllerButtons {
    fn default() -> Self {
        Self {
            all: [GameButtonState::default(); 12]
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct GameControllerDistinctButtons {
    pub move_up: GameButtonState,
    pub move_down: GameButtonState,
    pub move_left: GameButtonState,
    pub move_right: GameButtonState,
    pub action_up: GameButtonState,
    pub action_down: GameButtonState,
    pub action_left: GameButtonState,
    pub action_right: GameButtonState,
    pub left_shoulder: GameButtonState,
    pub right_shoulder: GameButtonState,
    pub back: GameButtonState,
    pub start: GameButtonState,
    pub terminator: GameButtonState
}


#[derive(Default)]
pub struct GameControllerInput {
    pub is_connected: bool,
    pub is_analog: bool,
    pub stick_average_x: f32,
    pub stick_average_y: f32,
    pub buttons: GameControllerButtons,
}


