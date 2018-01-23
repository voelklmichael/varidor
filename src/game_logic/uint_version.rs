use super::BoardTrait;
const BOARDSIZE: usize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum Directions {
    Up,
    Left,
    Right,
    Down,
}
const DIRECTIONS_COUNT: usize = 4;
const DIRECTIONS_ARRAY: [Directions; DIRECTIONS_COUNT] = [
    Directions::Up,
    Directions::Left,
    Directions::Right,
    Directions::Down,
];

#[derive(Clone, Copy)]
pub enum PlayerIndices {
    Black,
    White,
}
impl PlayerIndices {
    pub fn to_string(self) -> &'static str {
        match self {
            PlayerIndices::Black => "BLACK",
            PlayerIndices::White => "WHITE",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FieldIndices {
    pub column: usize,
    pub row: usize,
}
use std::fmt;
impl fmt::Debug for FieldIndices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.column, self.row)
    }
}
const COLUMN_MIN: usize = 0;
const COLUMN_MAX: usize = BOARDSIZE - 1;
const ROW_MIN: usize = 0;
const ROW_MAX: usize = BOARDSIZE - 1;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WallIsPlaced {
    IsEmpty,
    IsWall,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntersectionIsPlaced {
    IsEmpty,
    IsPlaced,
}

pub struct Board {
    current_player: PlayerIndices,
    white_number_of_walls: u8,
    black_number_of_walls: u8,
    white_position: FieldIndices,
    black_position: FieldIndices,
    white_shortest_paths: Vec<Vec<FieldIndices>>,
    black_shortest_paths: Vec<Vec<FieldIndices>>,
    walls_is_placed: [WallIsPlaced; 2 * BOARDSIZE * (BOARDSIZE - 1)],
    intersections_is_placed: [IntersectionIsPlaced; (BOARDSIZE - 1) * (BOARDSIZE - 1)],
    logbook: Vec<String>,
}

impl Board {
    fn wall_lookup(
        &self,
        lower_left_column: usize,
        lower_left_row: usize,
        dir_is_left_or_right: bool,
    ) -> WallIsPlaced {
        if dir_is_left_or_right {
            self.walls_is_placed[lower_left_column + lower_left_row * (BOARDSIZE - 1)]
        } else {
            self.walls_is_placed
                [lower_left_row + lower_left_column * (BOARDSIZE - 1) + BOARDSIZE * (BOARDSIZE - 1)]
        }
    }
    fn wall_lookup_mut(
        &mut self,
        lower_left_column: usize,
        lower_left_row: usize,
        dir_is_left_or_right: bool,
    ) -> &mut WallIsPlaced {
        if dir_is_left_or_right {
            &mut self.walls_is_placed[lower_left_column + lower_left_row * (BOARDSIZE - 1)]
        } else {
            &mut self.walls_is_placed
                [lower_left_row + lower_left_column * (BOARDSIZE - 1) + BOARDSIZE * (BOARDSIZE - 1)]
        }
    }
    fn intersectionwall_lookup(
        &self,
        lower_left_column: usize,
        lower_left_row: usize,
    ) -> IntersectionIsPlaced {
        self.intersections_is_placed[lower_left_column + lower_left_row * (BOARDSIZE - 1)]
    }
    fn intersectionwall_lookup_mut(
        &mut self,
        lower_left_column: usize,
        lower_left_row: usize,
    ) -> &mut IntersectionIsPlaced {
        &mut self.intersections_is_placed[lower_left_column + lower_left_row * (BOARDSIZE - 1)]
    }
    fn update_shortest_paths(&mut self) {
        self.black_shortest_paths = self.get_shortest_pathes(PlayerIndices::Black);
        self.white_shortest_paths = self.get_shortest_pathes(PlayerIndices::White);
    }
    fn get_remaining_walls_count(&mut self, player: PlayerIndices) -> &mut u8 {
        match player {
            PlayerIndices::Black => &mut self.black_number_of_walls,
            PlayerIndices::White => &mut self.white_number_of_walls,
        }
    }
}

pub enum MoveErrorType {
    BoardBoundary,
    Wall,
    FieldsNotAdjacent,
}
impl MoveErrorType {
    pub fn to_string(self) -> &'static str {
        match self {
            MoveErrorType::BoardBoundary => "outer boundary of board reached",
            MoveErrorType::Wall => "way blocksed by wall",
            MoveErrorType::FieldsNotAdjacent => "fields not adjacent",
        }
    }
}

