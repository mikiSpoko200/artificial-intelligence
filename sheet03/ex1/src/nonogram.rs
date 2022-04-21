use std::fs;
use std::path::Path;
use crate::nonogram::Field::Filled;


#[derive(Clone, Copy)]
pub enum Field {
    Filled,
    Empty,
}

impl<T> From<T> for Field where T: Into<u8> {
    fn from(val: T) -> Self {
        match val.into() {
            0 => Self::Empty,
            1 => Self::Filled,
            other => panic!("unexpected field value. expected 0 or 1, got: {}", other),
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Filled
    }
}

impl BlockSpec {

    /// Checks if block with given size is valid block for given index.
    fn is_block_size_valid(&self, block_size: u8, index: usize) -> bool {
        if let Some(&expected_block_size) = self.0.get(index) {
            expected_block_size == block_size
        } else {
            false
        }
    }

    /// Checks node consistency of the current constraint.
    pub fn is_node_consistent(&self, row: &[Field]) -> bool {
        let mut block_count = 0;
        let mut temp_block_size = 0;
        let mut rows = row.iter();
        while let Some(&field) = rows.next() {
            match field {
                Field::Filled => {
                    if temp_block_size == 0 {
                        block_count += 1;
                        if block_count > self.0.len() {  /* Found more blocks than expected - false. */
                            return false;
                        }
                    }
                    temp_block_size += 1
                }
                Field::Empty => {
                    if temp_block_size != 0 {
                        if !self.is_block_size_valid(temp_block_size, block_count - 1) { /* We found a block that does not match expected size - false. */
                            return false;
                        }
                        temp_block_size = 0
                    }
                }
            }
        }
        if temp_block_size != 0 && !self.is_block_size_valid(temp_block_size, block_count - 1) { return false; }
        if block_count < self.0.len() { return false; }  /* We checked all fields in row and found too few blocks - false. */
        true  /* If non of the above triggered than the row is valid - true. */
    }
}

pub struct BlockSpec(pub Vec<u8>);

#[cfg(test)]
mod tests_row_consistency {
    use crate::nonogram::Field;
    use super::BlockSpec;

    fn block_from_vec(array: Vec<u8>) -> Vec<Field> {
        array.into_iter().map(<Field as From<u8>>::from).collect()
    }

