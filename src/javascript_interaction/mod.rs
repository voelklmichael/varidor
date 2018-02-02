mod strings;
pub use self::strings::*;
use std::os::raw::{c_char, c_double};
use std::ffi::CString;

use super::{BOARDSIZE, DATA};
use super::board_square::*;

const FIELD_WIDTH: f64 = 50. * 5. / BOARDSIZE as f64;
const WALL_WIDTH: f64 = 10. * 5. / BOARDSIZE as f64;
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
    fn draw_line_stroke(
        begin_x: c_double,
        begin_y: c_double,
        end_x: c_double,
        end_y: c_double,
        line_width: c_double,
        red: c_double,
        green: c_double,
        blue: c_double,
        opacity: c_double,
    );
//fn alerting(x: c_double, y: c_double);
}
//#[no_mangle]
//pub extern "C" fn reset(width: c_double, height: c_double) {}

#[no_mangle]
pub fn get_walls_black() -> c_double {
    DATA.lock()
        .unwrap()
        .board
        .get_player_data(TwoPlayerIndices::Black)
        .get_wall_count() as c_double
}
#[no_mangle]
pub fn get_walls_white() -> c_double {
    DATA.lock()
        .unwrap()
        .board
        .get_player_data(TwoPlayerIndices::White)
        .get_wall_count() as c_double
}
#[no_mangle]
pub fn get_current_player_string() -> *mut c_char {
    let s = DATA.lock().unwrap().get_current_player().to_string();
    let s = CString::new(s.to_string()).unwrap();
    s.into_raw()
}
#[no_mangle]
pub fn get_current_player_color_string() -> *mut c_char {
    let s = DATA.lock()
        .unwrap()
        .get_current_player()
        .get_color_as_string();
    let s = CString::new(s.to_string()).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn on_click(pos_x: c_double, pos_y: c_double) {
    if pos_x > WALL_WIDTH && pos_y > WALL_WIDTH {
        let x = pos_x - WALL_WIDTH;
        let y = pos_y - WALL_WIDTH;
        let column = (x / DISTANCE) as usize;
        let row = (y / DISTANCE) as usize;
        if x - (x / DISTANCE).floor() * DISTANCE < FIELD_WIDTH
            && y - (y / DISTANCE).floor() * DISTANCE < FIELD_WIDTH
        {
            let mut data = DATA.lock().unwrap();
            data.move_player_by_field(FieldIndexSquare {
                column: column,
                row: row,
            }).map(|error| data.append_logbook(error.to_string().to_string()));
            data.wall_index_selected = None;
        } else if x + WALL_WIDTH < BOARD_SIZE && y + WALL_WIDTH < BOARD_SIZE {
            let dir_is_left_or_right = if y - (y / DISTANCE).floor() * DISTANCE < FIELD_WIDTH {
                true
            } else {
                false
            };
            let mut data = DATA.lock().unwrap();
            let selected_before = data.wall_index_selected;
            match selected_before {
                None => {
                    data.wall_index_selected = Some((column, row, dir_is_left_or_right));
                }
                Some((column_before, row_before, dir_is_left_or_right_before)) => {
                    use std;
                    data.wall_index_selected = None;
                    let min_column = std::cmp::min(column, column_before);
                    let min_row = std::cmp::min(row, row_before);
                    let max_column = std::cmp::max(column, column_before);
                    let max_row = std::cmp::max(row, row_before);
                    let current_player = data.get_current_player();
                    let error = if dir_is_left_or_right != dir_is_left_or_right_before {
                        Some(WallPlacmentError::NotConnected)
                    } else if max_column == min_column && min_row + 1 == max_row {
                        data.board.place_wall(
                            current_player,
                            FieldIndexSquare {
                                column: min_column,
                                row: min_row,
                            },
                            DirectionsSquare::Right,
                            WallDirections::Left,
                        )
                    } else if max_column == min_column + 1 && min_row == max_row {
                        data.board.place_wall(
                            current_player,
                            FieldIndexSquare {
                                column: min_column,
                                row: min_row,
                            },
                            DirectionsSquare::Up,
                            WallDirections::Right,
                        )
                    } else {
                        Some(WallPlacmentError::NotConnected)
                    };
                    match error {
                        None => data.next_player(),
                        Some(err) => {
                            data.append_logbook(err.to_string().to_string());
                        }
                    }
                }
            };
        }
    }
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

    let data = DATA.lock().unwrap();
    // draw fields
    for column_index in 0..BOARDSIZE {
        for row_index in 0..BOARDSIZE {
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
    // draw walls horizontally
    let wall_color = (100, 100, 100);
    for column_index in 0..BOARDSIZE as usize - 1 {
        for row_index in 0..BOARDSIZE as usize - 0 {
            if data.board.wall_lookup_unsafe(
                FieldIndexSquare {
                    column: column_index,
                    row: row_index,
                },
                true,
            ) == WallPlaced::IsWall
            {
                draw_rectangle(
                    column_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                    row_index as f64 * DISTANCE + WALL_WIDTH,
                    WALL_WIDTH,
                    FIELD_WIDTH,
                    wall_color.0 as f64,
                    wall_color.1 as f64,
                    wall_color.2 as f64,
                );
            }
        }
    }
    // draw walls vertically
    for column_index in 0..BOARDSIZE as usize - 0 {
        for row_index in 0..BOARDSIZE as usize - 1 {
            if data.board.wall_lookup_unsafe(
                FieldIndexSquare {
                    column: column_index,
                    row: row_index,
                },
                false,
            ) == WallPlaced::IsWall
            {
                draw_rectangle(
                    column_index as f64 * DISTANCE + WALL_WIDTH,
                    row_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                    FIELD_WIDTH,
                    WALL_WIDTH,
                    wall_color.0 as f64,
                    wall_color.1 as f64,
                    wall_color.2 as f64,
                );
            }
        }
    }
    // draw wall crossings
    for column_index in 0..BOARDSIZE as usize - 1 {
        for row_index in 0..BOARDSIZE as usize - 1 {
            if data.board.croosing_lookup_unsafe(FieldIndexSquare {
                column: column_index,
                row: row_index,
            }) == WallCrossing::IsWallCrossing
            {
                draw_rectangle(
                    column_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                    row_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                    WALL_WIDTH,
                    WALL_WIDTH,
                    wall_color.0 as f64,
                    wall_color.1 as f64,
                    wall_color.2 as f64,
                );
            }
        }
    }

    // draw selected wall
    let wall_selected_color = (218, 165, 32);
    let selected_before = data.wall_index_selected;
    if selected_before.is_some() {
        let (column_index, row_index, dir_is_left_or_right) = selected_before.unwrap();
        if dir_is_left_or_right {
            draw_rectangle(
                column_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                row_index as f64 * DISTANCE + WALL_WIDTH,
                WALL_WIDTH,
                FIELD_WIDTH,
                wall_selected_color.0 as f64,
                wall_selected_color.1 as f64,
                wall_selected_color.2 as f64,
            );
        } else {
            draw_rectangle(
                column_index as f64 * DISTANCE + WALL_WIDTH,
                row_index as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH,
                FIELD_WIDTH,
                WALL_WIDTH,
                wall_selected_color.0 as f64,
                wall_selected_color.1 as f64,
                wall_selected_color.2 as f64,
            );
        }
    }
    // add shortest path
    let add_shortest_path = |player, red, green, blue, line_thickness, offset| {
        let shortest_paths = data.board.get_player_data(player).get_shortest_paths();
        for path in shortest_paths {
            let mut previous_field = data.board.get_current_field(player);
            for &(next_field, _) in path {
                draw_line_stroke(
                    previous_field.column as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH / 2.
                        + offset,
                    previous_field.row as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH / 2. + offset,
                    next_field.column as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH / 2. + offset,
                    next_field.row as f64 * DISTANCE + WALL_WIDTH + FIELD_WIDTH / 2. + offset,
                    line_thickness,
                    red as f64,
                    green as f64,
                    blue as f64,
                    0.8,
                );
                previous_field = next_field;
            }
        }
    };
    let player = TwoPlayerIndices::White;
    let (red, green, blue) = (255, 255, 255);
    let line_thickness = 5.;
    let offset = -5.;
    add_shortest_path(player, red, green, blue, line_thickness, offset);
    let player = TwoPlayerIndices::Black;
    let (red, green, blue) = (0, 0, 0);
    let line_thickness = 2.;
    let offset = 0.;
    add_shortest_path(player, red, green, blue, line_thickness, offset);
    // draw player black
    let pos_black = data.board.get_current_field(TwoPlayerIndices::Black);
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
    let pos_white = data.board.get_current_field(TwoPlayerIndices::White);
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
