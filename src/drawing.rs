use crate::{CHAR_HEIGHT, CHAR_WIDTH};

pub struct Drawing {
    pub data: Vec<Vec<BraileChar>>,
    pub dirty: bool,
}

impl Drawing {
    pub(crate) fn update(&mut self, x: u32, y: u32) {
        let row_id = y/CHAR_HEIGHT;
        let row_pixel = y % CHAR_HEIGHT;
        let col_id = x / CHAR_WIDTH;
        let col_pixel = x % CHAR_WIDTH;

        let cell = match self.data.get_mut(row_id as usize) {
            Some(row) => {
                match row.get_mut(col_id as usize) {
                    Some(cell) => cell,
                    None => return
                }
            },
            None => return
        };
        if cell.set_pixel(col_pixel, row_pixel, CHAR_WIDTH, CHAR_HEIGHT) {
            self.dirty = true;
        }
    }

    pub(crate) fn new(rows: u32, columns: u32) -> Drawing {
        Drawing {
            data: (0..rows).map(|_| {
                (0..columns).map(|_| {
                    BraileChar::new()
                }).collect::<Vec<BraileChar>>()
            }).collect::<Vec<Vec<BraileChar>>>(),
            dirty: true
        }
    }

    pub(crate) fn draw(&mut self) {
        if !self.dirty {
            return;
        }
        print!("┌");
        (0..self.data.first().unwrap().len()).for_each(|_| print!("─"));
        println!("┐");
        for row in &self.data {
            print!("│");
            for cell in row {
                print!("{}", cell.render());
            }
            println!("│");
        }
        print!("└");
        (0..self.data.first().unwrap().len()).for_each(|_| print!("─"));
        println!("┘");
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        self.dirty = false;
    }
}

pub struct BraileChar {
    bits: [bool; 8],
}

impl BraileChar {
    fn new() -> BraileChar {
        BraileChar {
            bits: [false; 8],
        }
    }

    pub(crate) fn render(&self) -> char {
        let val: u32 = self.bits.iter().enumerate().map(|(i, v)| {if *v {1u32<<i} else {0u32}}).sum();
        let codepoint = 0x2800+val;
        char::from_u32(codepoint).unwrap()
    }

    pub(crate) fn set_pixel(&mut self, x: u32, y: u32, width:u32, height:u32) -> bool {
        // https://en.wikipedia.org/wiki/Braille_Patterns
        // 1 4
        // 2 5
        // 3 6
        // 7 8
        let col = x / (width/2);
        let row = y / (height/4);
        let bitnum = match col {
            0 => {
                match row {
                    0 => 1,
                    1 => 2,
                    2 => 3,
                    3 => 7,
                    _ => return false,
                }
            },
            1 => {
                match row {
                    0 => 4,
                    1 => 5,
                    2 => 6,
                    3 => 8,
                    _ => return false,
                }
            }
            _ => return false,
        };
        let dirty = self.bits[bitnum-1] == false;
        self.bits[bitnum-1] = true;
        dirty
    }


}


#[cfg(test)]
mod tests {
    use crate::drawing::{BraileChar, Drawing, CHAR_WIDTH, CHAR_HEIGHT};
    #[test]
    fn test_braile_render() {
        let mut braile_char = BraileChar::new();
        assert_eq!(braile_char.set_pixel(0, 0, CHAR_WIDTH, CHAR_HEIGHT), true);
        assert_eq!(braile_char.render(), '⠁');

        let mut braile_char = BraileChar::new();
        assert_eq!(braile_char.set_pixel(CHAR_WIDTH / 2, CHAR_HEIGHT * 3 / 4, CHAR_WIDTH, CHAR_HEIGHT), true);
        assert_eq!(braile_char.render(), '⢀');
    }

    #[test]
    fn test_drawing_render() {
        let mut drawing = Drawing::new(12, 60);
        drawing.update(0,0);
        assert_eq!(drawing.data[0][0].render(), '⠁');
        drawing.update(CHAR_WIDTH, 0);
        assert_eq!(drawing.data[0][1].render(), '⠁');
        drawing.update(0, CHAR_HEIGHT);
        assert_eq!(drawing.data[1][0].render(), '⠁');
    }
}