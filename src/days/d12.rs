use std::{fs, collections::HashSet};

pub fn d12() -> (String, String) {
    let read = fs::read("inputs/d12").unwrap();
    
    let mut start = 0;
    let mut lines: Vec<&[u8]> = vec![];
    for i in 0..read.len() {
        if read[i] == b'\n' {
            // End of line
            lines.push(&read[start..i]);
            start = i+1;
        }
    }

    let width = lines[0].len();
    let height = lines.len();

    // Create graph using two passes
    let mut graph = Graph {
        nodes: vec![],
        width,
        height
    };

    let mut start_pos = (0,0);
    let mut end_pos = (0,0);
    let mut starts = vec![];
    for y in 0..height {
        for x in 0..width {
            let c = lines[y][x];
            let h = match c {
                b'S' => {
                    start_pos = (x,y);
                    starts.push(graph.index((x,y)));
                    b'a'
                },
                b'E' => {
                    end_pos = (x,y);
                    b'z'
                },
                b'a' => {
                    starts.push(graph.index((x,y)));
                    b'a'
                }
                a => a
            };
            graph.nodes.push((h, vec![]));
        }
    }

    for y in 0..height {
        for x in 0..width {
            let c = &graph.nodes[y * width + x];

            let mut edges = vec![];

            // Create edges
            if let Some(top_node) = graph.get_node(x as i32, y as i32 - 1) {
                if top_node.0 <= c.0+1 {
                    // We can move to top node
                    edges.push((y-1)*width+x);
                }
            }
            
            if let Some(right_node) = graph.get_node((x+1) as i32, y as i32) {
                if right_node.0 <= c.0+1 {
                    // We can move to right node
                    edges.push(y*width+(x+1));
                }
            }

            if let Some(bottom_node) = graph.get_node(x as i32, (y+1) as i32) {
                if bottom_node.0 <= c.0+1 {
                    // We can move to top node
                    edges.push((y+1)*width+x);
                }
            }

            if let Some(left_node) = graph.get_node(x as i32 -1, y as i32) {
                if left_node.0 <= c.0+1 {
                    // We can move to left node
                    edges.push(y*width+(x-1));
                }
            }

            graph.nodes[y * width + x].1 = edges;
        }
    }

    (
        graph.bfs(vec![graph.index(start_pos)], end_pos).to_string(), 
        graph.bfs(starts, end_pos).to_string()
    )
}

type Node = (u8, Vec<usize>);

struct Graph {
    nodes: Vec<Node>,
    width: usize,
    height: usize,
}

impl Graph {
    pub fn get_node(&self, x: i32, y: i32) -> Option<&Node> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        } else {
            return Some(&self.nodes[(y as usize * self.width) + x as usize]);
        }
    }

    pub fn index(&self, pos: (usize, usize)) -> usize {
        pos.1 * self.width + pos.0
    }

    pub fn bfs(&self, starts: Vec<usize>, end: (usize, usize)) -> usize {
        let mut previous = vec![None; self.nodes.len()];
        
        let mut visited = HashSet::new();
        let mut queue = vec![];

        for s in starts {
            visited.insert(s);
            queue.push(s);
        }

        loop {
            let node = queue.remove(0);

            if node == self.index(end) {
                break;
            }
            for edge in self.nodes[node].1.iter() {
                if !visited.contains(&edge) {
                    visited.insert(*edge);
                    previous[*edge] = Some(node);
                    queue.push(*edge);
                }
            }
        }

        // Reconstruct path
        let mut path = vec![];
        let mut target = Some(self.index(end));
        if let Some(_) = previous[target.unwrap()] {
            while let Some(a) = target {
                path.insert(0, a);
                target = previous[a];
            }
        }

        path.len()-1
    }
}