use std::ascii::AsciiExt;
use std::env;
use std::io;
use std::io::Write;
use std::fs::File;
use std::iter::IntoIterator;
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
struct Entry {
    path: Vec<String>,
    struct_name: String,
    struct_name_no_ext: String,
    resource_str_name: String,
    resource_str_name_no_ext: String,
    file_path: String,
    enum_name: String,
}

pub fn package<'a, I, P1, P2>(files: I) -> io::Result<()>
    where I: IntoIterator<Item = &'a (P1, P2)>, P1: AsRef<Path> + 'a, P2: AsRef<Path> + 'a
{
    let entries = files.into_iter().map(|&(ref base_dir, ref file)| {
        let base_dir = base_dir.as_ref();
        let file = file.as_ref();

        println!("cargo:rerun-if-changed={}/{}", base_dir.display(), file.display());

        Entry {
            path: file.parent().into_iter().flat_map(|p| p.iter()).map(|val| {
                let val = val.to_str().expect("Cannot process non-UTF8 path");
                val.chars().filter(|c| c.is_alphanumeric()).collect::<String>()
            }).collect(),
            struct_name: {
                let val = file.file_name().unwrap();
                let val = val.to_os_string().into_string().unwrap();
                let val = val.chars().filter_map(|c| if c.is_alphanumeric() || c == '_' { Some(c) } else if c == '.' { Some('_') } else { None }).collect::<String>();
                val.to_ascii_uppercase()
            },
            struct_name_no_ext: {
                let val = file.file_stem().unwrap();
                let val = val.to_os_string().into_string().unwrap();
                let val = val.chars().filter_map(|c| if c.is_alphanumeric() || c == '_' { Some(c) } else if c == '.' { Some('_') } else { None }).collect::<String>();
                val.to_ascii_uppercase()
            },
            resource_str_name: file.display().to_string(),
            resource_str_name_no_ext: if file.iter().count() == 1 {
                file.file_stem().unwrap().to_os_string().into_string().unwrap()
            } else {
                file.parent().unwrap().display().to_string() + "/" + &file.file_stem().unwrap().to_os_string().into_string().unwrap()
            },
            file_path: base_dir.join(file).display().to_string(),
            enum_name: path_to_enum_variant(file),
        }
    }).collect::<Vec<_>>();

    let file_path = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&file_path).join("pocket-resources.rs");
    let mut file = File::create(&file_path).unwrap();

    try!(writeln!(file, r#"pub enum ResourceId {{"#));
    for entry in &entries { try!(writeln!(file, r"{},", entry.enum_name)); }
    try!(writeln!(file, r#"}}"#));

    try!(writeln!(file, r#"impl ResourceId {{"#));
    try!(writeln!(file, r#"    #[inline]"#));
    try!(writeln!(file, r#"    pub fn load(&self) -> &'static [u8] {{"#));
    try!(writeln!(file, r#"        match self {{"#));
    for entry in &entries {
        try!(writeln!(file, r##"
                &ResourceId::{} => &include_bytes!(r#"{}/{}"#)[..],
            "##, entry.enum_name, env::var("CARGO_MANIFEST_DIR").unwrap(), entry.file_path));
    }
    try!(writeln!(file, r#"        }}"#));
    try!(writeln!(file, r#"    }}"#));
    try!(writeln!(file, r#"    #[inline]"#));
    try!(writeln!(file, r#"    pub fn from_name(name: &str) -> Option<ResourceId> {{"#));
    for entry in &entries {
        try!(writeln!(file, r##"
                if name == r#"{}"# {{ return Some(ResourceId::{}); }}
            "##, entry.resource_str_name, entry.enum_name));

        if entry.resource_str_name != entry.resource_str_name_no_ext {
            if entries.iter().filter(|e| e.resource_str_name_no_ext == entry.resource_str_name_no_ext || e.resource_str_name == entry.resource_str_name_no_ext).count() == 1 {
            try!(writeln!(file, r##"
                    if name == r#"{}"# {{ return Some(ResourceId::{}); }}
                "##, entry.resource_str_name_no_ext, entry.enum_name));
            }
        }
    }
    try!(writeln!(file, r#"        None"#));
    try!(writeln!(file, r#"    }}"#));
    try!(writeln!(file, r#"}}"#));

    try!(write(&entries, &[], &mut file));
    Ok(())
}

fn write<W>(entries: &[Entry], base: &[String], output: &mut W) -> io::Result<()>
    where W: Write
{
    let mut sub_paths = HashSet::new();

    for entry in entries {
        if entry.path.len() > base.len() && &entry.path[..base.len()] == base {
            sub_paths.insert(&entry.path[base.len()]);
        }

        if entry.path != base {
            continue;
        }

        try!(write!(output, "#[allow(missing_docs)] pub const {}: ", entry.struct_name));
        for _ in 0 .. base.len() { try!(write!(output, r"super::")); }
        try!(write!(output, "ResourceId = "));
        for _ in 0 .. base.len() { try!(write!(output, r"super::")); }
        try!(writeln!(output, r"ResourceId::{};", entry.enum_name));

        if entry.struct_name != entry.struct_name_no_ext {
            if entries.iter().filter(|e| e.struct_name_no_ext == entry.struct_name_no_ext || e.struct_name == entry.struct_name_no_ext).count() == 1 {
                try!(write!(output, "#[allow(missing_docs)] pub const {}: ", entry.struct_name_no_ext));
                for _ in 0 .. base.len() { try!(write!(output, r"super::")); }
                try!(write!(output, "ResourceId = "));
                for _ in 0 .. base.len() { try!(write!(output, r"super::")); }
                try!(writeln!(output, r"ResourceId::{};", entry.enum_name));
            }
        }
    }

    for sub_path in sub_paths.iter() {
        try!(writeln!(output, r#"
            #[allow(missing_docs)]
            pub mod {} {{
        "#, sub_path));

        let mut base = base.to_vec();
        base.push(sub_path.to_string());
        try!(write(entries, &base, output));

        try!(writeln!(output, r#"
            }}
        "#));
    }
    
    Ok(())
}

/// Turns a path into a variant name for the enumeration of resources.
fn path_to_enum_variant<P>(path: P) -> String where P: AsRef<Path> {
    let path = path.as_ref();

    let components = path.iter()
                         .map(|val| {
                             let val = val.to_str().expect("Cannot process non-UTF8 path");
                             let val = val.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                             format!("{}{}", val[..1].to_ascii_uppercase(), val[1..].to_ascii_lowercase())
                         }).collect::<Vec<_>>();

    components.concat()
}
