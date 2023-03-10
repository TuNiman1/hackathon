use crate::component::Collision;
use crate::component::CollisionSolution;
use crate::component::Inertia;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Entity;
use bevy::prelude::Query;
use bevy::prelude::ResMut;
use bevy::prelude::Transform;

#[derive(Default)]
pub struct CollisionSystemData {
    previous_solutions: usize,
}

#[allow(clippy::many_single_char_names)]
pub fn collision_find(
    query: Query<(Entity, &Collision, &Transform, &Inertia)>,
    mut data: ResMut<CollisionSystemData>,
) -> Vec<CollisionSolution> {
    let mut solutions = Vec::with_capacity(data.previous_solutions);

    for (n, (e1, c1, t1, i1)) in query.iter().enumerate() {
        for (e2, c2, t2, i2) in query.iter().skip(n + 1) {
            if let Some(shift) =
                Collision::resolve(c1, c2, t1.translation.xy(), t2.translation.xy())
            {
                let relative_angle = (t2.translation - t1.translation).xy().normalize();
                let push = Inertia::bounce(i1, i2, relative_angle);
                append_solution(&mut solutions, e1.id(), shift, push);
                append_solution(&mut solutions, e2.id(), -shift, -push);
            }
        }
    }

    data.previous_solutions = solutions.len();

    return solutions;
}

fn append_solution(
    solutions: &mut Vec<CollisionSolution>,
    entity_id: u32,
    shift: Vec2,
    push: Vec2,
) {
    for solution in solutions.iter_mut() {
        if solution.entity_id == entity_id {
            solution.shift += shift;
            solution.push += push;
            return;
        }
    }

    solutions.push(CollisionSolution {
        entity_id,
        shift,
        push,
    });
}
