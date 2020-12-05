use crate::day03::tree_map::TreeMap;
use crate::day::Day;

mod tree_map;

pub struct Day3 {
    map: TreeMap,
}

impl Day3 {
    pub fn new(input: String) -> Self {
        Day3 { map: TreeMap::new(input) }
    }

    fn count_trees_for_slope(&self, x_step: usize, y_step: usize) -> usize {
        let mut trees = 0;
        let mut y = 0;
        let mut x = 0;
        while y < self.map.height {
            if self.map.is_tree(x, y) {
                trees += 1;
            }
            x += x_step;
            y += y_step;
        }
        return trees;
    }
}

impl Day for Day3 {
    fn task1(&self) -> String {
        let trees = self.count_trees_for_slope(3, 1);
        return format!("Found {} trees", trees);
    }

    fn task2(&self) -> String {
        let slopes = [
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2)
        ];

        let mut result = 1;

        for (x, y) in &slopes {
            result *= self.count_trees_for_slope(*x, *y);
        }

        return format!("Product: {}", result);
    }
}