#[derive(Debug)]
pub enum PlaceWallErrorType {
    BoardBoundary,
    WallAlreadyThere,
    IntesectionAlreadyThere,
    PlayerBlocked,
    NoMoreWalls,
}

pub enum WallPlacementDirections {
    Left,
    Right,
}

impl BoardTrait for Board {
    type FieldIndices = FieldIndices;
    type Directions = Directions;
    type PlayerIndices = PlayerIndices;
    type MoveErrorType = MoveErrorType;
    type PlaceWallErrorType = PlaceWallErrorType;
    type WallPlacementDirections = WallPlacementDirections;
    // create a new game
    fn get_logbook(&self) -> &Vec<String> {
        &self.logbook
    }
    fn next_player(&mut self) {
        match self.current_player {
            PlayerIndices::Black => self.current_player = PlayerIndices::White,
            PlayerIndices::White => self.current_player = PlayerIndices::Black,
        }
    }
    fn append_logbook(&mut self, new_line: String) {
        self.logbook.push(new_line);
    }
    fn new() -> Self {
        let number_of_walls = 5;
        let start_column = (BOARDSIZE - 1) / 2;
        let mut board = Board {
            current_player: PlayerIndices::White,
            white_number_of_walls: number_of_walls,
            black_number_of_walls: number_of_walls,
            white_shortest_paths: Vec::with_capacity(1),
            black_shortest_paths: Vec::with_capacity(1),
            white_position: FieldIndices {
                row: ROW_MAX,
                column: start_column,
            },
            black_position: FieldIndices {
                row: ROW_MIN,
                column: start_column,
            },
            walls_is_placed: [WallIsPlaced::IsEmpty; 2 * BOARDSIZE * (BOARDSIZE - 1)],
            intersections_is_placed: [IntersectionIsPlaced::IsEmpty;
                (BOARDSIZE - 1) * (BOARDSIZE - 1)],
            logbook: vec!["Game started".to_string()],
        };
        board.update_shortest_paths();
        board
    }
    fn get_current_player(&self) -> Self::PlayerIndices {
        self.current_player
    }

