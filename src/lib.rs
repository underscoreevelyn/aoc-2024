pub mod template;

// Use this file to add helper functions and additional modules.

pub mod directions {
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Direction {
        pub x: i32,
        pub y: i32,
    }

    pub const UP: Direction = Direction { x: -1, y: 0 };
    pub const DOWN: Direction = Direction { x: 1, y: 0 };
    pub const LEFT: Direction = Direction { x: 0, y: -1 };
    pub const RIGHT: Direction = Direction { x: 0, y: 1 };
    pub const UPLEFT: Direction = Direction { x: -1, y: -1 };
    pub const UPRIGHT: Direction = Direction { x: -1, y: 1 };
    pub const DOWNLEFT: Direction = Direction { x: 1, y: -1 };
    pub const DOWNRIGHT: Direction = Direction { x: 1, y: 1 };

    pub const DIRECTIONS: [Direction; 8] =
        [UPLEFT, LEFT, DOWNLEFT, DOWN, DOWNRIGHT, RIGHT, UPRIGHT, UP];

    pub const CARDINAL_DIRECTIONS: [Direction; 4] = [UP, LEFT, DOWN, RIGHT];

    impl Direction {
        /// rotates 90 degrees clockwise
        pub fn clockwise(&self) -> Direction {
            Direction {
                x: self.y,
                y: -self.x,
            }
        }

        /// rotates 90 degrees counter clockwise
        pub fn counter_clockwise(&self) -> Direction {
            Direction {
                x: -self.y,
                y: self.x,
            }
        }

        /// rotates 45 degrees clockwise
        /// unclear if this is useful
        pub fn half_clockwise(&self) -> Direction {
            let factor = if self.x == 0 || self.y == 0 { 1 } else { 2 };
            Direction {
                x: (self.x + self.y) / factor,
                y: (self.y - self.x) / factor,
            }
        }

        /// rotates 45 degrees counter clockwise
        /// unclear if this is useful
        pub fn half_counter_clockwise(&self) -> Direction {
            let factor = if self.x == 0 || self.y == 0 { 1 } else { 2 };
            Direction {
                x: (self.x - self.y) / factor,
                y: (self.y + self.x) / factor,
            }
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        /// for my own sanity
        fn test_rotations() {
            assert_eq!(UP.clockwise(), RIGHT);
            assert_eq!(UP.counter_clockwise(), LEFT);

            assert_eq!(RIGHT.clockwise(), DOWN);
            assert_eq!(RIGHT.counter_clockwise(), UP);

            assert_eq!(DOWN.clockwise(), LEFT);
            assert_eq!(DOWN.counter_clockwise(), RIGHT);

            assert_eq!(LEFT.clockwise(), UP);
            assert_eq!(LEFT.counter_clockwise(), DOWN);

            assert_eq!(UP.half_clockwise(), UPRIGHT);
            assert_eq!(UP.half_counter_clockwise(), UPLEFT);
            assert_eq!(UPRIGHT.half_clockwise(), RIGHT);
            assert_eq!(UPRIGHT.half_counter_clockwise(), UP);

            assert_eq!(
                Direction { x: 8, y: 0 }.clockwise(),
                Direction { x: 0, y: -8 }
            );
            assert_eq!(
                Direction { x: 8, y: 0 }.half_clockwise(),
                Direction { x: 8, y: -8 }
            );
            assert_eq!(
                Direction { x: 8, y: -8 }.half_clockwise(),
                Direction { x: 0, y: -8 }
            );
        }
    }
}

pub mod grid {
    use std::{
        fmt::Display,
        iter,
        ops::{Add, Index, IndexMut},
    };

    use crate::directions;

