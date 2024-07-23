use crate::{dev_tools::DebugContext, screen::Screen};
use bevy::prelude::*;

use super::spawn::map::ChunkTag;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Collider>();

    #[cfg(feature = "dev")]
    app.add_systems(Update, colliders_gizmos.run_if(in_state(Screen::Playing)));
    app.add_systems(Update, update_colliders.run_if(in_state(Screen::Playing)));
}

#[cfg(feature = "dev")]
pub fn colliders_gizmos(
    mut gizmos: Gizmos,
    debug_context: Res<DebugContext>,
    query: Query<(&Transform, &Collider)>,
) {
    use bevy::color::palettes::css::ORANGE_RED;

    if debug_context.enabled {
        for (transform, collider) in query.iter() {
            match collider {
                Collider::Rect(rect) => {
                    gizmos.rect_2d(
                        collider.center(),
                        0.,
                        rect.size(),
                        ORANGE_RED.with_alpha(0.6),
                    );
                }
                Collider::Circle(circle) => {
                    gizmos.circle_2d(
                        transform.translation.xy(),
                        circle.radius,
                        ORANGE_RED.with_alpha(0.6),
                    );
                }
            }
        }
    }
}

fn update_colliders(
    mut query: Query<
        (&GlobalTransform, &mut Collider),
        (Without<ChunkTag>, Without<ExcludeColliderUpdate>),
    >,
) {
    for (transform, mut collider) in query.iter_mut() {
        collider.pos(transform.translation().xy());
    }
}

// -------------------------------------------

#[derive(Debug, Reflect, Clone, Copy)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub enum Collider {
    Rect(Rect),
    Circle(Circle),
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ExcludeColliderUpdate;

// https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
// https://www.jeffreythompson.org/collision-detection/circle-rect.php
// https://learnopengl.com/In-Practice/2D-Game/Collisions/Collision-detection
// https://www.metanetsoftware.com/technique/tutorialA.html
// https://code.tutsplus.com/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331t

// collision with rotation
// https://stackoverflow.com/questions/62028169/how-to-detect-when-rotated-rectangles-are-colliding-each-other

impl Collider {
    pub fn rect(half_x: f32, half_y: f32) -> Self {
        Self::Rect(Rect::from_center_half_size(
            Vec2::ZERO,
            Vec2::new(half_x, half_y),
        ))
    }

    pub fn new_rect(origin: Vec2, half_size: Vec2) -> Self {
        Self::Rect(Rect::from_center_half_size(origin, half_size))
    }

    pub fn new_rect_corners(p0: Vec2, p1: Vec2) -> Self {
        Self::Rect(Rect::from_corners(p0, p1))
    }

    pub fn new_rect_half_size(half_size: Vec2) -> Self {
        Self::Rect(Rect::from_center_half_size(Vec2::ZERO, half_size))
    }

    pub fn new_circle(center: Vec2, radius: f32) -> Self {
        Self::Circle(Circle { center, radius })
    }

    // aabb
    pub fn collide(&self, other: &Collider) -> bool {
        match self {
            Self::Rect(rect) => match &other {
                Self::Rect(other_rect) => {
                    if rect.min.x < other_rect.max.x
                        && rect.max.x > other_rect.min.x
                        && rect.min.y < other_rect.max.y
                        && rect.max.y > other_rect.min.y
                    {
                        return true;
                    }
                    false
                }
                Self::Circle(other_circle) => {
                    let closest_x = rect.min.x.max(other_circle.center.x.min(rect.max.x));
                    let closest_y = rect.min.y.max(other_circle.center.y.min(rect.max.y));
                    let distance_x = other_circle.center.x - closest_x;
                    let distance_y = other_circle.center.y - closest_y;
                    let distance = distance_x * distance_x + distance_y * distance_y;
                    distance < other_circle.radius * other_circle.radius
                }
            },
            Self::Circle(circle) => match &other {
                Self::Rect(other_rect) => {
                    let closest_x = other_rect.min.x.max(circle.center.x.min(other_rect.max.x));
                    let closest_y = other_rect.min.y.max(circle.center.y.min(other_rect.max.y));
                    let distance_x = circle.center.x - closest_x;
                    let distance_y = circle.center.y - closest_y;
                    let distance = distance_x * distance_x + distance_y * distance_y;
                    distance < circle.radius * circle.radius
                }
                Self::Circle(other_circle) => {
                    let distance = (circle.center - other_circle.center).length();
                    distance < circle.radius + other_circle.radius
                }
            },
        }
    }

    pub fn left(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.min.x,
            Self::Circle(circle) => circle.center.x - circle.radius,
        }
    }

    pub fn right(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.max.x,
            Self::Circle(circle) => circle.center.x + circle.radius,
        }
    }

    pub fn top(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.max.y,
            Self::Circle(circle) => circle.center.y + circle.radius,
        }
    }

    pub fn bottom(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.min.y,
            Self::Circle(circle) => circle.center.y - circle.radius,
        }
    }

    pub fn center(&self) -> Vec2 {
        match self {
            Self::Rect(rect) => rect.center(),
            Self::Circle(circle) => circle.center,
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            Self::Rect(rect) => rect.size(),
            Self::Circle(circle) => Vec2::splat(circle.radius * 2.),
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.height(),
            Self::Circle(circle) => circle.radius * 2.,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.width(),
            Self::Circle(circle) => circle.radius * 2.,
        }
    }

    pub fn half_x(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.half_size().x,
            Self::Circle(circle) => circle.radius,
        }
    }

    pub fn half_y(&self) -> f32 {
        match self {
            Self::Rect(rect) => rect.half_size().y,
            Self::Circle(circle) => circle.radius,
        }
    }

    // new center position
    pub fn pos(&mut self, pos: Vec2) {
        match self {
            Self::Rect(rect) => {
                let half_size = rect.half_size();
                rect.min = pos - half_size;
                rect.max = pos + half_size;
            }
            Self::Circle(circle) => circle.center = pos,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

/// Axis-aligned bounding box collision with "side" detection
/// * `a_pos` and `b_pos` are the center positions of the rectangles, typically obtained by
/// extracting the `translation` field from a `Transform` component
/// * `a_size` and `b_size` are the dimensions (width and height) of the rectangles.
pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Collision::Left, b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, a_min.x - b_max.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Collision::Bottom, b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, a_min.y - b_max.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth.abs() < x_depth.abs() {
            Some(y_collision)
        } else {
            Some(x_collision)
        }
    } else {
        None
    }
}

pub fn check_wall_collision(
    target_entity_position: Vec3,
    wall_query: &Query<&Collider>,
    // wall_query: &Query<&ColliderBox, With<WallTile>>,
) -> bool {
    for collider in wall_query.iter() {
        let collision = collide(
            target_entity_position.xy().extend(0.),
            Vec2::splat(16.),
            collider.center().extend(0.),
            Vec2::splat(16.),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}
