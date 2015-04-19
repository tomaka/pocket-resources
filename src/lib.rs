#![feature(collections, path_ext, path_relative_from)]

use std::env;
use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::fs::PathExt;
use std::iter::IntoIterator;
use std::path::{Component, Path};
use std::slice::SliceConcatExt;

pub fn package<I>(directories: I, remove_extensions: bool) -> io::Result<()>
                  where I: IntoIterator, I::Item: AsRef<Path>
{
    let mut enum_output = format!(r#"
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Resource {{
            "#);

    let mut if_clause = format!("");
    let mut load_clause = format!("");

    for directory in directories.into_iter() {
        try!(visit_dirs(&directory, &mut |original_entry| {
            let original_entry = original_entry.path();
            let entry = original_entry.relative_from(&directory).unwrap();

            let res_name = path_to_resource_name(entry, remove_extensions);
            let enum_variant = path_to_enum_variant(entry, remove_extensions);

            enum_output.push_str(&format!(r#"
                /// {}
                {},
            "#, res_name, enum_variant));

            if_clause.push_str(&format!(r##"
                if name == r#"{}"# {{
                    Some(Resource::{})
                }} else 
            "##, res_name, enum_variant));

            load_clause.push_str(&format!(r##"
                &Resource::{} => include_bytes!(r#"{}"#),
            "##, enum_variant, env::current_dir().unwrap().join(&original_entry).display()));
        }));
    }

    enum_output.push_str("}");

    let file_path = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&file_path).join("pocket-resources.rs");
    let mut file = File::create(&file_path).unwrap();
    try!(writeln!(file.by_ref(), r#"
        {en}

        impl Resource {{
            pub fn from_name(name: &str) -> Option<Resource> {{
                {if_clause} {{
                    None
                }}
            }}

            pub fn load(&self) -> &'static [u8] {{
                match self {{
                    {load_clause}
                }}
            }}
        }}
    "#, en = enum_output, if_clause = if_clause, load_clause = load_clause));

    Ok(())
}

fn visit_dirs<P, C>(dir: P, mut cb: &mut C) -> io::Result<()>
                    where P: AsRef<Path>, C: FnMut(fs::DirEntry)
{
    let dir = dir.as_ref();

    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if entry.path().is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
                cb(entry);
            }
        }
    }

    Ok(())
}

/// Turns a path into a variant name for the enumeration of resources.
fn path_to_enum_variant<P>(path: P, remove_extensions: bool) -> String where P: AsRef<Path> {
    let path = path.as_ref();

    let components = path.parent().into_iter().flat_map(|p| p.iter())
                         .chain(if remove_extensions {
                             path.file_stem().into_iter()
                         } else {
                             path.file_name().into_iter()
                         })
                         .map(|val| {
                             let val = val.to_str().expect("Cannot process non-UTF8 path");
                             let val = val.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                             format!("{}{}", val[..1].to_uppercase(), val[1..].to_lowercase())
                         }).collect::<Vec<_>>();

    components.concat()
}

/// Turns a path into a resource name usable by the program.
fn path_to_resource_name<P>(path: P, remove_extensions: bool) -> String where P: AsRef<Path> {
    let path = path.as_ref();

    path.parent()
        .into_iter()
        .flat_map(|p| p.components().map(|component| {
            match component {
                Component::Prefix(_) => unreachable!(),
                Component::RootDir => unreachable!(),
                Component::CurDir => unreachable!(),
                Component::ParentDir => unreachable!(),
                Component::Normal(s) => s.to_str().expect("Cannot process non-UTF8 path"),
            }
        }))
        .chain(if remove_extensions {
            path.file_stem().map(|v| v.to_str().unwrap()).into_iter()
        } else {
            path.file_name().map(|v| v.to_str().unwrap()).into_iter()
        })
        .collect::<Vec<_>>()
        .connect("/")
}
