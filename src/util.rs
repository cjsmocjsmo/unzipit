use std::fs;
use std::path::Path;
use walkdir::WalkDir;
// use bzip2::read::BzDecoder;
// use flate2::read::GzDecoder;
// use md5::compute;
// use tar::Archive;
// use zip::ZipArchive;

pub fn prep_env() {
    let av_path = "/media/pipi/0123-4567/AV/";
    let converted_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Converted/";
    let toremove_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/ToRemove/";
    let b_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/BadImages/";
    let master_path = "/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master/";
    let gz1_path = "/media/pipi/0123-4567/GZ1/";
    let zip_path = "/media/pipi/0123-4567/ZIP/";
    let bz2_path = "/media/pipi/0123-4567/BZ2/";
    let bad_path = "/media/pipi/0123-4567/BadImages/";
    let gz1_unzip_path = "/media/pipi/0123-4567/Images/GZ1_Unzip/";
    let gz2_unzip_path = "/media/pipi/0123-4567/Images/GZ2_Unzip/";
    let zip_unzip_path = "/media/pipi/0123-4567/Images/ZIP_Unzip/";
    let bz2_unzip_path = "/media/pipi/0123-4567/Images/BZ2_Unzip/";

    let mut zip_list = Vec::new();
    zip_list.push(av_path);
    zip_list.push(converted_path);
    zip_list.push(toremove_path);
    zip_list.push(gz1_path);
    zip_list.push(zip_path);
    zip_list.push(bz2_path);
    zip_list.push(gz1_unzip_path);
    zip_list.push(gz2_unzip_path);
    zip_list.push(zip_unzip_path);
    zip_list.push(bz2_unzip_path);
    zip_list.push(bad_path);
    zip_list.push(b_path);
    zip_list.push(master_path);

    let _zlist = zip_list
        .iter()
        .map(|x| pf_create_dir(x))
        .collect::<Vec<_>>();
}

pub fn pf_create_dir(apath: &str) {
    let save_dir = Path::new(apath);
    if fs::metadata(save_dir).is_ok() {
        println!("Directory exists: {}", apath)
    } else {
        fs::create_dir_all(save_dir).expect("Unable to create directory");
    }
}

pub fn mv_zip_files(fname: String) {

    for e in WalkDir::new(fname)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let parts = &fname.split(".").collect::<Vec<&str>>();
            let ext = parts.last().unwrap();

            if ext == &"zip" || ext == &"ZIP" {
                let fparts = fname.split("/").collect::<Vec<&str>>();
                let filename = fparts.last().unwrap().replace(" ", "_");
                let addr = "/media/pipi/0123-4567/ZIP/".to_string() + &filename;
                match fs::rename(&fname, &addr) {
                    Ok(_) => println!("Moved: {}", addr),
                    Err(e) => println!("ZIP_Error: {}", e),
                };
            } else if ext == &"gz" || ext == &"GZ" {
                let fparts = fname.split("/").collect::<Vec<&str>>();
                let filename = fparts.last().unwrap().replace(" ", "_");
                let addr = "/media/pipi/0123-4567/GZ1/".to_string() + &filename;
                match fs::rename(&fname, &addr) {
                    Ok(_) => println!("Moved: {}", addr),
                    Err(e) => println!("GZ_Error: {}", e),
                };
            } else if ext == &"bz2" || ext == &"BZ2" {
                let fparts = fname.split("/").collect::<Vec<&str>>();
                let filename = fparts.last().unwrap().replace(" ", "_");
                let addr = "/media/pipi/0123-4567/BZ2/".to_string() + &filename;
                match fs::rename(&fname, &addr) {
                    Ok(_) => println!("Moved: {}", addr),
                    Err(e) => println!("BZ_Error: {}", e),
                };
            } else if ext == &"tar" {
                let fparts = fname.split("/").collect::<Vec<&str>>();
                let filename = fparts.last().unwrap();
                if fname == "P-009.tar" {
                    std::fs::remove_file(fname.clone()).unwrap();
                } else {
                    let addr = "/media/pipi/0123-4567/BZ2/".to_string() + &filename;
                    match fs::rename(&fname, &addr) {
                        Ok(_) => println!("Moved: {}", addr),
                        Err(e) => println!("BZ_Error: {}", e),
                    };
                }
            }

            println!("Moved: {}", fname.clone())
        }
    }
}
