struct File {
    id: i32,
    size: u32,
    moved: bool,
}

enum Entry {
    Empty(u32),
    File(File),
}

impl Entry {
    fn file(&self) -> Option<File> {
        if let Entry::File(f) = self {
            Some(File {
                id: f.id,
                size: f.size,
                moved: f.moved,
            })
        } else {
            None
        }
    }

    fn empty(&self) -> Option<u32> {
        if let Entry::Empty(space) = self {
            Some(*space)
        } else {
            None
        }
    }
}

fn parse_entries(disk: &Vec<u8>) -> Vec<Entry> {
    let mut entries = Vec::new();

    let mut is_file = true;
    let mut id = 0;
    for size in disk {
        let entry = if is_file {
            Entry::File(File {
                id,
                size: *size as u32,
                moved: false,
            })
        } else {
            Entry::Empty(*size as u32)
        };

        if is_file {
            id += 1;
        }

        is_file = !is_file;
        entries.push(entry);
    }

    entries
}

fn move_entry(entries: &mut Vec<Entry>, empty_space_index: usize, file_index: usize) {
    let free_space = entries[empty_space_index]
        .empty()
        .expect("expected empty space");

    let file = entries[file_index].file().expect("expected file");
    let remaining_space = free_space - file.size;

    // move file into space
    entries[empty_space_index] = Entry::File(File {
        id: file.id,
        size: file.size,
        moved: true,
    });

    // overwrite file with space
    entries[file_index] = Entry::Empty(file.size);

    // if there is remaining space we need to make sure it is preserved
    if remaining_space > 0 {
        entries.insert(empty_space_index + 1, Entry::Empty(remaining_space));
    }
}

fn create_block_list(entries: &[Entry]) -> Vec<i32> {
    let mut blocks = Vec::new();

    for e in entries {
        match e {
            Entry::File(f) => {
                for _ in 0..f.size {
                    blocks.push(f.id);
                }
            }
            Entry::Empty(count) => {
                for _ in 0..*count {
                    blocks.push(-1);
                }
            }
        }
    }

    blocks
}

fn calculate_checksum(blocks: &[i32]) -> i64 {
    let mut checksum = 0;

    for (index, block) in blocks.iter().enumerate() {
        let index64: i64 = index.try_into().expect("index overflowed");

        if *block >= 0 {
            checksum += (*block as i64) * index64;
        }
    }

    checksum
}

pub(crate) fn part2(disk: &Vec<u8>) -> i64 {
    let mut entries = parse_entries(disk);
    let mut back = entries.len() - 1;

    while back > 0 {
        // if empty space then move left
        if matches!(entries[back], Entry::Empty(_)) {
            back -= 1;
            continue;
        }

        let file = entries[back].file().unwrap();

        // this file has already been moved
        if file.moved {
            back -= 1;
            continue;
        }

        // find the left-most free-space that will fit the current item
        let mut front = 0;
        while front < back && !matches!(entries[front], Entry::Empty(size) if size >= file.size) {
            front += 1;
        }

        // no suitable free-space
        if front == back {
            back -= 1;
            continue;
        }

        move_entry(&mut entries, front, back);
    }

    let blocks = create_block_list(&entries);
    let checksum = calculate_checksum(&blocks);

    checksum
}
