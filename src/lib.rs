/*
  lib.rs
  contains program logic
*/
use std::fmt;

pub struct Board {
        pub marks: [[char; 3]; 3],
}

impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, 
                "  1   2   3\n1 {} | {} | {}\n -----------\n2 {} | {} | {}\n -----------\n3 {} | {} | {}", 
                self.marks[0][0], self.marks[0][1], self.marks[0][2], 
                self.marks[1][0], self.marks[1][1], self.marks[1][2],
                self.marks[2][0], self.marks[2][1], self.marks[2][2])
        }
}
