use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JSONTestCase {
    title: HashMap<String, String>,
    test_in: String,
    is_test: String,
    is_validator: String,
}

#[derive(Debug)]
pub struct TestCase {
    pub board: [[i8; 15]; 15],
    pub is_test: bool,
    pub is_validator: bool,
    pub title: String,
}

pub fn load_json(testcase: &str) -> TestCase {
    let contents = fs::read_to_string(testcase).expect("Should have been able to read the file");
    let p: JSONTestCase =
        serde_json::from_str(&contents).expect("Should have been able to parse the file");

    let board: [[i8; 15]; 15] = string_to_2d_array(&p.test_in);
    let is_test = p.is_test == "true";
    let is_validator = p.is_validator == "true";
    let title = get_title(&p);

    TestCase {
        board,
        is_test,
        is_validator,
        title,
    }
}

fn string_to_2d_array(input: &str) -> [[i8; 15]; 15] {
    let mut board: [[i8; 15]; 15] = [[0; 15]; 15];

    for (i, line) in input.lines().enumerate() {
        for (j, num) in line.split_whitespace().enumerate() {
            board[14 - i][j] = num.parse().unwrap();
        }
    }

    board
}

fn get_title(s: &JSONTestCase) -> String {
    s.title.get("1").unwrap().to_string()
}
