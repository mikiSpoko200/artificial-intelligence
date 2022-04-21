use std::path::Path;


pub struct BlockSpec(Vec<u8>);


pub struct RowSpec(Vec<BlockSpec>);


pub struct Nonogram {
    image: Vec<Vec<u8>>,
    row_block_spec: RowSpec,
    col_block_spec: RowSpec,
}

impl<'a, T: Into<&'a Path>> From<T> for Nonogram {
    fn from(path: T) -> Self {
        
    }
}


fn main() {
    println!("Hello, world!");
}
