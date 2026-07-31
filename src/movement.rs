use crate::protocol::PlayerInput;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyState {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub grounded: bool,
}

impl BodyState {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            velocity: [0.0; 3],
            yaw: 0.0,
            pitch: 0.0,
            grounded: false,
        }
    }
}

/// Movement constants matching the client's MovementConfig defaults.
pub struct MovementParams {
    pub max_walk_speed: f32,
    pub max_sprint_speed: f32,
    pub max_crouch_speed: f32,
    pub ground_acceleration: f32,
    pub ground_friction: f32,
    pub air_acceleration: f32,
    pub air_speed_cap: f32,
    pub jump_force: f32,
    pub gravity: f32,
    pub strafe_speed_multiplier: f32,
    pub backpedal_speed_multiplier: f32,
}

impl Default for MovementParams {
    fn default() -> Self {
        Self {
            max_walk_speed: 11.0,
            max_sprint_speed: 16.0,
            max_crouch_speed: 5.0,
            ground_acceleration: 80.0,
            ground_friction: 15.0,
            air_acceleration: 0.5,
            air_speed_cap: 0.1,
            jump_force: 6.5,
            gravity: 20.0,
            strafe_speed_multiplier: 0.85,
            backpedal_speed_multiplier: 0.65,
        }
    }
}

/// Simplified server-side advance matching client physics (no camera/mantle/dive/slide).
pub fn advance(state: &mut BodyState, input: &PlayerInput, params: &MovementParams, dt: f32) {
    state.yaw = input.look_yaw;
    state.pitch = input.look_pitch;

    state.grounded = state.position[1] <= 0.01 && state.velocity[1] <= 0.0;

    let wish_dir_raw = [input.movement[0], 0.0, input.movement[2]];
    let wish_dir_len =
        (wish_dir_raw[0] * wish_dir_raw[0] + wish_dir_raw[2] * wish_dir_raw[2]).sqrt();
    let has_input = wish_dir_len > 0.001;

    if has_input {
        let wish_dir = [
            wish_dir_raw[0] / wish_dir_len,
            0.0,
            wish_dir_raw[2] / wish_dir_len,
        ];

        let sprint = input.actions.contains(crate::protocol::PlayerActions::SPRINT);
        let crouch = input.actions.contains(crate::protocol::PlayerActions::CROUCH);

        let base_wish_speed = if crouch {
            params.max_crouch_speed
        } else if sprint {
            params.max_sprint_speed
        } else {
            params.max_walk_speed
        };

        if state.grounded {
            // Note: The input.movement is already camera-relative from the client.
            // For strafe/backpedal multipliers, we approximate using the sign of
            // the movement vector components.
            let raw_y = input.movement[1];
            let raw_x = input.movement[0];
            let dir_mult = if raw_y < 0.0 {
                params.backpedal_speed_multiplier
            } else if raw_y == 0.0 && raw_x.abs() > 0.0 {
                params.strafe_speed_multiplier
            } else {
                1.0
            };

            let wish_speed = base_wish_speed * dir_mult;

            // Ground acceleration — Quake projection model
            let current_speed =
                state.velocity[0] * wish_dir[0] + state.velocity[2] * wish_dir[2];
            let add_speed = wish_speed - current_speed;

            if add_speed > 0.0 {
                let accel_speed =
                    (params.ground_acceleration * dt * wish_speed).min(add_speed);
                state.velocity[0] += wish_dir[0] * accel_speed;
                state.velocity[2] += wish_dir[2] * accel_speed;
            }

            // Clamp to max speed
            let horiz = (state.velocity[0] * state.velocity[0]
                + state.velocity[2] * state.velocity[2])
                .sqrt();
            if horiz > wish_speed {
                let scale = wish_speed / horiz;
                state.velocity[0] *= scale;
                state.velocity[2] *= scale;
            }
        } else {
            // Air — near-zero acceleration
            let current_speed =
                state.velocity[0] * wish_dir[0] + state.velocity[2] * wish_dir[2];
            let add_speed = params.air_speed_cap - current_speed;

            if add_speed > 0.0 {
                let accel_speed =
                    (params.air_acceleration * dt * params.air_speed_cap).min(add_speed);
                state.velocity[0] += wish_dir[0] * accel_speed;
                state.velocity[2] += wish_dir[2] * accel_speed;
            }
        }
    }

    // Ground friction
    if state.grounded {
        let horiz_speed = (state.velocity[0] * state.velocity[0]
            + state.velocity[2] * state.velocity[2])
            .sqrt();
        if horiz_speed > 0.001 {
            let control = horiz_speed.max(6.0);
            let drop = control * params.ground_friction * dt;
            let new_speed = (horiz_speed - drop).max(0.0);
            let scale = new_speed / horiz_speed;
            state.velocity[0] *= scale;
            state.velocity[2] *= scale;
        }
    }

    // Gravity
    if !(state.grounded && state.velocity[1] <= 0.0) {
        state.velocity[1] -= params.gravity * dt;
    }

    // Jump
    let jump = input.actions.contains(crate::protocol::PlayerActions::JUMP);
    if jump && state.grounded {
        state.velocity[1] = params.jump_force;
        state.grounded = false;
    }

    // Integration
    state.position[0] += state.velocity[0] * dt;
    state.position[1] += state.velocity[1] * dt;
    state.position[2] += state.velocity[2] * dt;

    if state.position[1] < 0.0 {
        state.position[1] = 0.0;
        if state.velocity[1] < 0.0 {
            state.velocity[1] = 0.0;
        }
    }
}
