//! Primitive axis types

/// Pos | Neg | Neutral
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    /// Positive
    Pos,
    /// Negative
    Neg,
    /// Neutral
    Neutral,
}

impl Sign {
    pub fn to_i8(&self) -> i32 {
        match self {
            Self::Pos => 1,
            Self::Neg => -1,
            Self::Neutral => 0,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Self::Pos => 1,
            Self::Neg => -1,
            Self::Neutral => 0,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Self::Pos => 1,
            Self::Neg => -1,
            Self::Neutral => 0,
        }
    }

    pub fn to_isize(&self) -> isize {
        match self {
            Self::Pos => 1,
            Self::Neg => -1,
            Self::Neutral => 0,
        }
    }

    pub fn from_i32(x: i32) -> Self {
        if x > 0 {
            Self::Pos
        } else if x < 0 {
            Self::Neg
        } else {
            Self::Neutral
        }
    }

    pub fn inv(&self) -> Self {
        match self {
            Self::Pos => Self::Neg,
            Self::Neg => Self::Pos,
            Self::Neutral => Self::Neutral,
        }
    }
}

/// One of the four directions: N, E, S, W
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
}

impl Dir4 {
    pub fn x_sign(&self) -> Sign {
        use Dir4::*;
        use Sign::*;

        match self {
            E => Pos,
            N | S => Neutral,
            W => Neg,
        }
    }

    pub fn y_sign(&self) -> Sign {
        use Dir4::*;
        use Sign::*;

        match self {
            N => Pos,
            E | W => Neutral,
            S => Neg,
        }
    }

    pub fn signs(&self) -> [Sign; 2] {
        [self.x_sign(), self.y_sign()]
    }

    pub fn signs_i32(&self) -> [i32; 2] {
        [self.x_sign().to_i32(), self.y_sign().to_i32()]
    }

    pub fn signs_i64(&self) -> [i64; 2] {
        [self.x_sign().to_i64(), self.y_sign().to_i64()]
    }

    pub fn signs_isize(&self) -> [isize; 2] {
        [self.x_sign().to_isize(), self.y_sign().to_isize()]
    }
}

impl Dir4 {
    pub fn inv(&self) -> Dir4 {
        match self {
            Dir4::N => Dir4::S,
            Dir4::E => Dir4::W,
            Dir4::S => Dir4::N,
            Dir4::W => Dir4::E,
        }
    }
}

/// One of the eight directions: N, NE, E, SE, ..
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir8 {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

impl Dir8 {
    pub fn from_signs(signs: [Sign; 2]) -> Option<Self> {
        let x = signs[0].to_i8();
        let y = signs[1].to_i8();

        Some(match [x, y] {
            [0, 0] => return None,
            // clockwise
            [0, -1] => Dir8::N,
            [1, -1] => Dir8::NE,
            [1, 0] => Dir8::E,
            [1, 1] => Dir8::SE,
            [0, 1] => Dir8::S,
            [-1, 1] => Dir8::SW,
            [-1, 0] => Dir8::W,
            [-1, -1] => Dir8::NW,
            _ => unreachable!(),
        })
    }

    pub fn x_sign(&self) -> Sign {
        use Dir8::*;
        use Sign::*;

        match self {
            W | NW | SW => Neg,
            E | NE | SE => Pos,
            N | S => Neutral,
        }
    }

    pub fn y_sign(&self) -> Sign {
        use Dir8::*;
        use Sign::*;

        match self {
            N | NE | NW => Neg,
            S | SE | SW => Pos,
            E | W => Neutral,
        }
    }

    pub fn signs(&self) -> [Sign; 2] {
        [self.x_sign(), self.y_sign()]
    }

    pub fn signs_i32(&self) -> [i32; 2] {
        [self.x_sign().to_i32(), self.y_sign().to_i32()]
    }

    pub fn signs_i64(&self) -> [i64; 2] {
        [self.x_sign().to_i64(), self.y_sign().to_i64()]
    }

    pub fn signs_isize(&self) -> [isize; 2] {
        [self.x_sign().to_isize(), self.y_sign().to_isize()]
    }
}

impl Dir8 {
    pub const CLOCKWISE: &'static [Dir8; 8] = {
        use Dir8::*;
        &[N, NE, E, SE, S, SW, W, NW]
    };

    pub fn inv(&self) -> Self {
        match self {
            Dir8::N => Dir8::S,
            Dir8::NE => Dir8::SW,
            Dir8::E => Dir8::W,
            Dir8::SE => Dir8::NW,
            Dir8::S => Dir8::N,
            Dir8::SW => Dir8::NE,
            Dir8::W => Dir8::E,
            Dir8::NW => Dir8::SE,
        }
    }

    pub fn r45(&self) -> Self {
        match self {
            Dir8::N => Dir8::NE,
            Dir8::NE => Dir8::E,
            Dir8::E => Dir8::SE,
            Dir8::SE => Dir8::S,
            Dir8::S => Dir8::SW,
            Dir8::SW => Dir8::W,
            Dir8::W => Dir8::NW,
            Dir8::NW => Dir8::N,
        }
    }

    pub fn l45(&self) -> Self {
        match self {
            Dir8::N => Dir8::NW,
            Dir8::NE => Dir8::W,
            Dir8::E => Dir8::NE,
            Dir8::SE => Dir8::E,
            Dir8::S => Dir8::SE,
            Dir8::SW => Dir8::SW,
            Dir8::W => Dir8::SW,
            Dir8::NW => Dir8::W,
        }
    }

    pub fn r90(&self) -> Self {
        match self {
            Dir8::N => Dir8::E,
            Dir8::NE => Dir8::SE,
            Dir8::E => Dir8::S,
            Dir8::SE => Dir8::SW,
            Dir8::S => Dir8::W,
            Dir8::SW => Dir8::NW,
            Dir8::W => Dir8::N,
            Dir8::NW => Dir8::NE,
        }
    }

    pub fn l90(&self) -> Self {
        match self {
            Dir8::N => Dir8::W,
            Dir8::NE => Dir8::NE,
            Dir8::E => Dir8::N,
            Dir8::SE => Dir8::NE,
            Dir8::S => Dir8::E,
            Dir8::SW => Dir8::SE,
            Dir8::W => Dir8::S,
            Dir8::NW => Dir8::SW,
        }
    }
}
