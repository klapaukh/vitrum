use file_loader;

fn main() {
    let filename = "data/cube.stl";

    println!("You have selected the file {} to open", filename);

    match file_loader::load_file(filename) {
        Err(a) => println!("File loading failed with {:?}", a),
        Ok(l) => println!("File loaded with {:?}", l)
    }
}
