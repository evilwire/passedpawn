use colored::*;
use std::collections::HashMap;
use core::fmt;
use core::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
enum Color {
    White,
    Black,
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

#[allow(dead_code)]
#[derive(Debug)]
enum PieceName {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

impl Display for PieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(dead_code)]
#[derive(Hash, PartialEq, Eq, Debug, EnumIter)]
enum Rank {
    A, B, C, D, E, F, G, H,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Position {
    rank: Rank,
    file: i8,
}

impl ToString for Position {
    fn to_string(&self) -> String {
        return format!("{}{}", self.rank, self.file)
    }
}

impl Position {
    fn new(rank: Rank, file: i8) -> Position {
        Position { rank, file, }
    }
}

struct Piece {
    color: Color,
    name: PieceName,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self.color {
           Color::White => {
               match self.name {
                   PieceName::King => String::from("♔"),
                   PieceName::Pawn => String::from("♙"),
                   PieceName::Knight => String::from("♘"),
                   PieceName::Bishop => String::from("♗"),
                   PieceName::Rook => String::from("♖"),
                   PieceName::Queen => String::from("♕"),
               }
           }
           Color::Black => {
               match self.name {
                   PieceName::King => String::from("♚"),
                   PieceName::Pawn => String::from("♟︎"),
                   PieceName::Knight => String::from("♞"),
                   PieceName::Bishop => String::from("♝"),
                   PieceName::Rook => String::from("♜"),
                   PieceName::Queen => String::from("♛"),
               }
           },
        }
    }
}

struct Board {
    layout: HashMap<Position, Piece>,
}

impl Board {
    fn new() -> Board {
        let layout= HashMap::from([
            (Position::new(Rank::A, 1), Piece { color: Color::White, name: PieceName::Rook }),
            (Position::new(Rank::B, 1), Piece { color: Color::White, name: PieceName::Knight }),
            (Position::new(Rank::C, 1), Piece { color: Color::White, name: PieceName::Bishop }),
            (Position::new(Rank::D, 1), Piece { color: Color::White, name: PieceName::Queen }),
            (Position::new(Rank::E, 1), Piece { color: Color::White, name: PieceName::King }),
            (Position::new(Rank::F, 1), Piece { color: Color::White, name: PieceName::Bishop }),
            (Position::new(Rank::G, 1), Piece { color: Color::White, name: PieceName::Knight }),
            (Position::new(Rank::H, 1), Piece { color: Color::White, name: PieceName::Rook }),
            (Position::new(Rank::A, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::B, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::C, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::D, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::E, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::F, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::G, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::H, 2), Piece { color: Color::White, name: PieceName::Pawn }),
            (Position::new(Rank::A, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::B, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::C, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::D, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::E, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::F, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::G, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::H, 7), Piece { color: Color::Black, name: PieceName::Pawn }),
            (Position::new(Rank::A, 8), Piece { color: Color::Black, name: PieceName::Rook }),
            (Position::new(Rank::B, 8), Piece { color: Color::Black, name: PieceName::Knight }),
            (Position::new(Rank::C, 8), Piece { color: Color::Black, name: PieceName::Bishop }),
            (Position::new(Rank::D, 8), Piece { color: Color::Black, name: PieceName::Queen }),
            (Position::new(Rank::E, 8), Piece { color: Color::Black, name: PieceName::King }),
            (Position::new(Rank::F, 8), Piece { color: Color::Black, name: PieceName::Bishop }),
            (Position::new(Rank::G, 8), Piece { color: Color::Black, name: PieceName::Knight }),
            (Position::new(Rank::H, 8), Piece { color: Color::Black, name: PieceName::Rook }),
        ]);
        Board { layout }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        // for each file
        let mut board_layout = String::new();
        for n in (1..9).rev() {
            let rank_mark = format!("{}", n).dimmed().to_string();
            board_layout += &*rank_mark;
            for r in Rank::iter() {
                let position = Position::new(r, n);
                board_layout += &*match self.layout.get(&position) {
                    Some(x) => format!(" {}", x.to_string()),
                    None => " .".to_string()
                }
            }
            board_layout += "\n";
        }

        board_layout += &*"  a b c d e f g h".dimmed().to_string();
        return board_layout
    }
}

fn main() {
    let mut board = Board::new();
    println!("{}", board.to_string());

    board.layout.insert(Position::new(Rank::A, 4), Piece { color: Color::White, name: PieceName::Pawn});
    board.layout.remove(&Position::new(Rank::A, 2));

    println!("{}", board.to_string());
}
