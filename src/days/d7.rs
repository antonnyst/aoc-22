use std::fs;

pub fn d7() -> (String, String) {
    let read = fs::read_to_string("inputs/d7").unwrap();
    let mut commands = read.split("$").map(|x| x.trim() ).collect::<Vec<&str>>();
    commands.remove(0);

    let root = File {
        name: String::from("/"),
        parent: None,
        size: 0,
        children: vec![]
    };

    let mut state = State {
        files: vec![root],
        location: None,
    };

    for c in commands {
        command(&mut state, c);
    }
    
    let used_space = state.total_size(&FileId { index: 0 });
    let free_space = 70_000_000 - used_space;
    let min_remove = 30_000_000 - free_space;


    let mut best_size = used_space;


    let mut sum = 0;
    for i in 0..state.files.len() {
        let size = state.total_size(&FileId { index: i });

        if size > min_remove && size < best_size {
            best_size = size;
        }

        if state.files[i].children.len() > 0 {
            if size <= 100000 {
                sum += size;
            }
        }
    }

    (sum.to_string(), best_size.to_string())
}

fn command(state: &mut State, command: &str) {
    let cmd = &command[..2];
    let arg = &command[2..].trim();
    //println!("{:?}, {:?}", cmd, arg);
    match cmd {
        "cd" => cd(state, arg),
        "ls" => ls(state, arg),
        _ => unimplemented!()
    }
}


fn cd(state: &mut State, arg: &str) {
    match arg {
        "/" => state.location = Some(FileId { index: 0 }),
        ".." => {
            if let Some(location) = state.location {
                state.location = state.files[location.index].parent;
            }
        },
        arg => {
            if let Some(location) = state.location {
                let result = state.find(&location, arg);

                if let None = result {
                    println!("CD to none");
                }

                state.location = result;
            }
        }    
    }
}

fn ls(state: &mut State, arg: &str) {
    let files = arg.split('\n');
    for file in files {
        //println!("{:?}", file);
        let (size, name) = file.split_once(' ').unwrap();

        if let Some(location) = state.location.as_mut() {
            if !state.files[location.index].children.iter().any(
                |x| state.files[x.index].name == name) {
                // Add the new file to location
                let parent = Some(FileId{
                    index: location.index
                });

                let size = match size {
                    "dir" => 0,
                    s => s.parse().unwrap()
                };
                state.add_file(
                    String::from(name),
                    size,
                    parent,
                    vec![]
                );
            }
        }
    }
}


struct State {
    location: Option<FileId>,
    files: Vec<File>
}

#[derive(Clone, Copy)]
struct FileId {
    index: usize,
}

struct File {
    name: String,
    parent: Option<FileId>,
    size: u32,
    children: Vec<FileId>
}

impl State {
    pub fn find(&self, start: &FileId, name: &str) -> Option<FileId> {
        for child in self.files[start.index].children.iter() {
            if self.files[child.index].name == name {
                return Some(FileId { index: child.index });
            }
        };
        return None;        
    }
    pub fn total_size(&self, start: &FileId) -> u32 {
        let mut sum = self.files[start.index].size;
        for child in self.files[start.index].children.iter() {
            sum += self.total_size(child);
        };
        sum
    }
    pub fn add_file(&mut self, name: String, size: u32, parent: Option<FileId>, children: Vec<FileId>) -> FileId {
        let next_index = self.files.len();
        
        self.files.push(File {
            name,
            size,
            parent,
            children
        });

        if let Some(parent) = parent {
            self.files[parent.index].children.push(FileId { index: next_index });
        }

        FileId {
            index: next_index
        }
    }
}