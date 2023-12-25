use std::{env, error::Error, fs::File, path::PathBuf};

fn main() {
    for argument in env::args().skip(1) {
        let _ = write_new_file(argument);
    }
}

fn write_new_file(file_name: String) -> Result<(), Box<dyn Error>> {
    let path_buf: PathBuf;
    match env::current_dir() {
        Ok(dir) => path_buf = dir,
        Err(err) => panic!(
            "Couldn't determine current directory!, {:?}",
            err.to_string()
        ),
    }

    let path = path_buf.join(file_name);

    match File::create(path.as_path()) {
        Ok(_file) => return Ok(()),
        Err(err) => println!("Error occured when writing file, {}", err.to_string()),
    }

    return Ok(());
}
