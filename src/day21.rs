use utils;
use utils::Part;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct TwoPattern {
    a0:char, a1:char,
    b0:char, b1:char,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct ThreePattern {
    a0:char, a1:char, a2:char,
    b0:char, b1:char, b2:char,
    c0:char, c1:char, c2:char,
}

struct FourPattern {
    a0:char, a1:char, a2:char, a3:char,
    b0:char, b1:char, b2:char, b3:char,
    c0:char, c1:char, c2:char, c3:char,
    d0:char, d1:char, d2:char, d3:char,
}

#[derive(Debug)]
enum ArtPiece {
    Even(TwoPattern),
    Odd(ThreePattern),
}

pub fn solve(part: Part) -> i32 {
    const START:&str = ".#./..#/###";

    let mut input = String::new();
    utils::read_input_to_string(&mut input, 21).unwrap();

    let mut out = 0;

    let mut two_rules = HashMap::new();
    let mut three_rules = HashMap::new();

    let mut art:Vec<Vec<ArtPiece>> = vec![vec![ArtPiece::Odd(ThreePattern::from(START))]];

    println!("Start: {:?}", art[0][0]);

    for line in input.lines() {
        let tokens:Vec<&str> = line.split_whitespace().collect();
        match tokens[0].len() {
            5 => {
                two_rules.insert(TwoPattern::from(tokens[0]), ThreePattern::from(tokens[2]));
            },
            11 => {
                three_rules
                    .insert(ThreePattern::from(tokens[0]), FourPattern::from(tokens[2]));
            },
            _ => panic!("Unrecognized Input")
        }
    }

    let iterations = 5;

    for _ in 0..iterations {
        let mut new_art:Vec<Vec<ArtPiece>> = Vec::new();
        for row in 0..art.len() {
            match art[0][0] {
                ArtPiece::Even(_) => {
                    new_art.push(Vec::new());
                },
                ArtPiece::Odd(_) => {
                    new_art.push(Vec::new());
                    new_art.push(Vec::new()); //need another row to insert FourPattern split
                },
            }
            for col in 0..art.len() {
                match art[row][col] {
                    ArtPiece::Even(tile) => {
                        for p in tile.permutations() {
                            match two_rules.get(&p) {
                                Some(new_tile) => {
                                    println!("Matching Rule Found: Replacing {:?}with {:?}",
                                        tile, new_tile);
                                    new_art[row].push(ArtPiece::Odd(*new_tile));
                                    break;
                                },
                                None => {},
                            }
                        }
                    },
                    ArtPiece::Odd(tile) => {
                        for p in tile.permutations() {
                            match three_rules.get(&p) {
                                Some(new_tile) => {
                                    println!("Matching Rule Found: Replacing {:?}with {:?}",
                                        tile, new_tile);
                                    let new_tiles = new_tile.split();
                                    new_art[2*row].push(ArtPiece::Even(new_tiles[0]));
                                    new_art[2*row].push(ArtPiece::Even(new_tiles[1]));
                                    new_art[2*row+1].push(ArtPiece::Even(new_tiles[2]));
                                    new_art[2*row+1].push(ArtPiece::Even(new_tiles[3]));
                                    break;
                                },
                                None => {},
                            }
                        }
                    },
                }
            }
        }
        art = new_art;
    }

    let mut count = 0;

    for row in art {
        for piece in row {
            match piece {
                ArtPiece::Even(p) => {
                    count += p.count_on();
                },
                ArtPiece::Odd(p) => {
                    count += p.count_on();
                },
            }
        }
    }

    out = count;

    out
}

impl TwoPattern {
    fn rotated90r (&self) -> Self {
        TwoPattern {
            a0:self.b0, a1:self.a0,
            b0:self.b1, b1:self.a1,
        }
    }
    fn flipped_vert (&self) -> Self {
        TwoPattern {
            a0:self.a1, a1:self.a0,
            b0:self.b1, b1:self.b0,
        }
    }
    fn from(s:&str) -> Self {
        let cleaned = s.trim().replace('/',"");
        let mut tokens = cleaned.chars();
        TwoPattern{
            a0:tokens.next().unwrap(), a1:tokens.next().unwrap(),
            b0:tokens.next().unwrap(), b1:tokens.next().unwrap(),
        }
    }
    fn permutations(&self) -> Vec<Self> {
        vec![*self, self.rotated90r(), self.rotated90r().rotated90r(),
                self.rotated90r().rotated90r().rotated90r(), self.flipped_vert(),
                self.flipped_vert().rotated90r(), self.flipped_vert().rotated90r().rotated90r(),
                self.flipped_vert().rotated90r().rotated90r().rotated90r()]
    }
    fn count_on(&self) -> i32 {
        let mut count = 0;
        if self.a0 == '#' {count+=1;}
        if self.a1 == '#' {count+=1;}
        if self.b0 == '#' {count+=1;}
        if self.b1 == '#' {count+=1;}
        println!("Counted {} pixels on for {:?}", count, self);
        count
    }
}

impl fmt::Debug for TwoPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TwoPattern: \n{}{}\n{}{}\n", self.a0, self.a1, self.b0, self.b1)
    }
}

