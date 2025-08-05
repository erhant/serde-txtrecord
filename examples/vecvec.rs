use serde::{Deserialize, Serialize};
use serde_txtrecord::{from_txt_records, to_txt_records};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Matrix {
    name: String,
    data: Vec<Vec<i32>>,
    metadata: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct GameState {
    player_scores: Vec<Vec<u32>>,
    board: Vec<Vec<char>>,
    history: Vec<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Mathematical matrix
    let matrix = Matrix {
        name: "Identity Matrix".to_string(),
        data: vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
        metadata: vec![
            vec!["row1".to_string(), "3x3".to_string()],
            vec!["row2".to_string(), "identity".to_string()],
            vec!["row3".to_string(), "diagonal".to_string()],
        ],
    };

    let records = to_txt_records(&matrix)?;

    // Verify correct number of records for nested vectors
    assert_eq!(
        records.len(),
        24,
        "Matrix should generate exactly 24 records"
    );

    let records_map: std::collections::HashMap<String, String> = records.iter().cloned().collect();

    // Verify matrix name
    assert_eq!(
        records_map.get("name"),
        Some(&"Identity Matrix".to_string())
    );

    // Verify data array structure
    assert_eq!(records_map.get("data_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("data_0_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("data_1_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("data_2_len"), Some(&"3".to_string()));

    // Verify identity matrix values
    assert_eq!(records_map.get("data_0_0"), Some(&"1".to_string()));
    assert_eq!(records_map.get("data_0_1"), Some(&"0".to_string()));
    assert_eq!(records_map.get("data_1_1"), Some(&"1".to_string()));
    assert_eq!(records_map.get("data_2_2"), Some(&"1".to_string()));

    // Verify metadata structure
    assert_eq!(records_map.get("metadata_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("metadata_0_0"), Some(&"row1".to_string()));
    assert_eq!(
        records_map.get("metadata_1_1"),
        Some(&"identity".to_string())
    );

    // Deserialize back and verify complete roundtrip
    let deserialized_matrix: Matrix = from_txt_records(records)?;
    assert_eq!(
        matrix, deserialized_matrix,
        "Matrix roundtrip must preserve all data"
    );

    // Example 2: Game state with nested vectors
    let game_state = GameState {
        player_scores: vec![
            vec![100, 150, 200], // Player 1 scores across rounds
            vec![120, 140, 180], // Player 2 scores across rounds
            vec![90, 160, 220],  // Player 3 scores across rounds
        ],
        board: vec![
            vec!['X', 'O', 'X'],
            vec!['O', 'X', 'O'],
            vec!['X', 'X', 'O'],
        ],
        history: vec![
            vec!["move1".to_string(), "X:0,0".to_string()],
            vec!["move2".to_string(), "O:1,0".to_string()],
            vec!["move3".to_string(), "X:2,0".to_string()],
        ],
    };

    let records = to_txt_records(&game_state)?;

    // Verify correct number of records for complex nested structure
    assert_eq!(
        records.len(),
        36,
        "Game state should generate exactly 36 records"
    );

    let records_map: std::collections::HashMap<String, String> = records.iter().cloned().collect();

    // Verify player scores structure and values
    assert_eq!(records_map.get("player_scores_len"), Some(&"3".to_string()));
    assert_eq!(
        records_map.get("player_scores_0_0"),
        Some(&"100".to_string())
    );
    assert_eq!(
        records_map.get("player_scores_1_1"),
        Some(&"140".to_string())
    );
    assert_eq!(
        records_map.get("player_scores_2_2"),
        Some(&"220".to_string())
    );

    // Verify board structure and character values
    assert_eq!(records_map.get("board_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("board_0_0"), Some(&"X".to_string()));
    assert_eq!(records_map.get("board_1_1"), Some(&"X".to_string()));
    assert_eq!(records_map.get("board_2_2"), Some(&"O".to_string()));

    // Verify history structure and string values
    assert_eq!(records_map.get("history_len"), Some(&"3".to_string()));
    assert_eq!(records_map.get("history_0_0"), Some(&"move1".to_string()));
    assert_eq!(records_map.get("history_1_1"), Some(&"O:1,0".to_string()));
    assert_eq!(records_map.get("history_2_0"), Some(&"move3".to_string()));

    // Deserialize back and verify complete roundtrip
    let deserialized_game: GameState = from_txt_records(records)?;
    assert_eq!(
        game_state, deserialized_game,
        "Game state roundtrip must preserve all data"
    );

    Ok(())
}
