#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet, VecDeque};

////////////////////////////////////////////////////////////////////////////////

/// Represents a tile on a board. A tile can either be empty or a number from 1 to 8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tile(u8);

impl Tile {
    /// Creates a new tile.
    ///
    /// # Arguments
    ///
    /// * `maybe_value` - Some(1..=8) or None.
    ///
    /// # Panics
    ///
    /// Panics if value is 0 or > 8.
    pub fn new(maybe_value: Option<u8>) -> Self {
        if let Some(n) = maybe_value {
            assert!(n > 0u8 && n < 9u8, "invalid tile value: {}", n);
            Self(n)
        } else {
            Self(0)
        }
    }

    /// Creates an empty tile.
    pub fn empty() -> Self {
        Self(0)
    }

    /// Returns `Some(value)` if tile contains a value, otherwise returns `None`.
    pub fn number(&self) -> Option<u8> {
        if self.0 > 0 {
            Some(self.0)
        } else {
            None
        }
    }

    /// Returns true if tile does not contain a value.
    pub fn is_empty(&self) -> bool {
        self.number().is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a 3x3 board of tiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    tiles: [[Tile; 3]; 3],
}

impl Board {
    /// Creates a new `Board` from a 3x3 matrix if `Tile`s.
    ///
    /// # Panics
    ///
    /// Panics if `tiles` contains more than one instance if some tile.
    pub fn new(tiles: [[Tile; 3]; 3]) -> Self {
        let set = tiles.iter().flatten().cloned().collect::<HashSet<_>>();
        assert_eq!(set.len(), 9, "tiles have duplicate values {:?}", tiles);
        Self {
            tiles
        }
    }

    /// Returns a tile on a given `row` and `col`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` > 2.
    pub fn get(&self, row: usize, col: usize) -> Tile {
        self.tiles[row][col]
    }

    /// Swaps two given tiles.
    ///
    /// # Panics
    ///
    /// Panics if some of `r1`, `r2`, `c1` or `c2` > 2.
    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        let tile = self.tiles[r1][c1];
        self.tiles[r1][c1] = self.tiles[r2][c2];
        self.tiles[r2][c2] = tile;
    }

    /// Parses `Board` from string.
    ///
    /// # Arguments
    ///
    /// * `s` must be a string in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    ///
    /// # Panics
    ///
    /// Panics of `s` is the wrong format or does not represent a valid `Board`.
    pub fn from_string(s: &str) -> Self {
        let len: usize = s.len();
        assert_eq!(len, 12, "invalid input: {}", s);

        let mut tiles = [[Tile::empty(); 3]; 3];
        for r in 0..3 {
            for (c, chr) in s.bytes().skip(r * 4).take(3).enumerate() {
                if chr > b'0' && chr < b'9' {
                    tiles[r][c] = Tile::new(Some(chr - b'0'));
                }
            }
        }

        Self::new(tiles)
    }

    /// Returns a string representation of this board in the following format:
    ///
    /// '''
    /// .12
    /// 345
    /// 678
    /// '''
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for r in 0..3 {
            for c in 0..3 {
                if let Some(n) = self.get(r, c).number() {
                    str.push((b'0' + n) as char);
                } else {
                    str.push('.');
                }
            }
            str.push('\n');
        }
        str
    }

    // You might want to add some more methods here.

    pub fn is_solved(&self) -> bool {
        let set = self.tiles.iter().flatten().cloned().collect::<HashSet<_>>();
        for (i, tile) in set.iter().enumerate() {
            if let Some(n) = tile.number() {
                if n != i as u8 + 1 {
                    return false;
                }
            } else if i + 1 == set.len() {
                return true;
            }
        }
        false
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Returns the shortest sequence of moves that solves this board.
/// That is, a sequence of boards such that each consecutive board can be obtained from
/// the previous one via a single swap of an empty tile with some adjacent tile,
/// and the final board in the sequence is
///
/// '''
/// 123
/// 456
/// 78.
/// '''
///
/// If the board is unsolvable, returns `None`. If the board is already solved,
/// returns an empty vector.
pub fn solve(start: Board) -> Option<Vec<Board>> {
    if start.is_solved() {
        return Some(vec![]);
    }
    
    let queue = Vec::<Board>::new();
    // skip...
    Some(queue)
}

pub fn is_solvable(arr: &[u8]) -> bool {
    let mut inversions = 0;

    let len = arr.len();
    for i in 0..len {
        for j in i+1..len {
            if arr[i] != 0 && arr[j] != 0 && arr[i] > arr[j] {
                inversions += 1;
            }
        }
    }
    inversions % 2 == 0
}
