#[derive(Clone)]
struct Block {
    id: Option<usize>,
    size: u8,
}
impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match self.id {
            Some(id) => id.to_string(),
            None => String::from("."),
        };

        write!(f, "{}", id.repeat(self.size as usize))
    }
}

#[aoc2024::main(09)]
fn solution(input: &str) -> (usize, usize) {
    let mut items = vec![];
    for (loc, item) in input.chars().map(|item| item as u8 - 48).enumerate() {
        if loc % 2 == 0 {
            items.push(Block {
                id: Some(loc / 2),
                size: item,
            });
        } else {
            // don't crate a block for non empty space
            if item > 0 {
                items.push(Block {
                    id: None,
                    size: item,
                });
            }
        }
    }

    let mut items2 = items.clone();

    // get the last item, get the first free space item
    loop {
        let Some(free) = items.iter().position(|blk| blk.id.is_none()) else {
            break;
        };
        let mut free_block = items[free].clone();
        let mut last_block = items.pop().unwrap().clone();

        // don't relocate empty blocks
        if last_block.id.is_none() {
            continue;
        }

        if last_block.size == free_block.size {
            // update the free block
            free_block.id = last_block.id;
            items[free] = free_block;
        } else if last_block.size > free_block.size {
            if free == items.len() - 1 {
                // if our free block is at the end, move the last block to this location
                items[free] = last_block;
            } else {
                // update the free block and place the remainder back
                free_block.id = last_block.id;
                last_block.size = last_block.size - free_block.size;
                items[free] = free_block;
                items.push(last_block.clone());
            }
        } else if last_block.size < free_block.size {
            // upate the free block and insert a new block for the remaining space
            let new_block = Block {
                id: None,
                size: free_block.size - last_block.size,
            };

            free_block.id = last_block.id;
            free_block.size = last_block.size;
            items[free] = free_block;

            items.insert(free + 1, new_block);
        }
    }

    // part 2
    // let mut defragged = vec![];
    // let mut moved = HashSet::new();
    let mut curr_idx = items2.len() - 1;
    while curr_idx > 1 {
        let mv_blk = items2[curr_idx].clone();
        curr_idx -= 1;

        // don't move empty space
        if mv_blk.id.is_none() {
            continue;
        }

        // look if we have space from the left, before this index
        let Some(free) = items2.iter().enumerate().position(|(lok, blk)| {
            blk.id.is_none() && blk.size >= mv_blk.size && lok < curr_idx + 1
        }) else {
            continue; // do next
        };

        let free_block = items2[free].clone();

        if mv_blk.size == free_block.size {
            // update the free block
            items2[free] = mv_blk;
            items2[curr_idx + 1].id = None;
        } else if mv_blk.size < free_block.size {
            // upate the free block and insert a new block for the remaining space
            let new_block = Block {
                id: None,
                size: free_block.size - mv_blk.size,
            };
            items2[free] = mv_blk;
            items2[curr_idx + 1].id = None;
            items2.insert(free + 1, new_block);
            curr_idx += 1;
        }
    }

    let mut current: usize = 0;
    let checksum: usize = items
        .iter()
        .map(|item| {
            let mut total = 0;
            for ci in current..(current + item.size as usize) {
                if let Some(id) = item.id {
                    total += ci as usize * id
                }
            }
            current += item.size as usize;
            total
        })
        .sum();

    let mut current: usize = 0;
    let checksum2: usize = items2
        .iter()
        .map(|item| {
            let mut total = 0;
            for ci in current..(current + item.size as usize) {
                if let Some(id) = item.id {
                    total += ci as usize * id
                }
            }
            current += item.size as usize;
            total
        })
        .sum();

    (checksum, checksum2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 1928);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 2858);
        Ok(())
    }
}