    #[derive(Clone, PartialEq, Eq, Copy, Debug, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("({}, {})", self.x, self.y))
        }
    }

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Self { x, y }
        }
    }

    impl Add<Point> for Point {
        type Output = Point;

        fn add(self, rhs: Point) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<&Point> for Point {
        type Output = Point;

        fn add(self, rhs: &Point) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<directions::Direction> for Point {
        type Output = Point;

        fn add(self, rhs: directions::Direction) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<&directions::Direction> for Point {
        type Output = Point;

        fn add(self, rhs: &directions::Direction) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug, Hash)]
    pub struct Grid<T> {
        grid: Vec<Vec<T>>,
        size: (i32, i32),
    }

    pub trait IntoGrid<T> {
        fn into_grid(self) -> Grid<T>;
    }

    impl<T: Clone> Grid<T> {
        pub fn new(size: (usize, usize), default_value: T) -> Self {
            Self {
                grid: vec![vec![default_value; size.1]; size.0],
                size: (
                    size.0.try_into().expect("size value is way too high"),
                    size.1.try_into().expect("size value is way too high"),
                ),
            }
        }
    }

    impl<I> IntoGrid<<<I as Iterator>::Item as Iterator>::Item> for I
    where
        I: Iterator,
        <I as Iterator>::Item: Iterator,
    {
        fn into_grid(self) -> Grid<<<I as Iterator>::Item as Iterator>::Item> {
            Grid::from_vecs(self.map(|x| x.collect()).collect())
        }
    }

    impl<T> Grid<T> {
        pub fn from_vecs(grid: Vec<Vec<T>>) -> Self {
            let size = (
                grid.len().try_into().expect("grid is way too big"),
                if grid.len() > 0 {
                    grid[0].len().try_into().expect("grid is way too big")
                } else {
                    0
                },
            );
            Self { grid, size }
        }

        pub fn is_inside(&self, index: Point) -> bool {
            index.x >= 0 && index.x < self.size.0 && index.y >= 0 && index.y < self.size.1
        }

        pub fn get(&self, index: Point) -> Option<&T> {
            self.is_inside(index)
                .then(|| &self.grid[index.x as usize][index.y as usize])
        }

        pub fn set(&mut self, index: Point, value: T) -> Option<()> {
            self.is_inside(index)
                .then(|| self.grid[index.x as usize][index.y as usize] = value)
        }

        pub fn get_mut(&mut self, index: Point) -> Option<&mut T> {
            self.is_inside(index)
                .then(|| &mut self.grid[index.x as usize][index.y as usize])
        }

        pub fn enumerate(&self) -> PointIter {
            PointIter::new(self.size)
        }

        pub fn width(&self) -> i32 {
            self.size.0
        }
        pub fn height(&self) -> i32 {
            self.size.1
        }
    }

    impl<T: PartialEq> Grid<T> {
        pub fn index_of(&self, v: T) -> Option<Point> {
            self.enumerate().find(|x| self[x] == v)
        }
    }

    impl<T> Index<Point> for Grid<T> {
        type Output = T;

        fn index(&self, index: Point) -> &Self::Output {
            if !self.is_inside(index) {
                panic!("Attempted to index a grid with an out of bounds point! (Grid has dimensions {0:?}, index {index}", self.size);
            }

            &self.grid[index.x as usize][index.y as usize]
        }
    }

    impl<T> Index<&Point> for Grid<T> {
        type Output = T;

        fn index(&self, index: &Point) -> &Self::Output {
            if !self.is_inside(*index) {
                panic!("Attempted to index a grid with an out of bounds point! (Grid has dimensions {0:?}, index {index}", self.size);
            }

            &self.grid[index.x as usize][index.y as usize]
        }
    }

    impl<T> IndexMut<Point> for Grid<T> {
        fn index_mut(&mut self, index: Point) -> &mut Self::Output {
            if !self.is_inside(index) {
                panic!("Attempted to index a grid with an out of bounds point! (Grid has dimensions {0:?}, index {index}", self.size);
            }

            &mut self.grid[index.x as usize][index.y as usize]
        }
    }

    impl<T> IndexMut<&Point> for Grid<T> {
        fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
            if !self.is_inside(*index) {
                panic!("Attempted to index a grid with an out of bounds point! (Grid has dimensions {0:?}, index {index}", self.size);
            }

            &mut self.grid[index.x as usize][index.y as usize]
        }
    }

    impl ToString for Grid<char> {
        fn to_string(&self) -> String {
            self.grid
                .iter()
                .map(|x| x.iter().chain(iter::once(&'\n')).collect::<String>())
                .collect()
        }
    }

    pub struct PointIter {
        size: i32,
        width: i32,
        cur: i32,
    }

    impl Iterator for PointIter {
        type Item = Point;

        fn next(&mut self) -> Option<Self::Item> {
            (self.cur < self.size).then(|| {
                let p = Point {
                    x: self.cur % self.width,
                    y: self.cur / self.width,
                };
                self.cur += 1;
                p
            })
        }
    }

    impl PointIter {
        pub fn new((width, height): (i32, i32)) -> Self {
            Self {
                size: width * height,
                width,
                cur: 0,
            }
        }
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        fn index_boundaries() {
            let grid = (0..3).map(|x| (x * 2)..(x * 2 + 2)).into_grid();

            assert_eq!(grid.get(Point { x: 0, y: 0 }), Some(&0));
            assert_eq!(grid.get(Point { x: 0, y: 1 }), Some(&1));
            assert_eq!(grid.get(Point { x: 1, y: 0 }), Some(&2));
            assert_eq!(grid.get(Point { x: 1, y: 1 }), Some(&3));
            assert_eq!(grid.get(Point { x: 2, y: 0 }), Some(&4));
            assert_eq!(grid.get(Point { x: 2, y: 1 }), Some(&5));

            //negative
            assert_eq!(grid.get(Point { x: -1, y: 0 }), None);
            // off the bottom
            assert_eq!(grid.get(Point { x: 3, y: 0 }), None);
            // off the right
            assert_eq!(grid.get(Point { x: 1, y: 2 }), None);
        }

        #[test]
        fn enumerate() {
            let grid = Grid::new((3, 2), 0);
            let v: Vec<_> = grid.enumerate().collect();
            assert_eq!(
                v,
                [
                    (0, 0).into(),
                    (1, 0).into(),
                    (2, 0).into(),
                    (0, 1).into(),
                    (1, 1).into(),
                    (2, 1).into()
                ]
            );
        }
    }
}
