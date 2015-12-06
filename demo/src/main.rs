include!(concat!(env!("OUT_DIR"), "/pocket-resources.rs"));

fn main() {
    println!("Content of a.txt: {:?}",
            String::from_utf8(ResourceId::Atxt.load().to_vec()));

    println!("Content of a.txt: {:?}",
            String::from_utf8(ResourceId::from_name("a").unwrap().load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(ResourceId::Subdir1Btxt.load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(subdir1::B.load().to_vec()));

    println!("Content of b.txt: {:?}",
            String::from_utf8(ResourceId::from_name("subdir1/b").unwrap().load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(subdir2::C_TXT.load().to_vec()));

    println!("Content of c.txt: {:?}",
            String::from_utf8(ResourceId::from_name("subdir2/c").unwrap().load().to_vec()));
}
