
pub const FIELD_WIDTH: i8 = 16;
pub const FIELD_HEIGHT: i8 = 16;
extern crate rand;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Position {
    pub x: i8,
    pub y: i8
}

impl Position {

    fn truncate(pos: i8, limit: i8) -> i8 {
        if limit < 0 {
            panic!("limit must be positive number");
        }
        if pos >= limit {
            return pos % limit;
        } else if pos < 0 {
            return limit + pos;
        } else {
            return pos;
        }
    }
    /* returns a new Position */
    pub fn add(&self, uv: &UnitVector) -> Position {

        return Position {
            x: Position::truncate(self.x + uv.x, FIELD_WIDTH),
            y: Position::truncate(self.y + uv.y, FIELD_HEIGHT)
        }
    }


    pub fn new() -> Position {
        use self::rand::Rng;

        let mut rng = rand::thread_rng();

        Position {
            x: (rng.gen::<i8>() % FIELD_WIDTH).abs(),
            y: (rng.gen::<i8>() % FIELD_HEIGHT).abs()
        }
    }

}

#[derive(Debug, Clone)]
pub struct UnitVector {
    pub x: i8,
    pub y: i8
}

pub const UP: UnitVector = UnitVector {
    x: 0,
    y: -1
};
pub const DOWN: UnitVector = UnitVector {
    x: 0,
    y: 1
};
pub const LEFT: UnitVector = UnitVector {
    x: -1,
    y: 0
};
pub const RIGHT: UnitVector = UnitVector {
    x: 1,
    y: 0
};

static UNITVECTORS: &'static [UnitVector] = &[UP, DOWN, LEFT, RIGHT];

impl UnitVector {
    pub fn new() -> UnitVector {
        use self::rand::Rng;

        let mut rng = rand::thread_rng();
        let index = rng.gen::<usize>() % UNITVECTORS.len();
        return UNITVECTORS[index].clone();
    }

    pub fn dist_squared(&self, other: &UnitVector) -> i8 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}



