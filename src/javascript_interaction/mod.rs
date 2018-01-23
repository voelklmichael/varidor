mod strings;
pub use self::strings::*;
use std::os::raw::{c_char, c_double};
use std::ffi::CString;

use super::{BOARDSIZE, DATA};
use super::game_logic::*;

const FIELD_WIDTH: f64 = 50.;
const WALL_WIDTH: f64 = 10.;
const DISTANCE: f64 = FIELD_WIDTH + WALL_WIDTH;
const BOARD_SIZE: f64 = DISTANCE * BOARDSIZE as f64 + WALL_WIDTH;
// These functions are provided by the runtime
extern "C" {
    fn clear_screen(width: c_double, height: c_double);
    fn draw_rectangle(
        top_left_x: c_double,
        top_left_y: c_double,
        width: c_double,
        height: c_double,
        red: c_double,
        green: c_double,
        blue: c_double,
    );
    fn draw_circle(
        center_x: c_double,
        center_y: c_double,
        radius: c_double,
        red: c_double,
        green: c_double,
        blue: c_double,
        opacity: c_double,
    );
    fn draw_path_5_steps(
        pos1_x: c_double,
        pos1_y: c_double,
        pos2_x: c_double,
        pos2_y: c_double,
        pos3_x: c_double,
        pos3_y: c_double,
        pos4_x: c_double,
        pos4_y: c_double,
        pos5_x: c_double,
        pos5_y: c_double,
        red: c_double,
        green: c_double,
        blue: c_double,
        opacity: c_double,
    );
}
//#[no_mangle]
//pub extern "C" fn reset(width: c_double, height: c_double) {}

#[no_mangle]
pub extern "C" fn on_click(pos_x: c_double, pos_y: c_double) {
    let x = pos_x - WALL_WIDTH;
    let y = pos_y - WALL_WIDTH;

    if x - (x / DISTANCE).floor() * DISTANCE < FIELD_WIDTH
        && y - (y / DISTANCE).floor() * DISTANCE < FIELD_WIDTH
    {
        let column = (x / DISTANCE) as usize;
        let row = (y / DISTANCE) as usize;
        let mut data = DATA.lock().unwrap();
        data.move_player_by_field(FieldIndices {
            column: column,
            row: row,
        }).map(|error| data.append_logbook(error.to_string().to_string()));
    };
}

#[no_mangle]
pub fn get_logbook() -> *mut c_char {
    let data = DATA.lock().unwrap().get_logbook().clone();
    let s = data.join("\n");
    let s = CString::new(s).unwrap();
    s.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    // reset screen
    clear_screen(BOARD_SIZE, BOARD_SIZE);
    // draw fields
    for column_index in 0..BOARDSIZE * 1 {
        for row_index in 0..BOARDSIZE * 1 {
            let (red, green, blue) = if (column_index + row_index) & 1 == 0 {
                (222, 184, 135)
            } else {
                (139, 69, 19)
            };
            draw_rectangle(
                column_index as f64 * DISTANCE + WALL_WIDTH,
                row_index as f64 * DISTANCE + WALL_WIDTH,
                FIELD_WIDTH,
                FIELD_WIDTH,
                red as f64,
                green as f64,
                blue as f64,
            );
        }
    }
    // draw player black
    let data = DATA.lock().unwrap();
    let pos_black = data.get_player_field(PlayerIndices::Black);
    draw_circle(
        WALL_WIDTH + DISTANCE * pos_black.column as f64 + FIELD_WIDTH / 2.,
        WALL_WIDTH + DISTANCE * pos_black.row as f64 + FIELD_WIDTH / 2.,
        FIELD_WIDTH / 2. * 0.8,
        0.,
        0.,
        0.,
        0.8,
    );
    // draw player white
    let pos_white = data.get_player_field(PlayerIndices::White);
    let white_x = pos_white.column as f64;
    let white_y = pos_white.row as f64;
    let mut pos_x = [0f64; 5];
    let mut pos_y = [0f64; 5];
    let scaling = FIELD_WIDTH / 2.;
    let alpha = (360.0 / 5.0f64).to_radians() * 2.; // step size times 2 for pentragram instead of p
    for i in 0..5 {
        pos_x[i] = WALL_WIDTH + DISTANCE * white_x + FIELD_WIDTH / 2.
            + scaling * (alpha * (i as f64)).sin();
        pos_y[i] = WALL_WIDTH + DISTANCE * white_y + FIELD_WIDTH / 2.
            - scaling * (alpha * (i as f64)).cos();
    }
    draw_path_5_steps(
        pos_x[0],
        pos_y[0],
        pos_x[1],
        pos_y[1],
        pos_x[2],
        pos_y[2],
        pos_x[3],
        pos_y[3],
        pos_x[4],
        pos_y[4],
        255.,
        255.,
        255.,
        0.8,
    );
}

#[no_mangle]
pub extern "C" fn update(_: c_double) {}