    // check if a field is on the baseline of the board - top or bottom row
    fn is_final_field(field: Self::FieldIndices, player: Self::PlayerIndices) -> bool {
        use self::PlayerIndices::*;
        match player {
            White => field.row == ROW_MIN,
            Black => field.row == ROW_MAX,
        }
    }
    // get current position of a player - no checks done here!
    fn get_player_field(&self, player: Self::PlayerIndices) -> Self::FieldIndices {
        use self::PlayerIndices::*;
        match player {
            White => self.white_position,
            Black => self.black_position,
        }
    }
    // get current position of a player - no checks done here!
    fn set_player_field(&mut self, player: Self::PlayerIndices, new_position: Self::FieldIndices) {
        use self::PlayerIndices::*;
        match player {
            White => self.white_position = new_position,
            Black => self.black_position = new_position,
        }
    }
    // get fields adjacent to a given field
    fn get_adjacent_field(
        field: Self::FieldIndices,
        direction: Self::Directions,
    ) -> Option<Self::FieldIndices> {
        use self::Directions::*;
        match direction {
            Right => if field.column != COLUMN_MAX {
                Some(FieldIndices {
                    row: field.row,
                    column: field.column + 1,
                })
            } else {
                None
            },
            Left => if field.column != COLUMN_MIN {
                Some(FieldIndices {
                    row: field.row,
                    column: field.column - 1,
                })
            } else {
                None
            },
            Down => if field.row != ROW_MIN {
                Some(FieldIndices {
                    row: field.row - 1,
                    column: field.column,
                })
            } else {
                None
            },
            Up => if field.row != ROW_MAX {
                Some(FieldIndices {
                    row: field.row + 1,
                    column: field.column,
                })
            } else {
                None
            },
        }
    }
    /* not necessary
	// get all fields adjacent to a given one
    fn get_adjacent_fields(field: Self::FieldIndices) -> Vec<Self::FieldIndices> {
        let mut adjacent_fields = Vec::with_capacity(DIRECTIONS_COUNT);
        for direction in &DIRECTIONS_ARRAY {
            match Self::get_adjacent_field(field, *direction) {
                Some(x) => adjacent_fields.push(x),
                None => {}
            }
        }
        adjacent_fields
    }
    */
    // get all fields adjacent to a given one, but only ones which are connected
    fn get_fields_connected(&self, field: Self::FieldIndices) -> Vec<Self::FieldIndices> {
        let mut adjacent_fields = Vec::with_capacity(DIRECTIONS_COUNT);
        for direction in &DIRECTIONS_ARRAY {
            match Self::fields_connected(self, field, *direction) {
                Ok(x) => adjacent_fields.push(x),
                Err(_) => {}
            }
        }
        adjacent_fields
    }
    // ckeck if two fiels are connected
    fn fields_connected(
        &self,
        first_field: Self::FieldIndices,
        direction: Self::Directions,
    ) -> Result<Self::FieldIndices, Self::MoveErrorType> {
        // get current field
        use self::Directions::*;
        if let Some(next_field) = Self::get_adjacent_field(first_field, direction) {
            let wall_is_placed = match direction {
                Right => {
                    let lower_left_row = first_field.row; // number between include 0 and BOARDSIZE-1
                    let lower_left_column = first_field.column; // number between include 0 and BOARDSIZE-1
                    self.wall_lookup(lower_left_column, lower_left_row, true)
                }
                Left => {
                    let lower_left_row = next_field.row; // number between include 0 and BOARDSIZE-1
                    let lower_left_column = next_field.column; // number between include 0 and BOARDSIZE-1
                    self.wall_lookup(lower_left_column, lower_left_row, true)
                }
                Up => {
                    let lower_left_row = first_field.row; // number between include 0 and BOARDSIZE-1
                    let lower_left_column = first_field.column; // number between include 0 and BOARDSIZE-1
                    self.wall_lookup(lower_left_column, lower_left_row, false)
                }
                Down => {
                    let lower_left_row = next_field.row; // number between include 0 and BOARDSIZE-1
                    let lower_left_column = next_field.column; // number between include 0 and BOARDSIZE-1
                    self.wall_lookup(lower_left_column, lower_left_row, false)
                }
            };
            match wall_is_placed {
                WallIsPlaced::IsWall => Err(MoveErrorType::Wall),
                WallIsPlaced::IsEmpty => Ok(next_field),
            }
        } else {
            Err(MoveErrorType::BoardBoundary)
        }
    }

