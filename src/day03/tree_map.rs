use std::ops;
use std::ops::Index;

pub struct Row {
    fields: Vec<bool>
}

impl Row {
    pub fn new(line: &str) -> Self {
        let mut row: Vec<bool> = Vec::new();
        for char in line.chars() {
            row.push(char == '#');
        }
        return Row {
            fields: row
        };
    }
}

impl Index<usize> for Row {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let x = index % self.fields.len();
        return self.fields.get(x).unwrap();
    }
}

pub struct TreeMap {
    pub height: usize,
    rows: Vec<Row>,
}

impl TreeMap {
    pub fn new(map: String) -> Self {
        let mut result: Vec<Row> = Vec::new();
        for line in map.lines() {
            result.push(Row::new(line));
        }

        return TreeMap {
            height: result.len(),
            rows: result,
        };
    }

    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        return self[y][x];
    }
}

impl ops::Index<usize> for TreeMap {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        return self.rows.get(index).unwrap();
    }
}