impl ThreePattern {
    fn rotated90r (&self) -> Self {
        ThreePattern {
            a0:self.b0, a1:self.a0, a2:self.a1,
            b0:self.c0, b1:self.b1, b2:self.a2,
            c0:self.c1, c1:self.c2, c2:self.b2,
        }
    }

    fn flipped_vert (&self) -> Self {
        ThreePattern {
            a0:self.a2, a1:self.a1, a2:self.a0,
            b0:self.b2, b1:self.b1, b2:self.b0,
            c0:self.c2, c1:self.c1, c2:self.c0,
        }
    }
    fn from(s:&str) -> Self {
        let cleaned = s.trim().replace('/',"");
        let mut tokens = cleaned.chars();
        ThreePattern{
            a0:tokens.next().unwrap(), a1:tokens.next().unwrap(), a2:tokens.next().unwrap(),
            b0:tokens.next().unwrap(), b1:tokens.next().unwrap(), b2:tokens.next().unwrap(),
            c0:tokens.next().unwrap(), c1:tokens.next().unwrap(), c2:tokens.next().unwrap(),
        }
    }
    fn permutations(&self) -> Vec<Self> {
        vec![*self, self.rotated90r(), self.rotated90r().rotated90r(),
             self.rotated90r().rotated90r().rotated90r(), self.flipped_vert(),
             self.flipped_vert().rotated90r(), self.flipped_vert().rotated90r().rotated90r(),
             self.flipped_vert().rotated90r().rotated90r().rotated90r()]
    }
    fn count_on(&self) -> i32 {
        let mut count = 0;
        if self.a0 == '#' {count+=1;}
        if self.a1 == '#' {count+=1;}
        if self.a2 == '#' {count+=1;}
        if self.b0 == '#' {count+=1;}
        if self.b1 == '#' {count+=1;}
        if self.b2 == '#' {count+=1;}
        if self.c0 == '#' {count+=1;}
        if self.c1 == '#' {count+=1;}
        if self.c2 == '#' {count+=1;}
        count
    }
}

impl fmt::Debug for ThreePattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreePattern: \n{}{}{}\n{}{}{}\n{}{}{}\n", self.a0, self.a1, self.a2,
                                                                self.b0, self.b1, self.b2,
                                                                self.c0, self.c1, self.c2)
    }
}

impl FourPattern {
    fn from(s:&str) -> Self {
        let cleaned = s.trim().replace('/', "");
        let mut tokens = cleaned.chars();
        FourPattern{
            a0:tokens.next().unwrap(), a1:tokens.next().unwrap(), a2:tokens.next().unwrap(),
                a3:tokens.next().unwrap(),
            b0:tokens.next().unwrap(), b1:tokens.next().unwrap(), b2:tokens.next().unwrap(),
                b3:tokens.next().unwrap(),
            c0:tokens.next().unwrap(), c1:tokens.next().unwrap(), c2:tokens.next().unwrap(),
                c3:tokens.next().unwrap(),
            d0:tokens.next().unwrap(), d1:tokens.next().unwrap(), d2:tokens.next().unwrap(),
                d3:tokens.next().unwrap(),
        }
    }

    fn split(&self) -> Vec<TwoPattern> {
        vec![TwoPattern{ a0:self.a0, a1:self.a1,
                        b0:self.b0, b1:self.b1,}, TwoPattern{ a0:self.a2, a1:self.a3,
                                                                b0:self.b2, b1:self.b3,},
            TwoPattern{ a0:self.c0, a1:self.c1,
                        b0:self.d0, b1:self.d1,}, TwoPattern{ a0:self.c2, a1:self.c3,
                                                                b0:self.d2, b1:self.d3}]
    }
}

impl fmt::Debug for FourPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FourPattern: \n{}{}{}{}\n{}{}{}{}\n{}{}{}{}\n{}{}{}{}\n",
               self.a0, self.a1, self.a2, self.a3,
                self.b0, self.b1, self.b2, self.b3,
                self.c0, self.c1, self.c2, self.c3,
                self.d0, self.d1, self.d2, self.d3)
    }
}