    fn get_shortest_pathes(&self, player: Self::PlayerIndices) -> Vec<Vec<Self::FieldIndices>> {
        let mut shortest_paths = Vec::<Vec<FieldIndices>>::with_capacity(100);
        let mut paths = Vec::<Vec<FieldIndices>>::with_capacity(100);
        let mut visited_fields = Vec::with_capacity(BOARDSIZE * (BOARDSIZE - 1));
        // initial setup
        {
            let current_field = self.get_player_field(player);
            paths.push(vec![current_field]);
            visited_fields.push(current_field);
        }

        let mut current_last_index = 0;

        let mut final_field_not_visited = true;
        while final_field_not_visited && !paths.is_empty() {
            let mut new_visited_fields = Vec::with_capacity(paths.len());
            //println!("{:?}", "---");
            //println!(" visited {:?}", visited_fields);
            let mut new_paths = Vec::<Vec<FieldIndices>>::with_capacity(paths.len() * 2);
            for path in &paths {
                //println!("   path: {:?}", path);
                let connected_fields = self.get_fields_connected(path[current_last_index]);
                //println!("      connected: {:?}", connected_fields);
                //print!("          new fields:");
                for field in connected_fields {
                    if Self::is_final_field(field, player) {
                        final_field_not_visited = false;
                        let mut new_path = path.clone();
                        new_path.push(field);
                        shortest_paths.push(new_path);
                    } else {
                        if final_field_not_visited {
                            if *&visited_fields
                                .iter()
                                .all(|visited_field| *visited_field != field)
                            {
                                //print!(" {:?}", field);
                                new_visited_fields.push(field);
                                let mut new_path = path.clone();
                                new_path.push(field);
                                new_paths.push(new_path);
                            }
                        }
                    }
                }
                //println!("");
            }
            current_last_index += 1;
            //println!(" length: {:?}", current_last_index);
            paths = new_paths;
            for new_visited_field in new_visited_fields {
                if *&visited_fields
                    .iter()
                    .all(|visited_field| *visited_field != new_visited_field)
                {
                    visited_fields.push(new_visited_field);
                }
            }
        }
        shortest_paths
    }

