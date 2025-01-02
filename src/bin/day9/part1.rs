enum Block {
    File(u32),
    Empty,
}

fn parse_blocks(disk: &Vec<u8>) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut is_file = true;
    let mut file_id = 0;
    for size in disk {
        for _ in 0..*size {
            if is_file {
                blocks.push(Block::File(file_id));
            } else {
                blocks.push(Block::Empty);
            }
        }

        if is_file {
            file_id += 1;
        }

        is_file = !is_file;
    }

    blocks
}

pub(super) fn part1(disk: &Vec<u8>) -> usize {
    let mut blocks = parse_blocks(disk);

    let mut front = 0;
    while front < blocks.len() && !matches!(blocks[front], Block::Empty) {
        front += 1;
    }

    let mut back = blocks.len() - 1;
    while 0 < back && !matches!(blocks[back], Block::File(_)) {
        back -= 1;
    }

    while front < back {
        blocks.swap(front, back);

        // find next empty space
        while front < blocks.len() && !matches!(blocks[front], Block::Empty) {
            front += 1;
        }

        // last element
        while 0 < back && !matches!(blocks[back], Block::File(_)) {
            back -= 1;
        }
    }

    let mut checksum = 0;
    for (pos, block) in blocks.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += (*id as usize) * pos;
        }
    }

    checksum
}
