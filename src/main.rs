extern crate num_traits;
mod board_square;
use board_square::*;

fn main() {
    let mut board = SquareBoard3::new();
    board.place_wall(
        TwoPlayerIndices::White,
        FieldIndexSquare { column: 2, row: 1 },
        DirectionsSquare::Up,
        WallDirections::Left,
    );
    board.place_wall(
        TwoPlayerIndices::White,
        FieldIndexSquare { column: 2, row: 1 },
        DirectionsSquare::Up,
        WallDirections::Right,
    );
    {
        let shortest_paths = board
            .get_player_data(TwoPlayerIndices::Black)
            .get_shortest_paths();

        let FieldIndexSquare {
            column: cc,
            row: rr,
        } = board.get_current_field(TwoPlayerIndices::Black);
        for path in shortest_paths {
            println!("---");
            println!("   c:{:?}, r:{:?}", cc, rr);
            for &(FieldIndexSquare { column: c, row: r }, _) in path {
                println!("   c:{:?}, r:{:?}", c, r);
            }
        }
    }
    board.move_player(TwoPlayerIndices::Black, DirectionsSquare::Left);
    {
        let shortest_paths = board
            .get_player_data(TwoPlayerIndices::Black)
            .get_shortest_paths();

        let FieldIndexSquare {
            column: cc,
            row: rr,
        } = board.get_current_field(TwoPlayerIndices::Black);
        for path in shortest_paths {
            println!("---");
            println!("   c:{:?}, r:{:?}", cc, rr);
            for &(FieldIndexSquare { column: c, row: r }, _) in path {
                println!("   c:{:?}, r:{:?}", c, r);
            }
        }
    }
}
