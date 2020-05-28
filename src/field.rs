/// 21$B!_(B10$B$N%F%H%j%9$N%U%#!<%k%I$rI=8=(B
/// controller$B$+$i(Bstep$B$,8F$S=P$5$l$=$N$?$S$KMn2<=hM}$d:o=|=hM}$r9T$&M=Dj(B
use crate::mino;
use crate::mino::Mino;

// $B%U%#!<%k%I$N3F%V%m%C%/(B
struct FieldBlock {
    filled: bool,    // $B%V%m%C%/$K%_%N$,B8:_$9$k$+(B
    color: [f32; 4], // $B%V%m%C%/$N?'(B
}

// $B%F%H%j%9$N%U%#!<%k%I(B
struct Field {
    height: usize,
    width: usize,
    blocks: Vec<Vec<FieldBlock>>,
}

impl Field {
    /// Field$B$N%3%s%9%H%i%/%?(B
    pub fn new(height: usize, width: usize) -> Field {
        let mut blocks: Vec<Vec<FieldBlock>> = Vec::new();
        for _ in 0..height {
            let mut tmp_vec: Vec<FieldBlock> = Vec::new();
            for _ in 0..width {
                tmp_vec.push(FieldBlock {
                    filled: false,
                    color: [0 as f32; 4],
                });
            }
            blocks.push(tmp_vec);
        }
        Field {
            height: height,
            width: width,
            blocks: blocks,
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    // $B2#Ns$4$H$K(Bmino$B$,B7$C$F$$$k$+$rH=Dj$7B7$C$F$$$kNs$N%$%s%G%/%9$rJV$9(B
    // $B:o=|$7$?$+$H$$$&>pJs$H:o=|$7$?Ns$N>pJs$rJV$9(B
    pub fn is_filled_each_row(&self) -> Option<Vec<usize>> {
        let mut filled_rows = Vec::new();

        for h in 0..self.get_height() {
            let mut not_filled = false;
            for w in 0..self.get_width() {
                if !self.blocks[h][w].filled {
                    not_filled = true;
                    break;
                }
            }

            if !not_filled {
                filled_rows.push(h);
            }
        }

        if filled_rows.len() == 0 {
            return None;
        }

        return Some(filled_rows);
    }
}

pub enum Orientation {
    Upward,
    Rightward,
    Downward,
    Leftward,
}

struct ControlledMino<T: mino::Mino> {
    x: usize,
    y: usize,
    mino: T,
    ori: Orientation,
}

impl<T: mino::Mino> ControlledMino<T> {
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    // $B%_%N$N<oN`$H8~$-$+$i%U%#!<%k%I>e$G$N>uBV$r@8@.$9$k(B
    // $B%_%N$N8~$-$K$h$C$F(Bclosure$B$r@Z$jBX$($F$$$k(B
    pub fn render(&self) -> Vec<Vec<bool>> {
        let size = self.mino.get_size();
        if size < 1 {
            return vec![];
        }
        let shape = self.mino.get_shape();
        let mut method: Box<dyn FnMut(usize, usize) -> bool> = match self.ori {
            Orientation::Upward => Box::new(|i, j| shape[i][j]),
            Orientation::Rightward => Box::new(|i, j| shape[size - 1 - j][i]),
            Orientation::Downward => Box::new(|i, j| shape[size - 1 - i][size - 1 - j]),
            Orientation::Leftward => Box::new(|i, j| shape[j][size - 1 - i]),
        };
        (0..size)
            .map(|i| (0..size).map(|j| method(i, j)).collect())
            .collect()
    }

    pub fn right_rotate(&self) -> Orientation {
        match &self.ori {
            Orientation::Upward => Orientation::Rightward,
            Orientation::Rightward => Orientation::Downward,
            Orientation::Downward => Orientation::Leftward,
            Orientation::Leftward => Orientation::Upward,
        }
    }

    pub fn left_rotate(&self) -> Orientation {
        match &self.ori {
            Orientation::Upward => Orientation::Leftward,
            Orientation::Rightward => Orientation::Upward,
            Orientation::Downward => Orientation::Rightward,
            Orientation::Leftward => Orientation::Downward,
        }
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;

    #[test]
    fn test_new() {
        // block$B$,$9$Y$FKd$^$C$F$$$J$$$+$r%F%9%H(B
        let f = Field::new(5, 4);
        for h in 0..f.get_height() {
            for w in 0..f.get_width() {
                if f.blocks[h][w].filled {
                    assert!(false);
                }
            }
        }
        assert!(true);
    }

    #[test]
    fn test_is_filled_each_row() {
        let test_height = 5;
        let test_width = 4;

        struct TestCase {
            name: String,
            x: Vec<Vec<bool>>,
            want: Option<Vec<usize>>,
        };

        let cases = vec![
            TestCase {
                name: "all false".to_string(),
                x: vec![vec![false; test_width]; test_height],
                want: None,
            },
            TestCase {
                name: "all true".to_string(),
                x: vec![vec![true; test_width]; test_height],
                want: Some(vec![0, 1, 2, 3, 4]),
            },
            TestCase {
                // $B0lIt$,Kd$^$C$F$$$k(B
                name: "hand craft".to_string(),
                x: vec![
                    vec![true, true, true, true],
                    vec![false, false, true, false],
                    vec![false, true, true, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                ],
                want: Some(vec![0, 3]),
            },
        ];

        for case in cases {
            // block$B$,$9$Y$FKd$^$C$F$$$J$$$+$r%F%9%H(B
            let mut f = Field::new(test_height, test_width);
            for h in 0..f.get_height() {
                for w in 0..f.get_width() {
                    f.blocks[h][w].filled = case.x[h][w];
                }
            }
            match f.is_filled_each_row() {
                Some(return_value) => {
                    assert_eq!(Some(return_value), case.want, "case {}: failed", case.name)
                }
                None => assert_eq!(None, case.want, "failed {}", case.name),
            }
        }
    }
}

#[cfg(test)]
mod controlledmino_tests {
    use super::*;

    #[test]
    fn test_render() {
        struct TestCase {
            name: String,
            x: ControlledMino<mino::TMino>,
            want: Vec<Vec<bool>>,
        };

        let cases = vec![
            TestCase {
                name: "upward".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::new(),
                    ori: Orientation::Upward,
                },
                want: vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
            },
            TestCase {
                name: "right".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::new(),
                    ori: Orientation::Rightward,
                },
                want: vec![
                    vec![false, true, false],
                    vec![false, true, true],
                    vec![false, true, false],
                ],
            },
            TestCase {
                name: "downward".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::new(),
                    ori: Orientation::Downward,
                },
                want: vec![
                    vec![false, false, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ],
            },
            TestCase {
                name: "left".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::new(),
                    ori: Orientation::Leftward,
                },
                want: vec![
                    vec![false, true, false],
                    vec![true, true, false],
                    vec![false, true, false],
                ],
            },
        ];

        for case in cases {
            assert_eq!(case.x.render(), case.want, "case {}: failed", case.name)
        }
    }
}
