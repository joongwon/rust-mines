use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Mine,
}

struct Mines {
    width: usize,
    height: usize,
    mines: usize,
    board: Vec<Tile>,
}

impl Mines {
    pub fn new(seed: u32, width: usize, height: usize, mines: usize) -> Mines {
        let mut rand = rand::rngs::SmallRng::seed_from_u64(seed as u64);
        assert!(width > 0);
        assert!(height > 0);
        assert!(mines > 0);
        assert!(mines < width * height);

        let mut board = vec![Tile::Mine; mines];
        board.append(&mut vec![Tile::Empty; width * height - mines]);
        board.shuffle(&mut rand);

        Mines {
            width,
            height,
            mines,
            board,
        }
    }

    fn count_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                    continue;
                }
                if let Tile::Mine = self.board[ny as usize * self.width + nx as usize] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Copy, Clone)]
enum ViewTile {
    Hidden,
    Revealed(u8),
    Exploded,
    Flagged,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum PrimitiveViewTile {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    Hidden,
    Exploded,
    Flagged,
}

impl From<ViewTile> for PrimitiveViewTile {
    fn from(value: ViewTile) -> Self {
        match value {
            ViewTile::Hidden => PrimitiveViewTile::Hidden,
            ViewTile::Revealed(0) => PrimitiveViewTile::R0,
            ViewTile::Revealed(1) => PrimitiveViewTile::R1,
            ViewTile::Revealed(2) => PrimitiveViewTile::R2,
            ViewTile::Revealed(3) => PrimitiveViewTile::R3,
            ViewTile::Revealed(4) => PrimitiveViewTile::R4,
            ViewTile::Revealed(5) => PrimitiveViewTile::R5,
            ViewTile::Revealed(6) => PrimitiveViewTile::R6,
            ViewTile::Revealed(7) => PrimitiveViewTile::R7,
            ViewTile::Revealed(8) => PrimitiveViewTile::R8,
            ViewTile::Revealed(_) => unreachable!("Invalid number of mines"),
            ViewTile::Exploded => PrimitiveViewTile::Exploded,
            ViewTile::Flagged => PrimitiveViewTile::Flagged,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
}

#[wasm_bindgen]
pub struct MinesView {
    mines: Mines,
    tiles: Vec<ViewTile>,
    pub status: GameStatus,
    revealed: usize,
}

#[wasm_bindgen]
impl MinesView {
    pub fn generate(seed: u32, width: usize, height: usize, mines: usize) -> Self {
        let mines = Mines::new(seed, width, height, mines);
        MinesView::new(mines)
    }

    fn new(mines: Mines) -> MinesView {
        let tiles = vec![ViewTile::Hidden; mines.width * mines.height];
        MinesView {
            mines,
            tiles,
            status: GameStatus::Playing,
            revealed: 0,
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        let GameStatus::Playing = self.status else {
            return;
        };
        if x >= self.mines.width || y >= self.mines.height {
            return;
        }
        if let Tile::Mine = self.mines.board[y * self.mines.width + x] {
            self.tiles[y * self.mines.width + x] = ViewTile::Exploded;
            self.status = GameStatus::Lost;
            return;
        }
        let mut queue = std::collections::VecDeque::<(usize, usize)>::from([(x, y)]);
        while let Some((x, y)) = queue.pop_front() {
            let index = y * self.mines.width + x;
            let ViewTile::Hidden = self.tiles[index] else {
                continue;
            };
            let count = self.mines.count_mines(x, y);
            self.tiles[index] = ViewTile::Revealed(count);
            self.revealed += 1;
            if count != 0 {
                continue;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0
                        || nx >= self.mines.width as isize
                        || ny < 0
                        || ny >= self.mines.height as isize
                    {
                        continue;
                    }
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
        if self.revealed == self.mines.width * self.mines.height - self.mines.mines {
            self.status = GameStatus::Won;
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        let GameStatus::Playing = self.status else {
            return;
        };
        if x >= self.mines.width || y >= self.mines.height {
            return;
        }
        let index = y * self.mines.width + x;
        match self.tiles[index] {
            ViewTile::Hidden => self.tiles[index] = ViewTile::Flagged,
            ViewTile::Flagged => self.tiles[index] = ViewTile::Hidden,
            _ => {}
        }
    }

    pub fn reveal_around(&mut self, x: usize, y: usize) {
        if x >= self.mines.width || y >= self.mines.height {
            return;
        }
        let GameStatus::Playing = self.status else {
            return;
        };
        let num = match self.tiles[y * self.mines.width + x] {
            ViewTile::Revealed(num) => num,
            _ => return,
        };
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0
                    || nx >= self.mines.width as isize
                    || ny < 0
                    || ny >= self.mines.height as isize
                {
                    continue;
                }
                let index = ny as usize * self.mines.width + nx as usize;
                if let ViewTile::Flagged = self.tiles[index] {
                    count += 1;
                }
            }
        }
        if count == num {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0
                        || nx >= self.mines.width as isize
                        || ny < 0
                        || ny >= self.mines.height as isize
                    {
                        continue;
                    }
                    let index = ny as usize * self.mines.width + nx as usize;
                    if let ViewTile::Hidden = self.tiles[index] {
                        self.reveal(nx as usize, ny as usize);
                    }
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> PrimitiveViewTile {
        let index = y * self.mines.width + x;
        self.tiles[index].into()
    }

    pub fn get_by_index(&self, index: usize) -> PrimitiveViewTile {
        self.tiles[index].into()
    }

    pub fn width(&self) -> usize {
        self.mines.width
    }

    pub fn height(&self) -> usize {
        self.mines.height
    }
}

impl Display for MinesView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.mines.height {
            for x in 0..self.mines.width {
                let index = y * self.mines.width + x;
                match self.tiles[index] {
                    ViewTile::Hidden => write!(f, "# ")?,
                    ViewTile::Revealed(0) => write!(f, ". ")?,
                    ViewTile::Revealed(n) => write!(f, "{} ", n)?,
                    ViewTile::Exploded => write!(f, "X ")?,
                    ViewTile::Flagged => write!(f, "F ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(seed: u32) -> String {
        let mines = Mines::new(seed, 10, 10, 10);
        let mut view = MinesView::new(mines);
        view.reveal(0, 0);
        format!("{}", view)
    }

    #[test]
    fn test() {
        println!("{}", generate(0));
    }
}