    #[test]
    fn test_invalid_block_count_too_big() {
        let spec = BlockSpec(vec![3, 4]);
        let row = block_from_vec(vec![0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1]);
        assert!(!spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_invalid_block_count_too_small() {
        let spec = BlockSpec(vec![3, 4, 1]);
        let row = block_from_vec(vec![0, 1, 1, 1, 0, 1, 1, 1, 1, 0]);
        assert!(!spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_invalid_block_size_1() {
        let spec = BlockSpec(vec![3, 4]);
        let row = block_from_vec(vec![0, 1, 1, 1, 1, 0, 1, 1, 1, 0]);
        assert!(!spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_invalid_block_size_2() {
        let spec = BlockSpec(vec![1, 1, 1, 1, 1]);
        let row = block_from_vec(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
        assert!(!spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_invalid_block_size_3() {
        let spec = BlockSpec(vec![1]);
        let row = block_from_vec(vec![0, 0, 0, 0, 0, 1, 1, 0]);
        assert!(!spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_valid_1() {
        let spec = BlockSpec(vec![2, 4]);
        let row = block_from_vec(vec![0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0]);
        assert!(spec.is_node_consistent(row.as_slice()));
    }

    #[test]
    fn test_valid_2() {
        let spec = BlockSpec(vec![5]);
        let row = block_from_vec(vec![1, 1, 1, 1, 1]);
        assert!(spec.is_node_consistent(row.as_slice()));
    }
}

impl From<&str> for BlockSpec {
    fn from(spec_line: &str) -> Self {
        BlockSpec(spec_line
            .split_whitespace()
            .map(|number| number.parse().expect("Could not parse row specification."))
            .collect()
        )
    }
}



pub struct RowSpecConfig<'a, 'b: 'a, I> where I: Iterator<Item=&'b str> {
    block_count: usize,
    lines: &'a mut I,
}

impl<'a, 'b: 'a, I> RowSpecConfig<'a, 'b, I> where I: Iterator<Item=&'b str> {
    pub fn new(block_count: usize, lines: &'a mut I) -> Self { Self { block_count, lines } }
}

pub struct RowSpec(Vec<BlockSpec>);

impl<'a, 'b: 'a, I> From<RowSpecConfig<'a, 'b, I>> for RowSpec where I: Iterator<Item=&'b str> {
    fn from(spec_config: RowSpecConfig<'a, 'b, I>) -> Self {
        let mut row_spec = Vec::<BlockSpec>::with_capacity(spec_config.block_count);
        for line in spec_config.lines {
            row_spec.push(BlockSpec::from(line))
        }
        if row_spec.len() > spec_config.block_count {
            panic!("incorrect number of lines passed, expected: {}, got: {}", spec_config.block_count, row_spec.len());
        }
        Self(row_spec)
    }
}



struct NonogramConfiguration(usize, usize);

type Image = Vec<Vec<Field>>;

pub struct Iter<'a> {
    index: usize,
    image_ref: &'a Image,
}

impl<'a> Iter<'a> {
    pub fn new(image: &'a Image) -> Self {
        Self { index: 0, image_ref: image }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [Field];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.image_ref.len() {
            let ret = self.image_ref[self.index].as_slice();
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }
}

pub struct Nonogram {
    image: Vec<Vec<Field>>,
    image_transposed: Vec<Vec<Field>>,
    row_block_spec: RowSpec,
    col_block_spec: RowSpec,
}

impl Nonogram {
    pub fn new(image: Vec<Vec<Field>>, row_block_spec: RowSpec, col_block_spec: RowSpec) -> Self {
        let image_transposed = image.clone();
        Self { image, image_transposed, row_block_spec, col_block_spec }
    }

    pub fn rows(&self) -> Iter<'_> {
        Iter::new(&self.image)
    }

    pub fn cols(&self) -> Iter<'_> {
        Iter::new(&self.image_transposed)
    }

    fn parse_config_line(config_line: &str) -> NonogramConfiguration {
        let mut nonogram_size_iter = config_line
            .split_whitespace()
            .map(|word| word.parse::<usize>().expect("Could not parse Nonogram configuration line"));
        let width = nonogram_size_iter.next().expect("No width information found in Nonogram configuration.");
        let height = nonogram_size_iter.next().expect("No height information found in Nonogram configuration.");
        NonogramConfiguration(width, height)
    }
}

impl<'a, S> From<S> for Nonogram where S: Into<&'a str> {
    fn from(image: S) -> Self {
        let mut lines = image.into().lines();
        let config_line = lines.next().expect("Nonogram configuration line not found.");
        let NonogramConfiguration(width, height) = Nonogram::parse_config_line(config_line);

        let row_block_spec = RowSpec::from(RowSpecConfig::new(width, &mut lines));
        let col_block_spec = RowSpec::from(RowSpecConfig::new(height, &mut lines));
        let image = vec![vec![Default::default(); width]; height];

        Self::new(image, row_block_spec, col_block_spec)
    }
}

/* ehhhh: https://github.com/rust-lang/rust/issues/50133 */
struct E0119CircumventionWrapper<T>(T);

impl<'a, T> TryFrom<E0119CircumventionWrapper<T>> for Nonogram where T: Into<&'a Path> {
    type Error = std::io::Error;

    fn try_from(path: E0119CircumventionWrapper<T>) -> Result<Self, Self::Error> {
        let data = fs::read_to_string(path.0.into())?;
        Ok(Nonogram::from(data.as_str()))
    }
}
