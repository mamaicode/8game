use std::collections::{HashSet, VecDeque};

// State of the game
pub type GameState = [[u8; 3]; 3];

const MOVEMENTS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, Down, Left, Right

struct Node {
    state: GameState,
    path: Vec<String>,
}

fn is_goal_state(state: &GameState) -> bool {
    let goal_state: GameState = [[1, 2, 3], [8, 0, 4], [7, 6, 5]]; // Goal state
    state == &goal_state
}

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

    if new_row < 3 && new_col < 3 {
        new_state[blank_row][blank_col] = state[new_row][new_col];
        new_state[new_row][new_col] = 0;
    }

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

// Perform Breadth-First Search to find the path to the goal state
pub fn bfs_search(initial_state: GameState) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let initial_node = Node {
        state: initial_state,
        path: Vec::new(),
    };

    queue.push_back(initial_node);

    while let Some(node) = queue.pop_front() {
        let current_state = node.state;

        if is_goal_state(&current_state) {
            return Some(node.path);
        }

        visited.insert(current_state);

        let neighbors = generate_neighbors(&current_state);
        for neighbor in neighbors {
            if !visited.contains(&neighbor) && is_valid_state(&neighbor) {
                let mut path = node.path.clone();
                let movement = match neighbor {
                    _ if apply_movement(&current_state, &(1, 0)) == neighbor => "down",
                    _ if apply_movement(&current_state, &(-1, 0)) == neighbor => "up",
                    _ if apply_movement(&current_state, &(0, 1)) == neighbor => "right",
                    _ => "left",
                };
                path.push(movement.to_string());
                let neighbor_node = Node {
                    state: neighbor,
                    path,
                };
                queue.push_back(neighbor_node);
            }
        }
    }

    None
}
