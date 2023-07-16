#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Agent>,
    left_score: i32,
    left_last_action: Option<Action>,
    right: Box<dyn Agent>,
    right_score: i32,
    right_last_action: Option<Action>,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left,
            left_score: 0,
            left_last_action: None,
            right,
            right_score: 0,
            right_last_action: None,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_action = self.left.play(self.right_last_action);
        let right_action = self.right.play(self.left_last_action);

        self.left_last_action = Some(left_action);
        self.right_last_action = Some(right_action);
        
        let outcome = match (left_action, right_action) {
            (Action::Cooperate, Action::Cooperate) => RoundOutcome::BothCooperated,
            (Action::Cheat, Action::Cooperate) => RoundOutcome::LeftCheated,
            (Action::Cooperate, Action::Cheat) => RoundOutcome::RightCheated,
            (Action::Cheat, Action::Cheat) => RoundOutcome::BothCheated,
        };

        match outcome {
            RoundOutcome::BothCooperated => {
                self.left_score += 2;
                self.right_score += 2;
            }
            RoundOutcome::LeftCheated => {
                self.left_score += 3;
                self.right_score -= 1;
            }
            RoundOutcome::RightCheated => {
                self.left_score -= 1;
                self.right_score += 3;
            }
            RoundOutcome::BothCheated => (),
        }

        outcome
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Cooperate,
    Cheat,
}

pub trait Agent {
    fn play(&mut self, opponent_last_action: Option<Action>) -> Action;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn play(&mut self, _: Option<Action>) -> Action {
        Action::Cheat
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn play(&mut self, _: Option<Action>) -> Action {
        Action::Cooperate
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    was_betrayed: bool
}

impl Agent for GrudgerAgent {
    fn play(&mut self, opponent_last_action: Option<Action>) -> Action {
        if let Some(opponent_last_action) = opponent_last_action {
            if self.was_betrayed {
                Action::Cheat
            } else if opponent_last_action == Action::Cheat {
                self.was_betrayed = true;
                opponent_last_action
            } else {
                Action::Cooperate
            }
        } else {
            Action::Cooperate
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
}

impl Agent for CopycatAgent {
    fn play(&mut self, opponent_last_action: Option<Action>) -> Action {
        if let Some(opponent_last_action) = opponent_last_action {
            opponent_last_action
        } else {
            Action::Cooperate
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    round: usize,
    actions: [Action; 4],
    was_betrayed: bool,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        Self {
            round: 0,
            actions: [
                Action::Cooperate,
                Action::Cheat,
                Action::Cooperate,
                Action::Cooperate
            ],
            was_betrayed: false,
        }
    }
}

impl Agent for DetectiveAgent {
    fn play(&mut self, opponent_last_action: Option<Action>) -> Action {
        self.was_betrayed = self.was_betrayed 
            || opponent_last_action.is_some_and(|a| a == Action::Cheat);
        
        if self.round < self.actions.len() {
            let action = self.actions[self.round];
            self.round += 1;
            action
        }
        else if self.was_betrayed {
            opponent_last_action.unwrap()
        } else {
            Action::Cheat
        }
    }
}