    fn place_wall(
        &mut self,
        player: Self::PlayerIndices,
        first_field: Self::FieldIndices,
        direction: Self::Directions,
        wall_direction: Self::WallPlacementDirections,
    ) -> Option<Self::PlaceWallErrorType> {
        if *self.get_remaining_walls_count(player) == 0 {
            return Some(PlaceWallErrorType::NoMoreWalls);
        }
        use self::Directions::*;
        let current_row = first_field.row;
        let current_column = first_field.column;
        // check if board boundary is reached by moving toward
        if (direction == Right && first_field.column == COLUMN_MAX)
            || (direction == Left && first_field.column == COLUMN_MIN)
            || (direction == Up && first_field.row == ROW_MAX)
            || (direction == Down && first_field.row == ROW_MIN)
        {
            return Some(PlaceWallErrorType::BoardBoundary);
        }

        // check if wall is already there
        let is_wall = match direction {
            Right => {
                let lower_left_column = current_column; // number between include 0 and BOARDSIZE-1
                let lower_left_row = current_row; // number between include 0 and BOARDSIZE-1
                self.wall_lookup(lower_left_column, lower_left_row, true)
            }
            Left => {
                let lower_left_column = current_column - 1; // number between include 0 and BOARDSIZE-1
                let lower_left_row = current_row; // number between include 0 and BOARDSIZE-1
                self.wall_lookup(lower_left_column, lower_left_row, true)
            }
            Up => {
                let lower_left_column = current_column; // number between include 0 and BOARDSIZE-1
                let lower_left_row = current_row; // number between include 0 and BOARDSIZE-1
                self.wall_lookup(lower_left_column, lower_left_row, false)
            }
            Down => {
                let lower_left_column = current_column; // number between include 0 and BOARDSIZE-1
                let lower_left_row = current_row - 1; // number between include 0 and BOARDSIZE-1
                self.wall_lookup(lower_left_column, lower_left_row, false)
            }
        };
        if is_wall == WallIsPlaced::IsWall {
            return Some(PlaceWallErrorType::WallAlreadyThere);
        }

        // check if second wall fields are inside boundary and if intersection is empty,
        // if everything is ok, place two walls
        match (direction, wall_direction) {
            (Up, WallPlacementDirections::Left) => {
                if current_column == COLUMN_MIN {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column - 1;
                    let lower_left_row = current_row;
                    let dir_is_left_or_right = false;
                    if self.wall_lookup(lower_left_column, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column + 1,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column + 1,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }
            (Up, WallPlacementDirections::Right) => {
                if current_column == COLUMN_MAX {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column;
                    let lower_left_row = current_row;
                    let dir_is_left_or_right = false;
                    if self.wall_lookup(lower_left_column + 1, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column + 1,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column + 1,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }

            (Down, WallPlacementDirections::Left) => {
                if current_column == COLUMN_MAX {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column;
                    let lower_left_row = current_row - 1;
                    let dir_is_left_or_right = false;
                    if self.wall_lookup(lower_left_column + 1, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column + 1,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column + 1,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }
            (Down, WallPlacementDirections::Right) => {
                if current_column == COLUMN_MIN {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column - 1;
                    let lower_left_row = current_row - 1;
                    let dir_is_left_or_right = false;
                    if self.wall_lookup(lower_left_column, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column + 1,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column + 1,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }

            (Left, WallPlacementDirections::Left) => {
                if current_row == ROW_MIN {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column - 1;
                    let lower_left_row = current_row - 1;
                    let dir_is_left_or_right = true;
                    if self.wall_lookup(lower_left_column, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row + 1,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row + 1,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }
            (Left, WallPlacementDirections::Right) => {
                if current_row == ROW_MAX {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column - 1;
                    let lower_left_row = current_row;
                    let dir_is_left_or_right = true;
                    if self.wall_lookup(lower_left_column, lower_left_row + 1, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row + 1,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row + 1,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }

            (Right, WallPlacementDirections::Left) => {
                if current_row == ROW_MAX {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column;
                    let lower_left_row = current_row;
                    let dir_is_left_or_right = true;
                    if self.wall_lookup(lower_left_column, lower_left_row + 1, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row + 1,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row + 1,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }
            (Right, WallPlacementDirections::Right) => {
                if current_row == ROW_MIN {
                    Some(PlaceWallErrorType::BoardBoundary)
                } else {
                    let lower_left_column = current_column;
                    let lower_left_row = current_row - 1;
                    let dir_is_left_or_right = true;
                    if self.wall_lookup(lower_left_column, lower_left_row, dir_is_left_or_right)
                        == WallIsPlaced::IsWall
                    {
                        Some(PlaceWallErrorType::WallAlreadyThere)
                    } else if self.intersectionwall_lookup(lower_left_column, lower_left_row)
                        == IntersectionIsPlaced::IsPlaced
                    {
                        Some(PlaceWallErrorType::IntesectionAlreadyThere)
                    } else {
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.wall_lookup_mut(
                            lower_left_column,
                            lower_left_row + 1,
                            dir_is_left_or_right,
                        ) = WallIsPlaced::IsWall;
                        *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                            IntersectionIsPlaced::IsPlaced;
                        // update player positions
                        self.update_shortest_paths();
                        if self.white_shortest_paths.is_empty()
                            || self.black_shortest_paths.is_empty()
                        {
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.wall_lookup_mut(
                                lower_left_column,
                                lower_left_row + 1,
                                dir_is_left_or_right,
                            ) = WallIsPlaced::IsEmpty;
                            *self.intersectionwall_lookup_mut(lower_left_column, lower_left_row) =
                                IntersectionIsPlaced::IsEmpty;
                            self.update_shortest_paths();
                            Some(PlaceWallErrorType::PlayerBlocked)
                        } else {
                            *self.get_remaining_walls_count(player) -= 1;
                            None
                        }
                    }
                }
            }
        }
    }
    fn move_player_by_field(
        &mut self,
        new_field: Self::FieldIndices,
    ) -> Option<Self::MoveErrorType> {
        let player = self.get_current_player();
        let current_field = self.get_player_field(player);
        let mut direction = None;
        for dir in &DIRECTIONS_ARRAY {
            let x = Self::get_adjacent_field(current_field, *dir)
                .map(|adjacent_field| adjacent_field == new_field);
            if x.is_some() && x.unwrap() {
                direction = Some(dir);
                break;
            }
        }
        if direction.is_none() {
            Some(MoveErrorType::FieldsNotAdjacent)
        } else {
            match self.fields_connected(current_field, *direction.unwrap()) {
                Ok(_) => {
                    self.set_player_field(player, new_field);
                    if Board::is_final_field(self.get_player_field(player), player) {
                        self.append_logbook(
                            "player ".to_string() + &player.to_string()
                                + &" has won the game!".to_string(),
                        )
                    };
                    self.next_player();
                    None
                }
                Err(x) => Some(x),
            }
        }
    }
}
