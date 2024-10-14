// Pathfinding
// "Stolen" from https://www.youtube.com/watch?v=QTUEyAZmdv4

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_ldtk::{utils::grid_coords_to_translation, GridCoords};
use pathfinding::prelude::astar;

use crate::{
    config::{GRID_SIZE, TILE_SIZE},
    grid::{Grid, GridLocation},
};

pub struct PathfindingError;

#[derive(Clone, Reflect, Default, Component)]
#[reflect(Component)]
pub struct Path {
    pub steps: VecDeque<GridCoords>,
}

impl<T> Grid<T> {
    pub fn path_to(&self, start: &GridCoords, goal: &GridCoords) -> Result<Path, PathfindingError> {
        let result = astar(
            start,
            |p| {
                neumann_neighbors(self, p)
                    .iter()
                    .map(|neighbor| (neighbor.clone(), 1))
                    .collect::<Vec<_>>()
            },
            |p| GridLocation::from(*p).distance(&GridLocation::from(*goal)) / 3,
            |p| p == goal,
        );

        if let Some((steps, _length)) = result {
            // Convert to VecDeque
            let mut steps: VecDeque<GridCoords> = steps.into();
            // Remove the first node, as it's always the one the entity is on
            steps.pop_front();
            // Return a path with the steps
            Ok(Path { steps: steps })
        } else {
            Err(PathfindingError)
        }
    }
}

pub fn neumann_neighbors<T>(grid: &Grid<T>, location: &GridCoords) -> Vec<GridCoords> {
    let (x, y) = (location.x, location.y);

    let mut sucessors = Vec::new();
    if let Some(left) = x.checked_sub(1) {
        let location = GridCoords::new(left as i32, y as i32);
        if !grid.occupied(&GridLocation::from(location)) {
            sucessors.push(location);
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = GridCoords::new(x as i32, down as i32);
        if !grid.occupied(&GridLocation::from(location)) {
            sucessors.push(location);
        }
    }
    if x + 1 < GRID_SIZE.x {
        let right = x + 1;
        let location = GridCoords::new(right as i32, y as i32);
        if !grid.occupied(&GridLocation::from(location)) {
            sucessors.push(location);
        }
    }
    if y + 1 < GRID_SIZE.y {
        let up = y + 1;
        let location = GridCoords::new(x as i32, up as i32);
        if !grid.occupied(&GridLocation::from(location)) {
            sucessors.push(location);
        }
    }
    sucessors
}

pub fn pathfinding_gizmos(mut gizmos: Gizmos, query: Query<&Path>) {
    for path in &query {
        for step in &path.steps {
            let location = grid_coords_to_translation(*step, TILE_SIZE);

            gizmos.rect_2d(
                location,
                0.,
                Vec2::new(TILE_SIZE.x as f32, TILE_SIZE.y as f32),
                Color::srgb(1., 0., 0.),
            );
        }
    }
}
