mod game;

use game::{bfs_search, GameState};

fn main() {
    let initial_state: GameState = [[1, 2, 0], [8, 4, 3], [7, 6, 5]]; // Initial state
    if let Some(path) = bfs_search(initial_state) {
        println!("Solution Found! The path of 'sh' to the goal is: {}", path.join(", "));
    } else {
        println!("No solution found.");
    }
}
