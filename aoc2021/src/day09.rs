use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct HeightMap {
    map_width: usize,
    points: Vec<u8>
}
impl HeightMap {
    fn new(map: Vec<String>) -> Self {
        let mut map_width = 0;
        let mut points = vec!();

        for line in map {
            map_width = line.len();
            points.append(&mut line.chars().map(|item| item.to_digit(10).unwrap() as u8).collect())
        }
        Self {
           map_width,
           points
        }
    }

    fn get_item_above(&self, index: usize) -> u8 {
        if index < self.map_width {
            return 9;
        }
        *self.points.get(index-self.map_width).unwrap_or(&9)
    }
    fn get_item_below(&self, index: usize) -> u8 {
        *self.points.get(index+self.map_width).unwrap_or(&9)
    }
    fn get_item_left(&self, index: usize) -> u8 {
        let left = index%self.map_width;
        if left == 0 {
            9
        } else {
            *self.points.get(index-1).unwrap_or(&9)
        }
    }
    fn get_item_right(&self, index: usize) -> u8 {
        let left = index+1%self.map_width;
        if left == 0 {
            9
        } else {
            *self.points.get(index+1).unwrap_or(&9)
        }
    }

    fn get_lowest_points(&self) -> Vec<u8> {
        let mut res = vec!();

        for (idx, item) in self.points.iter().enumerate() {
            // get our neighbours
            let above:u8 = self.get_item_above(idx);
            let left:u8 = self.get_item_left(idx);
            let right:u8 = self.get_item_below(idx);
            let below:u8 = self.get_item_right(idx);

            if *item < above && *item < left && *item < right && *item < below {
                res.push(*item);
            }
        }
        res
    }

    fn get_lowest_point_index(&self) -> Vec<usize> {
        let mut res = vec!();

        for (idx, item) in self.points.iter().enumerate() {
            // get our neighbours
            let above:u8 = self.get_item_above(idx);
            let left:u8 = self.get_item_left(idx);
            let right:u8 = self.get_item_below(idx);
            let below:u8 = self.get_item_right(idx);

            if *item < above && *item < left && *item < right && *item < below {
                res.push(idx);
            }
        }
        res
    }

    fn get_basins(&mut self) -> Vec<Vec<u8>>{
        // loop over all points and collect the bassins
        let mut bassins: Vec<Vec<u8>> = vec!();
        for idx in self.get_lowest_point_index() {
            let res = self.get_bassin_for(idx);
            if !res.is_empty() {
                bassins.push(res);
            }
        }

        bassins
    }

    fn get_bassin_for(&mut self, index: usize) -> Vec<u8> {
        let mut ret: Vec<u8> = vec!();
        // if the item is 9 we are done
        if *self.points.get(index).unwrap() == 9 {
            return ret;
        }

        // add ourself to the vec, and make sure we are updated so
        // we won't be added again...
        ret.push(*self.points.get(index).unwrap());
        self.points[index] = 9;

        // add our neighbours if they fit..
        if index >= self.map_width {
            ret.append(&mut self.get_bassin_for(index-self.map_width));
        }
        if index+self.map_width < self.points.len() {
            ret.append(&mut self.get_bassin_for(index+self.map_width));
        }
        if index%self.map_width != 0 && index > 0 {
            ret.append(&mut self.get_bassin_for(index-1));
        }
        if (index+1)%self.map_width != 0 && index < self.points.len() {
            ret.append(&mut self.get_bassin_for(index+1));
        }

        ret
    }
}

pub fn part1(input: Vec<String>) -> u32 {
    let map = HeightMap::new(input);
    map.get_lowest_points().iter().map(|i| (i+1) as u32).collect::<Vec<u32>>().iter().sum()
}

pub fn part2(input: Vec<String>) -> u32 {
    let mut map = HeightMap::new(input);
    let mut basins = map.get_basins();

    basins.sort_by(|v1, v2| v2.len().cmp(&v1.len()));
    basins[0].len() as u32 * basins[1].len() as u32 * basins[2].len() as u32
}
