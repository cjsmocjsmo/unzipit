// use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn walk_images_dir_for_zip_files(apath: String) -> Vec<String> {
    let mut keeper_vec = Vec::new();
    let mut idx = 0;
    let keeplist = [
        "gz", "GZ", "zip", "ZIP", "bz2", "BZ2",
    ];

    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if keeplist.contains(&ext) {
                    keeper_vec.push(fname.clone());
                };
            };
        };
    }
    println!("Total files: {}\n", idx);

    keeper_vec
}


pub fn walk_zip_dir(apath: String) ->  Vec<String> {
    let mut idx = 0;
    let mut ziplist = Vec::new();
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if ext == &"zip" || ext == &"ZIP" {
                    ziplist.push(fname.clone());
                }
            };
        };
    }
    println!("Total files: {}\n", idx);

    ziplist
}


pub fn walk_gz_dir(apath: String) ->  Vec<String> {
    let mut idx = 0;
    let mut gzlist = Vec::new();
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if ext == &"gz" || ext == &"GZ" {
                    gzlist.push(fname.clone());
                }
            };
        };
    }
    println!("Total files: {}\n", idx);

    gzlist
}
pub fn walk_bz2_dir(apath: String) ->  Vec<String> {
    let mut idx = 0;
    let mut bz2list = Vec::new();
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if ext == &"bz2" || ext == &"BZ2" {
                    bz2list.push(fname.clone());
                }
            };
        };
    }
    println!("Total files: {}\n", idx);

    bz2list
}

pub fn get_ext_list(apath: String) ->  Vec<String> {
    let mut idx = 0;
    let mut extlist = Vec::new();
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            idx += 1;
            let fname = e.path().to_string_lossy().to_string();
            let path = Path::new(&fname);
            if path.is_file() {
                let parts = &fname.split(".").collect::<Vec<&str>>();
                let ext = parts.last().unwrap();
                if !extlist.contains(&ext.to_string()) {
                    extlist.push(ext.to_string());
                }
            };
        };
    }

    println!("Total files: {}\n", idx);

    extlist
}