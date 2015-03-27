include!(concat!(env!("OUT_DIR"), "/pocket-resources.rs"));

fn main() {
    println!("Content of a.txt: {:?}",
            String::from_utf8(Resource::Atxt.load().to_vec()));

    println!("Content of a.txt: {:?}",
            String::from_utf8(Resource::from_name("a.txt").unwrap().load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(Resource::Subdir1Btxt.load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(Resource::from_name("subdir1/b.txt").unwrap().load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(Resource::Subdir2Ctxt.load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(Resource::from_name("subdir2/c.txt").unwrap().load().to_vec()));
}
