use crate::fix::Fix;
use crate::fx;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Character {
    Mario,
    Luigi,
    Wario,
    Yoshi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum State {
    Wait,
    Jump,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Player {
    character: Character,
    position_y: Fix,
    velocity_y: Fix,
    vert_accel: Fix,
    terminal_velocity: Fix,
    triple_jump: bool,
    can_flutter_jump: bool,
    is_flutter_jumping: bool,
    state: State,
}

impl Player {
    pub const VERT_ACCEL: f64 = -4.0;
    pub const TERMINAL_VELOCITY: f64 = -75.0;

    pub const JUMP_FACTORS: [f64; 4] = [
        1.0,
        1.0,
        0.81982421875, // 3358/4096
        0.89990234375, // 3686/4096
    ];
    pub const JUMP_UP_VERT_ACCEL: f64 = -8.0;
    pub const JUMP_UP_VERT_ACCEL_HOLDING_B: f64 = -3.25;
    pub const JUMP_UP_VERT_ACCEL_YOSHI_HOLDING_B: f64 = -3.0;

    pub const FLUTTER_JUMP_MAX_START_VERT_SPEED: f64 = -8.0;
    pub const FLUTTER_JUMP_UP_VERT_ACCELERATION: f64 = 1.0;
    pub const FLUTTER_JUMP_DOWN_VERT_ACCELERATION: f64 = 0.75;
    pub const FLUTTER_JUMP_MAX_VERT_SPEED: f64 = 17.0;

    pub const GROUND_POUND_ASCENT: f64 = 64.0;
    pub const GROUND_POUND_INIT_VEL: f64 = -50.0;
    
    pub fn new(character: Character, position_y: Fix, horz_speed: Fix, jump_index: usize) -> Self {
        Self {
            character,
            position_y,
            velocity_y: (fx!([42.0, 52.0, 69.0][jump_index]) + (horz_speed >> 2)) * fx!(Self::JUMP_FACTORS[character as usize]),
            vert_accel: fx!(Self::VERT_ACCEL),
            terminal_velocity: fx!(Self::TERMINAL_VELOCITY),
            triple_jump: jump_index == 2,
            can_flutter_jump: true,
            is_flutter_jumping: false,
            state: State::Wait,
        }
    }

    pub fn position_y(&self) -> Fix {
        self.position_y
    }

    pub fn velocity_y(&self) -> Fix {
        self.velocity_y
    }

    pub fn update_jump(&mut self, holding_b: bool) {
        self.vert_accel = if self.velocity_y < fx!(0.0) {
            fx!(Self::VERT_ACCEL)
        } else if holding_b {
            fx!(Self::JUMP_UP_VERT_ACCEL_HOLDING_B)
        } else {
            fx!(Self::JUMP_UP_VERT_ACCEL)
        }
    }

    pub fn update_jump_luigi(&mut self, holding_b: bool) {
        self.vert_accel = if self.velocity_y < fx!(0.0) {
            if holding_b {
                fx!(Self::VERT_ACCEL) >> 2
            } else {
                fx!(Self::VERT_ACCEL)
            }
        } else if holding_b {
            fx!(Self::JUMP_UP_VERT_ACCEL_HOLDING_B)
        } else {
            fx!(Self::JUMP_UP_VERT_ACCEL)
        }
    }

    pub fn update_jump_yoshi(&mut self, holding_b: bool) {
        if self.is_flutter_jumping {
            if holding_b {
                self.velocity_y += if self.velocity_y >= fx!(0.0) {
                    fx!(Self::FLUTTER_JUMP_UP_VERT_ACCELERATION)
                } else {
                    fx!(Self::FLUTTER_JUMP_DOWN_VERT_ACCELERATION)
                }
            }

            if !holding_b || self.velocity_y >= fx!(Self::FLUTTER_JUMP_MAX_VERT_SPEED) {
                self.is_flutter_jumping = false;
                self.can_flutter_jump = false;
            }

        } else {
            self.vert_accel = if self.velocity_y < fx!(0.0) {
                90 * fx!(Self::VERT_ACCEL) / 100
            } else if holding_b {
                90 * fx!(Self::JUMP_UP_VERT_ACCEL_YOSHI_HOLDING_B) / 100
            } else {
                fx!(Self::JUMP_UP_VERT_ACCEL)
            };

            if holding_b && self.can_flutter_jump && self.velocity_y < fx!(Self::FLUTTER_JUMP_MAX_START_VERT_SPEED) {
                self.is_flutter_jumping = true;
                self.vert_accel = fx!(0.0);
            }
        }
    }

    pub fn update(&mut self, holding_b: bool) {
        match self.state {
            State::Wait => {
                // Assume b is pressed here
                // Velocity set during initialization
                // self.velocity_y = fx!(if self.double_jump {52.0} else {42.0}) * fx!(Self::JUMP_FACTOR);
                self.state = State::Jump;
            }

            State::Jump => {
                if self.triple_jump {
                    self.update_jump(true)
                } else {
                    match self.character {
                        Character::Mario | Character::Wario => self.update_jump(holding_b),
                        Character::Luigi => self.update_jump_luigi(holding_b),
                        Character::Yoshi => self.update_jump_yoshi(holding_b),
                    }
                }
            }
        }

        self.velocity_y = (self.velocity_y + self.vert_accel).max(self.terminal_velocity);
        self.position_y += self.velocity_y;
    }

    /// Updates the player starting a ground pound until their position becomes negative.
    /// Ground pounding from the floor isn't allowed, so if the player is on the floor,
    /// the position gets set to `fx!(-1.0)`
    pub fn update_ground_pound_until_below(&mut self) {
        if self.state == State::Wait {
            self.position_y = fx!(-1.0);
            return;
        }

        self.position_y += fx!(Self::GROUND_POUND_ASCENT);
        self.velocity_y = fx!(Self::GROUND_POUND_INIT_VEL);
        self.vert_accel = fx!(Self::VERT_ACCEL);

        while self.position_y >= fx!(0.0) {
            self.velocity_y = (self.velocity_y + self.vert_accel).max(self.terminal_velocity);
            self.position_y += self.velocity_y;
        }
    }

    /// Updates the yoshi until a condition is true, then returns the number of frames it took.
    pub fn update_until(&mut self, holding_b: impl Fn(&Self, usize) -> bool, cond: impl Fn(&Self) -> bool) -> usize {
        let mut frame = 0;
        while !cond(self) {
            self.update(holding_b(self, frame));
            frame += 1;
        }
        frame
    }
}