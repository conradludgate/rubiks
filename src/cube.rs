pub struct Cube {
    edges: [(Edge, EdgeOrientation); 12],
    corners: [(Corner, CornerOrientation); 8],
}

#[derive(Debug)]
pub enum EdgeOrientation {
    Zero,
    One,
}

#[derive(Debug)]
pub enum CornerOrientation {
    Zero,
    One,
    Two,
}

#[derive(Debug)]
pub struct Edge(u8);

#[derive(Debug)]
pub struct Corner(u8);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Side {
    Red,
    White,
    Blue,
    Orange,
    Yellow,
    Green,
}

impl Side {
    pub fn edge(a: Self, b: Self) -> Option<Edge> {
        match (a, b) {
            (Side::Red, Side::White) | (Side::White, Side::Red) => Some(Edge(0)),
            (Side::Red, Side::Blue) | (Side::Blue, Side::Red) => Some(Edge(1)),
            (Side::Red, Side::Yellow) | (Side::Yellow, Side::Red) => Some(Edge(2)),
            (Side::Red, Side::Green) | (Side::Green, Side::Red) => Some(Edge(3)),
            (Side::White, Side::Blue) | (Side::Blue, Side::White) => Some(Edge(4)),
            (Side::White, Side::Orange) | (Side::Orange, Side::White) => Some(Edge(5)),
            (Side::White, Side::Green) | (Side::Green, Side::White) => Some(Edge(6)),
            (Side::Blue, Side::Orange) | (Side::Orange, Side::Blue) => Some(Edge(7)),
            (Side::Blue, Side::Yellow) | (Side::Yellow, Side::Blue) => Some(Edge(8)),
            (Side::Orange, Side::Yellow) | (Side::Yellow, Side::Orange) => Some(Edge(9)),
            (Side::Orange, Side::Green) | (Side::Green, Side::Orange) => Some(Edge(10)),
            (Side::Yellow, Side::Green) | (Side::Green, Side::Yellow) => Some(Edge(11)),
            _ => None,
        }
    }
    pub fn corner(a: Self, b: Self, c: Self) -> Option<Corner> {
        match (a, b, c) {
            (Side::Red, Side::White, Side::Blue)
            | (Side::Red, Side::Blue, Side::White)
            | (Side::White, Side::Red, Side::Blue)
            | (Side::White, Side::Blue, Side::Red)
            | (Side::Blue, Side::Red, Side::White)
            | (Side::Blue, Side::White, Side::Red) => Some(Corner(0)),

            (Side::Red, Side::White, Side::Green)
            | (Side::Red, Side::Green, Side::White)
            | (Side::White, Side::Red, Side::Green)
            | (Side::White, Side::Green, Side::Red)
            | (Side::Green, Side::White, Side::Red)
            | (Side::Green, Side::Red, Side::White) => Some(Corner(1)),

            (Side::Red, Side::Blue, Side::Yellow)
            | (Side::Red, Side::Yellow, Side::Blue)
            | (Side::Blue, Side::Red, Side::Yellow)
            | (Side::Blue, Side::Yellow, Side::Red)
            | (Side::Yellow, Side::Red, Side::Blue)
            | (Side::Yellow, Side::Blue, Side::Red) => Some(Corner(2)),

            (Side::Red, Side::Yellow, Side::Green)
            | (Side::Red, Side::Green, Side::Yellow)
            | (Side::Yellow, Side::Red, Side::Green)
            | (Side::Yellow, Side::Green, Side::Red)
            | (Side::Green, Side::Red, Side::Yellow)
            | (Side::Green, Side::Yellow, Side::Red) => Some(Corner(3)),

            (Side::White, Side::Blue, Side::Orange)
            | (Side::White, Side::Orange, Side::Blue)
            | (Side::Blue, Side::White, Side::Orange)
            | (Side::Blue, Side::Orange, Side::White)
            | (Side::Orange, Side::White, Side::Blue)
            | (Side::Orange, Side::Blue, Side::White) => Some(Corner(4)),

            (Side::White, Side::Orange, Side::Green)
            | (Side::White, Side::Green, Side::Orange)
            | (Side::Orange, Side::White, Side::Green)
            | (Side::Orange, Side::Green, Side::White)
            | (Side::Green, Side::White, Side::Orange)
            | (Side::Green, Side::Orange, Side::White) => Some(Corner(5)),

            (Side::Blue, Side::Orange, Side::Yellow)
            | (Side::Blue, Side::Yellow, Side::Orange)
            | (Side::Orange, Side::Blue, Side::Yellow)
            | (Side::Orange, Side::Yellow, Side::Blue)
            | (Side::Yellow, Side::Blue, Side::Orange)
            | (Side::Yellow, Side::Orange, Side::Blue) => Some(Corner(6)),

            (Side::Orange, Side::Yellow, Side::Green)
            | (Side::Orange, Side::Green, Side::Yellow)
            | (Side::Yellow, Side::Orange, Side::Green)
            | (Side::Yellow, Side::Green, Side::Orange)
            | (Side::Green, Side::Orange, Side::Yellow)
            | (Side::Green, Side::Yellow, Side::Orange) => Some(Corner(7)),

            _ => None,
        }
    }
}
