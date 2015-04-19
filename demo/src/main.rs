include!(concat!(env!("OUT_DIR"), "/pocket-resources.rs"));

fn main() {
    println!("Content of a.txt: {:?}",
            String::from_utf8(Resource::A.load().to_vec()));

    println!("Content of a.txt: {:?}",
            String::from_utf8(Resource::from_name("a").unwrap().load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(Resource::Subdir1B.load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(Resource::from_name("subdir1/b").unwrap().load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(Resource::Subdir2C.load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(Resource::from_name("subdir2/c").unwrap().load().to_vec()));
}
