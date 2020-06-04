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
    // TODO : map, all, any$B$"$?$j$r;H$&$H$b$C$H4J7i$K=q$1$k$i$7$$$N$G=$@5(B
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
    grounded: bool,
}

impl<T: mino::Mino> ControlledMino<T> {
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_grounded(&self) -> bool {
        self.grounded
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

    // TODO : $B2sE>8e$N>uBV$,IT@5$G$J$$$+$NH=Dj$rDI2C(B
    // TODO : SRS$B$NF3F~(B
    pub fn right_rotate(&mut self) {
        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Rightward,
            Orientation::Rightward => Orientation::Downward,
            Orientation::Downward => Orientation::Leftward,
            Orientation::Leftward => Orientation::Upward,
        };
    }

    // TODO : $B2sE>8e$N>uBV$,IT@5$G$J$$$+$NH=Dj$rDI2C(B
    // TODO : SRS$B$NF3F~(B
    pub fn left_rotate(&mut self) {
        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Leftward,
            Orientation::Rightward => Orientation::Upward,
            Orientation::Downward => Orientation::Rightward,
            Orientation::Leftward => Orientation::Downward,
        }
    }

    // move$B$OM=Ls8l$i$7$$$N$G;H$($J$$(B
    // $B%_%N$r0\F0$5$;$k(B
    pub fn move_mino(&mut self, field: &Field, ori: Orientation) {
        let size = self.mino.get_size();
        let rendered_mino = self.render();
        let mut moved_mino_x = self.get_x();
        let mut moved_mino_y = self.get_y();

        match ori {
            Orientation::Upward => return,
            Orientation::Rightward => moved_mino_x += 1,
            Orientation::Downward => moved_mino_y += 1,
            Orientation::Leftward => {
                if moved_mino_x == 0 {
                    return;
                } else {
                    moved_mino_x -= 1
                }
            }
        }

        println!("{} {}", moved_mino_x, moved_mino_y);
        println!("{:?}", rendered_mino);

        // $B%_%N$r0l$D2<$K0\F0$5$;$k$3$H$,2DG=$+H=Dj(B
        let mut movable = true;
        for i in 0..size {
            for j in 0..size {
                if rendered_mino[i][j] {
                    let x_in_field = j + moved_mino_x;
                    let y_in_field = i + moved_mino_y;

                    // $B%U%#!<%k%I$N6-3&%A%'%C%/(B
                    // $B0\F0@h$N%V%m%C%/$,Kd$^$C$F$$$J$$$+$r%A%'%C%/(B
                    if x_in_field >= field.get_width()
                        || y_in_field >= field.get_height()
                        || field.blocks[y_in_field][x_in_field].filled
                    {
                        movable = false;
                        break;
                    }
                }
            }
        }

        if movable {
            self.x = moved_mino_x;
            self.y = moved_mino_y;
        } else {
            match ori {
                Orientation::Downward => self.grounded = true,
                _ => {}
            }
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
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
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
                    mino: mino::TMino::default(),
                    ori: Orientation::Rightward,
                    grounded: false,
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
                    mino: mino::TMino::default(),
                    ori: Orientation::Downward,
                    grounded: false,
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
                    mino: mino::TMino::default(),
                    ori: Orientation::Leftward,
                    grounded: false,
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

    #[test]
    fn test_right_rotate() {
        struct TestCase {
            name: String,
            x: Orientation,
            want: i32,
        };

        let cases = vec![
            TestCase {
                name: "upward".to_string(),
                x: Orientation::Upward,
                want: 1,
            },
            TestCase {
                name: "rightward".to_string(),
                x: Orientation::Rightward,
                want: 2,
            },
            TestCase {
                name: "downward".to_string(),
                x: Orientation::Downward,
                want: 3,
            },
            TestCase {
                name: "leftward".to_string(),
                x: Orientation::Leftward,
                want: 0,
            },
        ];

        let mut m = ControlledMino {
            x: 0,
            y: 0,
            mino: mino::TMino::default(),
            ori: Orientation::Upward,
            grounded: false,
        };
        for case in cases {
            m.ori = case.x;
            m.right_rotate();
            let result = match m.ori {
                Orientation::Upward => 0,
                Orientation::Rightward => 1,
                Orientation::Downward => 2,
                Orientation::Leftward => 3,
            };
            assert_eq!(result, case.want, "case {}: failed", case.name)
        }
    }

    #[test]
    fn test_left_rotate() {
        struct TestCase {
            name: String,
            x: Orientation,
            want: i32,
        };

        let cases = vec![
            TestCase {
                name: "upward".to_string(),
                x: Orientation::Upward,
                want: 3,
            },
            TestCase {
                name: "rightward".to_string(),
                x: Orientation::Rightward,
                want: 0,
            },
            TestCase {
                name: "downward".to_string(),
                x: Orientation::Downward,
                want: 1,
            },
            TestCase {
                name: "leftward".to_string(),
                x: Orientation::Leftward,
                want: 2,
            },
        ];

        let mut m = ControlledMino {
            x: 0,
            y: 0,
            mino: mino::TMino::default(),
            ori: Orientation::Upward,
            grounded: false,
        };
        for case in cases {
            m.ori = case.x;
            m.left_rotate();
            let result = match m.ori {
                Orientation::Upward => 0,
                Orientation::Rightward => 1,
                Orientation::Downward => 2,
                Orientation::Leftward => 3,
            };
            assert_eq!(result, case.want, "case {}: failed", case.name)
        }
    }

    #[test]
    fn test_move() {
        struct TestCase {
            name: String,
            x: ControlledMino<mino::TMino>,
            move_ori: Orientation,
            want: (usize, usize, bool),
        };

        let field_height = 5;
        let field_width = 4;
        let field_filled = vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, true],
            vec![false, false, false, true],
        ];

        let cases = vec![
            TestCase {
                name: "落下可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Downward,
                want: (0, 1, false),
            },
            TestCase {
                name: "右移動可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Rightward,
                want: (1, 0, false),
            },
            TestCase {
                name: "左移動可能".to_string(),
                x: ControlledMino {
                    x: 1,
                    y: 0,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Leftward,
                want: (0, 0, false),
            },
            TestCase {
                name: "下のブロックが埋まっているため落下不可能".to_string(),
                x: ControlledMino {
                    x: 1,
                    y: 1,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Downward,
                want: (1, 1, true),
            },
            TestCase {
                name: "ブロックが埋まっているため右移動不可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 3,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Rightward,
                want: (0, 3, false),
            },
            TestCase {
                name: "フィールド境界のため左移動不可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 3,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Leftward,
                want: (0, 3, false),
            },
            TestCase {
                name: "フィールド境界のため落下不可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 3,
                    mino: mino::TMino::default(),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Downward,
                want: (0, 3, true),
            },
        ];

        let mut f = Field::new(field_height, field_width);
        for h in 0..f.get_height() {
            for w in 0..f.get_width() {
                f.blocks[h][w].filled = field_filled[h][w];
            }
        }

        for case in cases {
            let mut input = case.x;
            input.move_mino(&f, case.move_ori);
            assert_eq!(
                (input.get_x(), input.get_y(), input.get_grounded()),
                case.want,
                "case {}: failed",
                case.name
            )
        }
    }
}
