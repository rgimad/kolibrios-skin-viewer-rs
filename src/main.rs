use std::env;
use std::os::windows::process;
use std::path::Path;
use std::io::Cursor;
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
    margin: SkinMargin,
    active: SkinFrameColors,
    inactive: SkinFrameColors,
    system_colors: SkinSystemColors,
    // TODO ....
}

#[derive(Debug)]
struct SkinMargin {
    right: u16,
    left: u16,
    bottom: u16,
    top: u16,
}

#[derive(Debug)]
struct SkinFrameColors {
    inner: u32,
    outer: u32,
    frame: u32,
}

#[derive(Debug)]
struct SkinSystemColors {
    taskbar: u32,
    taskbar_text: u32,
    work_dark: u32,
    work_light: u32,
    window_title: u32,
    work: u32,
    work_button: u32,
    work_button_text: u32,
    work_text: u32,
    work_graph: u32,
}

fn read_skin_file(file_path: &Path) -> Result<Skin, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let skin_data = &buffer[..];

    let mut cursor1 = Cursor::new(skin_data);

    // TODO add kpacked files processing
    // let magic = skin_data.read_u32::<LittleEndian>()?;
    // if magic == KPACK_MAGIC {
    //     skin_data = &unpack(skin_data)?;
    // }

    let magic = cursor1.read_u32::<LittleEndian>()?;
    if magic != SKIN_MAGIC {
        return Err("The uploaded file is not a skin!".into());
    }

    let version = cursor1.read_u32::<LittleEndian>()?;
    let params = cursor1.read_u32::<LittleEndian>()?;
    let buttons = cursor1.read_u32::<LittleEndian>()?;
    let bitmaps = cursor1.read_u32::<LittleEndian>()?;

    // println!("params = {}", params as usize + 4);

    let mut cursor2 = Cursor::new(&skin_data[params as usize + 4..]);

    let margin_right = cursor2.read_u16::<LittleEndian>()?;
    let margin_left = cursor2.read_u16::<LittleEndian>()?;
    let margin_bottom = cursor2.read_u16::<LittleEndian>()?;
    let margin_top = cursor2.read_u16::<LittleEndian>()?;

    let active_inner = cursor2.read_u32::<LittleEndian>()?;
    let active_outer = cursor2.read_u32::<LittleEndian>()?;
    let active_frame = cursor2.read_u32::<LittleEndian>()?;

    let inactive_inner = cursor2.read_u32::<LittleEndian>()?;
    let inactive_outer = cursor2.read_u32::<LittleEndian>()?;
    let inactive_frame = cursor2.read_u32::<LittleEndian>()?;

    let _dtp_size = cursor2.read_u32::<LittleEndian>()?;
    println!("_dtp_size = {}", _dtp_size);
    let sc_taskbar = cursor2.read_u32::<LittleEndian>()?;
    let sc_taskbar_text = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_dark  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_light  = cursor2.read_u32::<LittleEndian>()?;
    let sc_window_title  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_button  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_button_text  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_text  = cursor2.read_u32::<LittleEndian>()?;
    let sc_work_graph  = cursor2.read_u32::<LittleEndian>()?;

    // TODO parse further

    Ok(Skin {
        version: version,
        params: params,
        buttons: buttons,
        bitmaps: bitmaps,
        margin: SkinMargin {
            right: margin_right,
            left: margin_left,
            bottom: margin_bottom,
            top: margin_top,
        },
        active: SkinFrameColors {
            inner: active_inner,
            outer: active_outer,
            frame: active_frame,
        },
        inactive: SkinFrameColors {
            inner: inactive_inner,
            outer: inactive_outer,
            frame: inactive_frame,
        },
        system_colors: SkinSystemColors {
            taskbar: sc_taskbar,
            taskbar_text: sc_taskbar_text,
            work_dark: sc_work_dark,
            work_light: sc_work_light,
            window_title: sc_window_title,
            work: sc_work,
            work_button: sc_work_button,
            work_button_text: sc_work_button_text,
            work_text: sc_work_text,
            work_graph: sc_work_graph,
        },
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

    println!("skin_obj = {:#X?} ", skin_obj);


    // loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    //     draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    //     next_frame().await
    // }
}
