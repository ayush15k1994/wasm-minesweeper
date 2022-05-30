use std::fmt::Display;
use crate::random::random_range;
use std::collections::HashSet;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

fn mine_to_emoji(num_mines: u8) -> &'static str {
    match num_mines {
        0 => "🟦 ",    
        1 => "1️⃣ ",    
        2 => "2️⃣ ",    
        3 => "3️⃣ ",    
        4 => "4️⃣ ",    
        5 => "5️⃣ ",    
        6 => "6️⃣ ",    
        7 => "7️⃣ ",    
        8 => "8️⃣ ",    
        _ => { "" }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x,y);

                if !self.open_fields.contains(&pos) {
                    if self.lost && self.mines.contains(&pos) {
                        f.write_str("💣 ")?;    
                    } else if self.flagged_fields.contains(&pos) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } 
                else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f, "{}", mine_to_emoji(self.neighboring_mines(pos)))?;
                    write!(f, " ");
                }
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl Minesweeper {

    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width: width, height: height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;
        
        (x.max(1)-1 ..=(x+1).min(width-1))
            .flat_map(move |i| {
                (y.max(1)-1..=(y+1).min(height-1))
                .map(move |j| (i, j))
            })
            .filter(move |&pos| pos != (x, y)) 
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }
 
    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.open_fields.contains(&position) {
            let mine_count = self.neighboring_mines(position);
            let flag_count = self.iter_neighbors(position)
                .filter(|neighbor| self.flagged_fields.contains(&neighbor))
                .count() as u8;
            
            if mine_count == flag_count {
                for neighbor in self.iter_neighbors(position) {
                    if !self.flagged_fields.contains(&neighbor)
                        && !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            return None;
        }

        if self.lost 
            || self.flagged_fields.contains(&position)  {
            return None;
        }

        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighboring_mines(position);

            if mine_count == 0 {
                for neighbor in self.iter_neighbors(position) {
                    if !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            Some(OpenResult::NoMine(8))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }

    pub fn is_lost(&self) -> bool {
        self.lost
    }

    pub fn is_won(&self) -> bool {
        if self.lost {
            return false
        } 

        if self.flagged_fields.len() + self.open_fields.len() == self.height * self.width {
            return true
        }

        return false
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        ms.open((6, 6));

        println!("{}", ms);
    }
}