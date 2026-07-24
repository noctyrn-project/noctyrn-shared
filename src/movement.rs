use crate::protocol::PlayerInput;

/// Per-player state tracked by the advance function.
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
    pub direction_change_penalty: f32,
    pub air_acceleration: f32,
    pub air_speed_cap: f32,
    pub max_horizontal_speed: f32,
    pub jump_force: f32,
    pub gravity: f32,
}

impl Default for MovementParams {
    fn default() -> Self {
        Self {
            max_walk_speed: 11.0,
            max_sprint_speed: 16.0,
            max_crouch_speed: 5.0,
            ground_acceleration: 55.0,
            ground_friction: 10.0,
            direction_change_penalty: 0.6,
            air_acceleration: 10.0,
            air_speed_cap: 1.8,
            max_horizontal_speed: 18.0,
            jump_force: 6.5,
            gravity: 20.0,
        }
    }
}

/// Advance a player's state by one tick given their input and movement params.
///
/// Uses Quake-style acceleration + friction model, matching the client:
///
/// 1. Yaw/pitch set from input
/// 2. Grounded = pos.y <= 0.01 && vel.y <= 0
/// 3. Acceleration (projection model, ground or air)
/// 4. Friction (speed-dependent, grounded only)
/// 5. Gravity (skipped when grounded and falling)
/// 6. Jump (if grounded + JUMP flag)
/// 7. Horizontal speed clamp
/// 8. Euler integration
/// 9. Ground clamp at y=0
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

        let wish_speed = if crouch {
            params.max_crouch_speed
        } else if sprint {
            params.max_sprint_speed
        } else {
            params.max_walk_speed
        };

        if state.grounded {
            let horiz_vel_len = (state.velocity[0] * state.velocity[0]
                + state.velocity[2] * state.velocity[2])
                .sqrt();
            if horiz_vel_len > 0.5 {
                let dot = (state.velocity[0] / horiz_vel_len) * wish_dir[0]
                    + (state.velocity[2] / horiz_vel_len) * wish_dir[2];
                if dot < 0.0 {
                    let penalty =
                        1.0 + (1.0 - params.direction_change_penalty) * (-dot) * dt * 30.0;
                    state.velocity[0] /= penalty;
                    state.velocity[2] /= penalty;
                }
            }

            let current_speed =
                state.velocity[0] * wish_dir[0] + state.velocity[2] * wish_dir[2];
            let add_speed = wish_speed - current_speed;

            if add_speed > 0.0 {
                let accel_speed =
                    (params.ground_acceleration * dt * wish_speed).min(add_speed);
                state.velocity[0] += wish_dir[0] * accel_speed;
                state.velocity[2] += wish_dir[2] * accel_speed;
            }
        } else {
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

    if state.grounded {
        let horiz_speed = (state.velocity[0] * state.velocity[0]
            + state.velocity[2] * state.velocity[2])
            .sqrt();
        if horiz_speed > 0.001 {
            let min_control_speed = 4.0;
            let control = horiz_speed.max(min_control_speed);
            let drop = control * params.ground_friction * dt;
            let new_speed = (horiz_speed - drop).max(0.0);
            let scale = new_speed / horiz_speed;
            state.velocity[0] *= scale;
            state.velocity[2] *= scale;
        }
    }

    if !(state.grounded && state.velocity[1] <= 0.0) {
        state.velocity[1] -= params.gravity * dt;
    }

    let jump = input.actions.contains(crate::protocol::PlayerActions::JUMP);
    if jump && state.grounded {
        state.velocity[1] = params.jump_force;
        state.grounded = false;
    }

    let horiz_speed = (state.velocity[0] * state.velocity[0]
        + state.velocity[2] * state.velocity[2])
        .sqrt();
    if horiz_speed > params.max_horizontal_speed {
        let scale = params.max_horizontal_speed / horiz_speed;
        state.velocity[0] *= scale;
        state.velocity[2] *= scale;
    }

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
