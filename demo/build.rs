extern crate pocket_resources;

fn main() {
    pocket_resources::package(&[
        ("resources", "a.txt"),
        ("resources", "subdir1/b.txt"),
        ("resources", "subdir1/d"),
        ("resources", "subdir2/c.txt"),
        ("resources", "subdir2/e.txt1"),
        ("resources", "subdir2/e.txt2"),
    ]).unwrap();
}
