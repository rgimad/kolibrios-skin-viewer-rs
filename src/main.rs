use std::env;
use std::os::windows::process;
use std::path::Path;
use std::io::Cursor;
use std::time::Duration;
use byteorder::{LittleEndian, ReadBytesExt};
use macroquad::miniquad::window::screen_size;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use macroquad::prelude::*;

const SKIN_MAGIC: u32  = 0x4E494B53; // 'SKIN'
const KPACK_MAGIC: u32 = 0x4B43504B; // 'KPACK'

#[derive(Debug)]
struct Skin {
    version: u32,
    margin: SkinMargin,
    active: SkinFrameColors,
    inactive: SkinFrameColors,
    system_colors: SkinSystemColors,
    buttons: Vec<SkinButton>,
    bitmaps: Vec<SkinBitmap>,
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

#[derive(Debug)]
struct SkinButton {
    btntype: u32,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct SkinBitmap {
    kind: u16,
    bmptype: u16,
    width: u32,
    height: u32,
    data: Vec<u8>,
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
    let params_base = cursor1.read_u32::<LittleEndian>()?;
    let buttons_base = cursor1.read_u32::<LittleEndian>()?;
    let bitmaps_base = cursor1.read_u32::<LittleEndian>()?;

    // println!("params = {}", params as usize + 4);

    let mut cursor2 = Cursor::new(&skin_data[params_base as usize + 4..]);

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

    let mut btns = vec![];
    let mut cursor_buttons = Cursor::new(&skin_data[buttons_base as usize..]);
    loop {
        let x = cursor_buttons.read_u32::<LittleEndian>()?;
        if x == 0 {
            break;
        }
        let btn_type = x;
        let btn_left = cursor_buttons.read_u16::<LittleEndian>()?;
        let btn_top = cursor_buttons.read_u16::<LittleEndian>()?;
        let btn_width = cursor_buttons.read_u16::<LittleEndian>()?;
        let btn_height = cursor_buttons.read_u16::<LittleEndian>()?;
        btns.push(SkinButton{btntype: btn_type, left: btn_left, top: btn_top, width: btn_width, height: btn_height});
    }

    let mut bmps = vec![];
    let mut cursor_bitmaps = Cursor::new(&skin_data[bitmaps_base as usize..]);
    loop {
        let word1 = cursor_bitmaps.read_u16::<LittleEndian>()?;
        let word2 = cursor_bitmaps.read_u16::<LittleEndian>()?;
        if word1 == 0 && word2 == 0 {
            break;
        }
        let bmp_kind = word1;
        let bmp_type = word2;
        let posbm = cursor_bitmaps.read_u32::<LittleEndian>()?;
        let mut cursor_bmp = Cursor::new(&skin_data[posbm as usize..]);
        let bmp_width = cursor_bmp.read_u32::<LittleEndian>()?;
        let bmp_height = cursor_bmp.read_u32::<LittleEndian>()?;
        // let bmp_size = bmp_width*bmp_height*3;
        
        let mut bmp_data = vec![];
        for _ in 0..bmp_height as usize * bmp_width as usize {
            let bb = cursor_bmp.read_u8()?;
            let gg = cursor_bmp.read_u8()?;
            let rr = cursor_bmp.read_u8()?;
            bmp_data.extend([rr, gg, bb, 255]);
        }
        bmps.push(SkinBitmap { kind: bmp_kind, bmptype: bmp_type, width: bmp_width, height: bmp_height, data: bmp_data });
    }

    // TODO parse further

    Ok(Skin {
        version: version,
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
        buttons: btns,
        bitmaps: bmps,
    })
}


fn dup_image_horiz(data: &[u8], width: usize, height: usize, target_width: usize) -> Vec<u8> {
    if data.len() != width * height * 4 {
        panic!("Input data size does not match width and height.");
    }
    if width == 0 || height == 0 || target_width == 0 {
        panic!("Width, height, and target_width must be greater than zero.");
    }

    let mut result = Vec::with_capacity(target_width * height * 4);

    for row in 0..height {
        for col in 0..target_width {
            let source_col = col % width; // Calculate the column in the original image

            // Calculate the index in the original image
            let source_index = (row * width + source_col) * 4;

            // Copy the RGBA values from the original image to the new image
            result.push(data[source_index]);     // R
            result.push(data[source_index + 1]); // G
            result.push(data[source_index + 2]); // B
            result.push(data[source_index + 3]); // A
        }
    }

    result
}


#[macroquad::main("KolibriOS skin viewer")]
async fn main() {
    // request_new_screen_size(300.0, 100.0);
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

    // println!("skin_obj = {:#X?} ", skin_obj);
    //println!("skin_obj = {:X?} ", skin_obj);

    println!("len(bmps) = {}", skin_obj.bitmaps.len());

    println!("{} {}", screen_width(), screen_height());

    

    let wx = 50.;
    let wy = 50.;
    let ww = 300.;
    let wh = 280.;

    // let texture = Texture2D::from_rgba8(skin_obj.bitmaps[2].width as u16, skin_obj.bitmaps[2].height as u16, &skin_obj.bitmaps[2].data);

    let mut active_texture_buttons: Option<Texture2D> = None;
    let mut active_texture_panel: Option<Texture2D> = None;
    let mut active_bmp_panel: Option<&SkinBitmap> = None;

    let mut inactive_texture_buttons: Option<Texture2D> = None;
    let mut inactive_texture_panel: Option<Texture2D> = None;
    let mut inactive_bmp_panel: Option<&SkinBitmap> = None;

    for bmp in &skin_obj.bitmaps {
        match bmp.kind {
            2 => {
                let texture = Some(Texture2D::from_rgba8(bmp.width as u16, bmp.height as u16, &bmp.data));
                if bmp.bmptype == 1 {
                    active_texture_buttons = texture;
                } else {
                    inactive_texture_buttons = texture;
                }
            }
            3 => {
                if bmp.bmptype == 1 {
                    active_bmp_panel = Some(bmp);
                } else {
                    inactive_bmp_panel = Some(bmp);
                }
            }
            _ => {

            }
        }
    }

    let panel_width = active_bmp_panel.unwrap().width as usize*(ww as usize - active_texture_buttons.as_ref().unwrap().width() as usize);
    let rep = dup_image_horiz(&active_bmp_panel.unwrap().data, active_bmp_panel.unwrap().width as usize, active_bmp_panel.unwrap().height as usize, panel_width);
    let active_texture_panel = Some(Texture2D::from_rgba8(panel_width as u16, active_bmp_panel.unwrap().height as u16, &rep));
    let rep = dup_image_horiz(&inactive_bmp_panel.unwrap().data, inactive_bmp_panel.unwrap().width as usize, inactive_bmp_panel.unwrap().height as usize, panel_width);
    let inactive_texture_panel = Some(Texture2D::from_rgba8(panel_width as u16, inactive_bmp_panel.unwrap().height as u16, &rep));


    loop {
        clear_background(WHITE);
        
        draw_rectangle(wx - 1., wy - 1., ww + 2., wh + 2., Color::from_hex(skin_obj.active.outer));
        draw_rectangle(wx, wy,  ww, wh, Color::from_hex(skin_obj.active.frame));

        draw_texture(&active_texture_buttons.as_ref().unwrap(), wx + ww - active_texture_buttons.as_ref().unwrap().width() + 1., wy - 1., WHITE);

        draw_texture(&active_texture_panel.as_ref().unwrap(), wx + 1., wy - 1., WHITE);

        next_frame().await;

        std:: thread ::sleep(Duration::from_millis(10));
    }


    // loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

    //     draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

    //     next_frame().await
    // }
}
