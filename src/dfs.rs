use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

pub type GameState = [[u8; 3]; 3];

const MOVEMENTS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, Down, Left, Right

fn is_valid_state(state: &GameState) -> bool {
    let mut num_occurrences = [0; 9];
    for row in state.iter() {
        for &num in row.iter() {
            num_occurrences[num as usize] += 1;
        }
    }
    num_occurrences[0] == 1 && num_occurrences.iter().all(|&occurrences| occurrences <= 1)
}

fn get_blank_space(state: &GameState) -> (usize, usize) {
    for (i, row) in state.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if num == 0 {
                return (i, j);
            }
        }
    }
    unreachable!();
}

fn apply_movement(state: &GameState, movement: &(i8, i8)) -> GameState {
    let (blank_row, blank_col) = get_blank_space(state);
    let mut new_state = *state;
    let (new_row, new_col) = (
        (blank_row as i8 + movement.0) as usize,
        (blank_col as i8 + movement.1) as usize,
    );
    new_state[blank_row][blank_col] = state[new_row][new_col];
    new_state[new_row][new_col] = 0;
    new_state
}

fn generate_neighbors(state: &GameState) -> Vec<GameState> {
    let mut neighbors = Vec::new();
    let (blank_row, blank_col) = get_blank_space(state);
    for movement in MOVEMENTS.iter() {
        let (new_row, new_col) = (
            (blank_row as i8 + movement.0) as usize,
            (blank_col as i8 + movement.1) as usize,
        );
        if new_row < 3 && new_col < 3 {
            let neighbor = apply_movement(state, movement);
            neighbors.push(neighbor);
        }
    }
    neighbors
}

// Perform Depth-First Search to generate the "eight" game
fn dfs_search(current_state: GameState, visited: &mut HashSet<GameState>) -> Option<GameState> {
    if visited.len() == 362_880 {
        // All possible game states have been visited
        return None;
    }

    visited.insert(current_state);

    let mut rng = thread_rng();
    let mut neighbors = generate_neighbors(&current_state);
    neighbors.shuffle(&mut rng);

    for neighbor in neighbors {
        if !visited.contains(&neighbor) && is_valid_state(&neighbor) {
            if let Some(solution) = dfs_search(neighbor, visited) {
                return Some(solution);
            }
        }
    }

    visited.remove(&current_state);
    None
}

pub fn dfs_generate(initial_state: GameState) -> Option<GameState> {
    let mut visited = HashSet::new();
    dfs_search(initial_state, &mut visited)
}
