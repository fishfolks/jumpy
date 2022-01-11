use macroquad::color;
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World as CollisionWorld};

use hecs::World;

use serde::{Deserialize, Serialize};

use crate::json;
use crate::physics::GRAVITY;
use crate::Transform;

const FRICTION_LERP: f32 = 0.96;
const STOP_THRESHOLD: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct PhysicsBodyParams {
    pub size: Vec2,
    pub offset: Vec2,
    pub has_mass: bool,
    pub has_friction: bool,
    pub can_rotate: bool,
    pub bouncyness: f32,
}

impl Default for PhysicsBodyParams {
    fn default() -> Self {
        PhysicsBodyParams {
            size: vec2(16.0, 16.0),
            offset: Vec2::ZERO,
            has_mass: true,
            has_friction: true,
            can_rotate: true,
            bouncyness: 0.0,
        }
    }
}

/// Regular simulated physics bodies.
/// Note that rotation is abstract, only set on the transform to be used for draws. The colliders
/// are axis-aligned and will not be affected by rotation.
pub struct PhysicsBody {
    pub actor: Actor,
    pub offset: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub is_on_ground: bool,
    pub was_on_ground: bool,
    /// Will be `true` if the body is currently on top of a platform/jumpthrough tile
    pub is_on_platform: bool,
    /// If this is `true` the body will be affected by gravity
    pub has_mass: bool,
    pub has_friction: bool,
    pub can_rotate: bool,
    pub bouncyness: f32,
    pub is_deactivated: bool,
}

impl PhysicsBody {
    pub fn new<V: Into<Option<Vec2>>>(
        actor: Actor,
        velocity: V,
        params: PhysicsBodyParams,
    ) -> Self {
        let velocity = velocity.into().unwrap_or_default();

        PhysicsBody {
            actor,
            offset: params.offset,
            size: params.size,
            velocity,
            is_on_ground: false,
            was_on_ground: false,
            is_on_platform: false,
            has_mass: params.has_mass,
            has_friction: params.has_friction,
            can_rotate: params.can_rotate,
            bouncyness: params.bouncyness,
            is_deactivated: false,
        }
    }

    pub fn as_rect(&self, position: Vec2) -> Rect {
        let position = position + self.offset;
        Rect::new(position.x, position.y, self.size.x, self.size.y)
    }
}

pub fn update_physics_bodies(world: &mut World) {
    let dt = get_frame_time();

    let mut collision_world = storage::get_mut::<CollisionWorld>();

    for (_, (transform, body)) in world.query_mut::<(&mut Transform, &mut PhysicsBody)>() {
        collision_world.set_actor_position(body.actor, transform.position + body.offset);

        if !body.is_deactivated {
            let position = collision_world.actor_pos(body.actor);

            {
                let position = position + vec2(0.0, 1.0);

                body.was_on_ground = body.is_on_ground;

                body.is_on_ground = collision_world.collide_check(body.actor, position);

                // FIXME: Using this to set `is_on_ground` caused weird glitching behavior when jumping up through platforms
                let tile = collision_world.collide_solids(position, body.size.x as i32, body.size.y as i32);

                body.is_on_platform = tile == Tile::JumpThrough;
            }

            if !body.is_on_ground && body.has_mass {
                body.velocity.y += GRAVITY * dt;
            }

            if !collision_world.move_h(body.actor, body.velocity.x * dt) {
                body.velocity.x *= -body.bouncyness;
            }

            if !collision_world.move_v(body.actor, body.velocity.y * dt) {
                body.velocity.y *= -body.bouncyness;
            }

            if body.can_rotate {
                apply_rotation(dt, transform, &mut body.velocity, body.is_on_ground);
            }

            if body.is_on_ground && body.has_friction {
                body.velocity.x *= FRICTION_LERP;
                if body.velocity.x.abs() <= STOP_THRESHOLD {
                    body.velocity.x = 0.0;
                }
            }

            transform.position = collision_world.actor_pos(body.actor) - body.offset;
        }
    }
}

pub fn debug_draw_physics_bodies(world: &mut World) {
    for (_, (transform, body)) in world.query::<(&Transform, &PhysicsBody)>().iter() {
        if !body.is_deactivated {
            let rect = body.as_rect(transform.position);

            let color = if body.is_on_platform {
                color::YELLOW
            } else if body.is_on_ground {
                color::RED
            } else {
                color::GREEN
            };

            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RigidBodyParams {
    #[serde(with = "json::vec2_def")]
    pub offset: Vec2,
    #[serde(with = "json::vec2_def")]
    pub size: Vec2,
    #[serde(default, skip_serializing_if = "json::is_false")]
    pub can_rotate: bool,
}

impl Default for RigidBodyParams {
    fn default() -> Self {
        RigidBodyParams {
            offset: Vec2::ZERO,
            size: vec2(16.0, 16.0),
            can_rotate: false,
        }
    }
}

/// Simple physics bodies that has a velocity and optional rotation.
/// Note that rotation is abstract, only set on the transform to be used for draws. The colliders
/// are axis-aligned and will not be affected by rotation.
pub struct RigidBody {
    pub offset: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub can_rotate: bool,
}

impl RigidBody {
    pub fn new<V: Into<Option<Vec2>>>(velocity: V, params: RigidBodyParams) -> Self {
        let velocity = velocity.into().unwrap_or_default();

        RigidBody {
            offset: params.offset,
            size: params.size,
            velocity,
            can_rotate: params.can_rotate,
        }
    }

    pub fn as_rect(&self, position: Vec2) -> Rect {
        let position = position + self.offset;
        Rect::new(position.x, position.y, self.size.x, self.size.y)
    }
}

pub fn update_rigid_bodies(world: &mut World) {
    let dt = get_frame_time();

    for (_, (transform, body)) in world.query_mut::<(&mut Transform, &mut RigidBody)>() {
        transform.position += body.velocity * dt;

        if body.can_rotate {
            apply_rotation(dt, transform, &mut body.velocity, false);
        }
    }
}

pub fn debug_draw_rigid_bodies(world: &mut World) {
    for (_, (transform, body)) in world.query::<(&Transform, &RigidBody)>().iter() {
        let rect = body.as_rect(transform.position);

        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, color::RED)
    }
}

fn apply_rotation(dt: f32, transform: &mut Transform, velocity: &mut Vec2, is_on_ground: bool) {
    if !is_on_ground {
        transform.rotation += velocity.x.abs() * 0.00045 + velocity.y.abs() * 0.00015;

        velocity.y += GRAVITY * dt;
    } else {
        transform.rotation %= std::f32::consts::PI * 2.;
        let goal = if transform.rotation <= std::f32::consts::PI {
            std::f32::consts::PI
        } else {
            std::f32::consts::PI * 2.
        };

        let rest = goal - transform.rotation;
        if rest.abs() >= 0.1 {
            transform.rotation += (rest * 0.1).max(0.1);
        }
    }

    velocity.x *= 0.96;
    if velocity.x.abs() <= 1. {
        velocity.x = 0.0;
    }
}
