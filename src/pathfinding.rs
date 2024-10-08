// Pathfinding
// "Stolen" from https://www.youtube.com/watch?v=QTUEyAZmdv4

use std::collections::VecDeque;

use bevy::prelude::*;
use pathfinding::prelude::astar;

use crate::{
    config::GRID_SIZE,
    grid::{Grid, GridLocation},
};

pub struct PathfindingError;

#[derive(Clone, Reflect, Default, Component)]
#[reflect(Component)]
pub struct Path {
    pub steps: VecDeque<GridLocation>,
}

impl<T> Grid<T> {
    pub fn path_to(
        &self,
        start: &GridLocation,
        goal: &GridLocation,
    ) -> Result<Path, PathfindingError> {
        let result = astar(
            start,
            |p| {
                neumann_neighbors(self, p)
                    .iter()
                    .map(|neighbor| (neighbor.clone(), 1))
                    .collect::<Vec<_>>()
            },
            |p| p.distance(goal) / 3,
            |p| p == goal,
        );

        if let Some((steps, _length)) = result {
            // Convert to VecDeque
            let mut steps: VecDeque<GridLocation> = steps.into();
            // Remove the first node, as it's always the one the entity is on
            steps.pop_front();
            // Return a path with the steps
            Ok(Path { steps: steps })
        } else {
            Err(PathfindingError)
        }
    }
}

pub fn neumann_neighbors<T>(grid: &Grid<T>, location: &GridLocation) -> Vec<GridLocation> {
    let (x, y) = (location.x as u32, location.y as u32);

    let mut sucessors = Vec::new();
    if let Some(left) = x.checked_sub(1) {
        let location = GridLocation::new(left, y);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if let Some(down) = y.checked_sub(1) {
        let location = GridLocation::new(x, down);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if x + 1 < GRID_SIZE as u32 {
        let right = x + 1;
        let location = GridLocation::new(right, y);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    if y + 1 < GRID_SIZE as u32 {
        let up = y + 1;
        let location = GridLocation::new(x, up);
        if !grid.occupied(&location) {
            sucessors.push(location);
        }
    }
    sucessors
}
