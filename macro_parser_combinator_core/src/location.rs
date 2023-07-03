
#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

impl Location {
    pub fn new() -> Self {
        Self {
            line: 1,
            col: 1,
            offset: 0,
        }
    }
    pub fn update(self, input: &str) -> (Self, Self) {
        let (col_inc, line_inc, offset_inc) = input
            .as_bytes()
            .iter()
            .fold((0,0,0), |(c, l, o),&i| {
                if i == b'\n' {//TODO:\r?
                    (1, l+1, o+1)
                }else {
                    (c+1, l, o+1)
                }
            });
        (Location{line: self.line+line_inc, col: if line_inc == 0 {self.col+col_inc} else {col_inc}, offset: self.offset + offset_inc},
            Location{line: line_inc, col: col_inc, offset: offset_inc})
    }
    pub fn update_char(self, input: char) -> (Self, Self) {
        if input == '\n' {//TODO:\r?
            (Location{line: self.line+1, col: 1, offset: self.offset + 1 },
                Location{line: 1, col: 1, offset: 1 })
        }else {
            (Location{line: self.line, col: self.col+1, offset: self.offset + 1 },
                Location{line: 0, col: 1, offset: 1 })
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_loc() {
    let s = "abc \n bcd";
    let loc = Location::new();
    let ret = loc.update(s);
    println!("{},{}", ret.1.line, ret.1.col);
}
