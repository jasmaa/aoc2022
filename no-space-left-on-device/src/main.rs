use std::io::{self, BufReader};

mod filesystem;

fn main() {
    let stdin = io::stdin();
    let r = BufReader::new(stdin.lock());
    let line_parses = filesystem::lines::parse(r);

    let root_ref = filesystem::tree::parse(line_parses).unwrap();
    println!(
        "Summed size of all directories with size <= 100000: {}",
        filesystem::tree::find_directories_lte_threshold(root_ref.clone(), 100000)
            .iter()
            .map(|node| { node.borrow().total_size() })
            .sum::<u64>()
    );
    println!(
        "Size of smallest removable directory: {}",
        filesystem::tree::find_smallest_removable_directory(root_ref.clone(), 70000000, 30000000)
            .unwrap()
            .borrow()
            .total_size(),
    );
}
