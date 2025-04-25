use std::env;
use std::os::windows::process;
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::error::Error;
// use macroquad::prelude::*;

const SKIN_MAGIC: u32  = 0x4E494B53; // 'SKIN'
const KPACK_MAGIC: u32 = 0x4B43504B; // 'KPACK'

#[derive(Debug)]
struct Skin {
    version: u32,
    params: u32,
    buttons: u32,
    bitmaps: u32,
    // TODO ....
}

fn read_skin_file(file_path: &Path) -> Result<Skin, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut skin_data = &buffer[..];

    // TODO add kpacked files processing
    // let magic = skin_data.read_u32::<LittleEndian>()?;
    // if magic == KPACK_MAGIC {
    //     skin_data = &unpack(skin_data)?;
    // }

    let magic = skin_data.read_u32::<LittleEndian>()?;
    if magic != SKIN_MAGIC {
        return Err("The uploaded file is not a skin!".into());
    }

    let version = skin_data.read_u32::<LittleEndian>()?;
    let params = skin_data.read_u32::<LittleEndian>()?;
    let buttons = skin_data.read_u32::<LittleEndian>()?;
    let bitmaps = skin_data.read_u32::<LittleEndian>()?;

    // TODO parse further

    Ok(Skin {
        version,
        params,
        buttons,
        bitmaps,
    })
}


// #[macroquad::main("KolibriOS skin viewer")] async
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", Path::new(&args[0]).file_name().unwrap().to_str().unwrap());
        std::process::exit(1);
    }

    let input_file_path = Path::new(&args[1]);

    if !input_file_path.exists() || !input_file_path.is_file() {
        println!("File '{}' does not exist", input_file_path.display());
    }

    let skin_obj = match read_skin_file(input_file_path) {
        Ok(s) => {s},
        Err(e) => {
            println!("Error {e}");
            std::process::exit(1);
        }
    };

    println!("skin_obj = {:#?} ", skin_obj);


    // loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    //     draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    //     next_frame().await
    // }
}
