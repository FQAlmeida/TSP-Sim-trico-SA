pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct DataNode {
    pub group: u8,
    pub point: Point,
}

pub type Data = Vec<DataNode>;

pub fn load(fp: &'static str) -> Data {
    let mut data = vec![];
    let fd = std::fs::read_to_string(fp).expect("To be able to open the file");
    let lines = fd.lines();
    for line in lines {
        let items = line.split_whitespace().collect::<Vec<&str>>();
        // println!("{}", line.to_string());
        let group: u8 = items[0].parse().unwrap();
        let x: usize = items[1].parse().unwrap();
        let y: usize = items[2].parse().unwrap();
        // println!("|{}\t||{}\t||{}|", x, y, group);
        data.push(DataNode {
            group,
            point: Point { x, y },
        });
    }

    return data;
}

#[cfg(test)]
mod tests {
    use crate::load;

    #[test]
    fn path_100_has_100_items() {
        let data = load("../data/inst_100.txt");
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn path_100_item_54_x_is_2945_y_is_1622() {
        let data = load("../data/inst_100.txt");
        assert_eq!(data[54].group, 55);
        assert_eq!(data[54].point.x, 2945);
        assert_eq!(data[54].point.y, 1622);
    }
}
