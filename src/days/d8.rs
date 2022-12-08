use std::fs;

pub fn d8() -> (String, String) {
    let read = fs::read("inputs/d8").unwrap();
    let lines: Vec<&[u8]> = read.split(|&x| x==b'\n').collect();

    let width = lines[0].len();
    let height = lines.len()-1;

    let mut forest:Vec<Tree> = vec![];


    // Build forest
    for y in 0..height {
        for x in 0..width {
            forest.push(Tree::new(lines[y][x]));
        }
    }

    // Forward pass
    for y in 0..height {
        for x in 0..width {
            // Update left max
            let left_tree = get_tree_at(&forest, width as i32, height as i32, x as i32 - 1, y as i32);

            let mut max_left = 0;
            if let Some(left_tree) = left_tree {
                if left_tree.height >= left_tree.max_left {
                    // New max tree
                    max_left = left_tree.height;
                } else {
                    max_left = left_tree.max_left;
                }
            }

            // Update top max 
            let top_tree = get_tree_at(&forest, width as i32, height as i32, x as i32, y as i32 - 1);

            let mut max_top = 0;
            if let Some(top_tree) = top_tree {
                if top_tree.height >= top_tree.max_top {
                    // New max tree
                    max_top = top_tree.height;
                } else {
                    max_top = top_tree.max_top;
                }
            }

            let current_tree_mut = &mut forest[(x+y*width) as usize];
            current_tree_mut.max_left = max_left;
            current_tree_mut.max_top = max_top;
        }
    }


    let mut count = 0;

    let mut max_scenery = 0;
    // Backwards pass 
    for y in (0..height).rev() {
        for x in (0..width).rev() {
            // Update right max
            let right_tree = get_tree_at(&forest, width as i32, height as i32, x as i32 + 1, y as i32);

            let mut max_right = 0;
            if let Some(right_tree) = right_tree {
                if right_tree.height >= right_tree.max_right {
                    // New max tree
                    max_right = right_tree.height;
                } else {
                    max_right = right_tree.max_right;
                }
            }

            // Update bottom max 
            let bottom_tree = get_tree_at(&forest, width as i32, height as i32, x as i32, y as i32 + 1);

            let mut max_bottom = 0;
            if let Some(bottom_tree) = bottom_tree {
                if bottom_tree.height >= bottom_tree.max_bottom {
                    // New max tree
                    max_bottom = bottom_tree.height;
                } else {
                    max_bottom = bottom_tree.max_bottom;
                }
            }

            let current_tree = &mut forest[(x+y*width) as usize];
            current_tree.max_right = max_right;
            current_tree.max_bottom = max_bottom;
        
            // Count visible
            if current_tree.height > current_tree.max_top || current_tree.height > current_tree.max_right || current_tree.height > current_tree.max_bottom || current_tree.height > current_tree.max_left {
                count += 1;
            }

            let current_height = current_tree.height;

            // Calculate scenery
            let mut left_scenery = x;
            for sx in (0..x).rev() {
                let tree = get_tree_at(&forest, width as i32, height as i32, sx as i32, y as i32);
                if let Some(tree) = tree {
                    if tree.height >= current_height {
                        // Found blocking tree
                        left_scenery = x - sx;
                        break;
                    }
                }
            }
            let mut right_scenery = width - 1 - x;
            for sx in (x+1)..width {
                let tree = get_tree_at(&forest, width as i32, height as i32, sx as i32, y as i32);
                if let Some(tree) = tree {
                    if tree.height >= current_height {
                        // Found blocking tree
                        right_scenery = sx - x;
                        break;
                    }
                }
            }
            let mut top_scenery = y;
            for sy in (0..y).rev() {
                let tree = get_tree_at(&forest, width as i32, height as i32, x as i32, sy as i32);
                if let Some(tree) = tree {
                    if tree.height >= current_height {
                        // Found blocking tree
                        top_scenery = y - sy;
                        break;
                    }
                }
            }
            let mut bottom_scenery = height - 1 - y;
            for sy in (y+1)..height {
                let tree = get_tree_at(&forest, width as i32, height as i32, x as i32, sy as i32);
                if let Some(tree) = tree {
                    if tree.height >= current_height {
                        // Found blocking tree
                        bottom_scenery = sy - y;
                        break;
                    }
                }
            }

            let scenery = bottom_scenery * top_scenery * left_scenery * right_scenery;
            max_scenery = max_scenery.max(scenery);
        }
    }

    (count.to_string(), max_scenery.to_string())
}

fn get_tree_at(forest: &Vec<Tree>, width: i32, height: i32, x: i32, y: i32) -> Option<&Tree> {
    if x < 0  || y < 0 || x >= width || y >= height {
        return None;
    } else {
        return Some(&forest[(x+y*width) as usize]);
    }
}


struct Tree {
    height: u8,
    max_top: u8,
    max_right: u8,
    max_bottom: u8,
    max_left: u8,
}

impl Tree {
    pub fn new(height: u8) -> Tree {
        Tree {
            height,
            max_top: 0,
            max_right: 0,
            max_bottom: 0,
            max_left: 0
        }  
    }
}