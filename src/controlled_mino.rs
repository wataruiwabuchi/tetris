use crate::field;
use crate::mino;

/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Orientation {
    Upward,
    Rightward,
    Downward,
    Leftward,
}

pub struct ControlledMino {
    x: i64, // 左上座標なのでマイナスの値をとりうる
    y: i64,
    ori: Orientation,
    grounded: bool,
    mino: Box<dyn mino::Mino>,
}

impl ControlledMino {
    pub fn new(x: i64, mino: Box<dyn mino::Mino>) -> Self {
        ControlledMino {
            x: x,
            y: 0,
            ori: Orientation::Upward,
            grounded: false,
            mino: mino,
        }
    }
    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }

    pub fn get_grounded(&self) -> bool {
        self.grounded
    }

    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }

    pub fn get_mino(&mut self) -> &mut Box<dyn mino::Mino> {
        &mut self.mino
    }

    // ミノの種類と向きからフィールド上での状態を生成する
    // ミノの向きによってclosureを切り替えている
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

    pub fn right_rotate(&mut self, field: &field::Field) {
        let original_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Rightward,
            Orientation::Rightward => Orientation::Downward,
            Orientation::Downward => Orientation::Leftward,
            Orientation::Leftward => Orientation::Upward,
        };

        if !self.is_invalid_position(field) {
            return;
        }

        // 回転不可能な場合
        self.ori = original_ori;
    }

    // TODO : delta[0]の場合はdefaultと同じなので関数を統合してもいいはず
    // TODO : left_rotate_with_srsと処理が重なる部分が多いので処理の共通化を検討
    pub fn right_rotate_with_srs(&mut self, field: &field::Field) {
        let original_y = self.y;
        let original_x = self.x;
        let original_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Rightward,
            Orientation::Rightward => Orientation::Downward,
            Orientation::Downward => Orientation::Leftward,
            Orientation::Leftward => Orientation::Upward,
        };

        // 参考: https://tetris.wiki/Super_Rotation_System
        // 参考とはyの正負が反転している
        // TODO: 型の判定などを用いてもっと直接的に判定したい
        let delta = if self.mino.get_size() == 4 {
            // Iミノの場合
            match original_ori {
                Orientation::Upward => [[0, 0], [-2, 0], [1, 0], [-2, -1], [1, 2]],
                Orientation::Rightward => [[0, 0], [-1, 0], [2, 0], [-1, 2], [2, -1]],
                Orientation::Downward => [[0, 0], [2, 0], [-1, 0], [2, 1], [-1, -2]],
                Orientation::Leftward => [[0, 0], [1, 0], [-2, 0], [1, -2], [-2, 1]],
            }
        } else {
            // Iミノ以外
            match original_ori {
                Orientation::Upward => [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
                Orientation::Rightward => [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]],
                Orientation::Downward => [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
                Orientation::Leftward => [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
            }
        };

        for d in delta.iter() {
            let dy = -d[1]; // 参考の正負の反転を補正
            let dx = d[0];
            let moved_y = self.y as i64 + dy;
            let moved_x = self.x as i64 + dx;

            self.y = moved_y;
            self.x = moved_x;
            let invalid_movement = self.is_invalid_position(field);
            self.y = original_y;
            self.x = original_x;

            if !invalid_movement {
                self.y = moved_y;
                self.x = moved_x;
                return;
            }
        }

        // 回転不可能な場合
        self.ori = original_ori;
    }

    pub fn left_rotate(&mut self, field: &field::Field) {
        let original_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Leftward,
            Orientation::Rightward => Orientation::Upward,
            Orientation::Downward => Orientation::Rightward,
            Orientation::Leftward => Orientation::Downward,
        };

        if !self.is_invalid_position(field) {
            return;
        }

        // 回転不可能な場合
        self.ori = original_ori;
    }

    // TODO : delta[0]の場合はdefaultと同じなので関数を統合してもいいはず
    pub fn left_rotate_with_srs(&mut self, field: &field::Field) {
        let original_y = self.y;
        let original_x = self.x;
        let original_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Leftward,
            Orientation::Rightward => Orientation::Upward,
            Orientation::Downward => Orientation::Rightward,
            Orientation::Leftward => Orientation::Downward,
        };

        // 参考: https://tetris.wiki/Super_Rotation_System
        // 参考とはyの正負が反転している
        // TODO: 型の判定などを用いてもっと直接的に判定したい
        let delta = if self.mino.get_size() == 4 {
            // Iミノの場合
            match original_ori {
                Orientation::Upward => [[0, 0], [-1, 0], [2, 0], [-1, 2], [2, -1]],
                Orientation::Rightward => [[0, 0], [2, 0], [-1, 0], [2, 1], [-1, -2]],
                Orientation::Downward => [[0, 0], [1, 0], [-2, 0], [1, -2], [-2, 1]],
                Orientation::Leftward => [[0, 0], [-2, 0], [1, 0], [-2, -1], [1, 2]],
            }
        } else {
            // Iミノ以外
            match original_ori {
                Orientation::Upward => [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
                Orientation::Rightward => [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]],
                Orientation::Downward => [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
                Orientation::Leftward => [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
            }
        };

        for d in delta.iter() {
            let dy = -d[1]; // 参考の正負の反転を補正
            let dx = d[0];
            let moved_y = self.y as i64 + dy;
            let moved_x = self.x as i64 + dx;

            self.y = moved_y;
            self.x = moved_x;
            let invalid_movement = self.is_invalid_position(field);
            self.y = original_y;
            self.x = original_x;

            if !invalid_movement {
                self.y = moved_y;
                self.x = moved_x;
                return;
            }
        }

        // 回転不可能な場合
        self.ori = original_ori;
    }

    // moveは予約語らしいので使えない
    // ミノを移動させる
    pub fn move_mino(&mut self, field: &field::Field, ori: Orientation) {
        let original_y = self.y;
        let original_x = self.x;
        let mut moved_y = self.get_y() as i64;
        let mut moved_x = self.get_x() as i64;

        match ori {
            Orientation::Upward => moved_y -= 1,
            Orientation::Rightward => moved_x += 1,
            Orientation::Downward => moved_y += 1,
            Orientation::Leftward => moved_x -= 1,
        }

        self.y = moved_y;
        self.x = moved_x;
        let invalid_movement = self.is_invalid_position(field);
        self.y = original_y;
        self.x = original_x;

        if !invalid_movement {
            self.y = moved_y;
            self.x = moved_x;
            match ori {
                Orientation::Downward => self.grounded = false,
                _ => {}
            }
        } else {
            match ori {
                Orientation::Downward => self.grounded = true,
                _ => {}
            }
        }
    }

    fn is_invalid_position(&self, field: &field::Field) -> bool {
        let rendered_mino = self.render();
        let mut invalid = false;
        for i in 0..self.mino.get_size() {
            for j in 0..self.mino.get_size() {
                if !rendered_mino[i][j] {
                    continue;
                }

                let y = self.y + i as i64;
                let x = self.x + j as i64;

                if y < 0 || y >= field.get_height() as i64 {
                    invalid = true;
                    break;
                }
                if x < 0 || x >= field.get_width() as i64 {
                    invalid = true;
                    break;
                }

                if field.get_block(y as usize, x as usize).filled {
                    invalid = true;
                    break;
                }
            }
        }
        invalid
    }
}

#[cfg(test)]
mod controlledmino_tests {
    use super::*;

    #[test]
    fn test_render() {
        struct TestCase {
            name: String,
            x: ControlledMino,
            want: Vec<Vec<bool>>,
        };

        let cases = vec![
            TestCase {
                name: "upward".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
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
                    ori: Orientation::Rightward,
                    grounded: false,
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
            field: Vec<Vec<bool>>,
            want: i32,
        };

        let cases = vec![
            TestCase {
                name: "valid upward".to_string(),
                x: Orientation::Upward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 1,
            },
            TestCase {
                name: "invalid upward".to_string(),
                x: Orientation::Upward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, true, false],
                ],
                want: 0,
            },
            TestCase {
                name: "invalid bordering upward".to_string(),
                x: Orientation::Upward,
                field: vec![vec![false, false, false], vec![false, false, false]],
                want: 0,
            },
            TestCase {
                name: "valid rightward".to_string(),
                x: Orientation::Rightward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 2,
            },
            TestCase {
                name: "invalid rightward".to_string(),
                x: Orientation::Rightward,
                field: vec![
                    vec![false, false, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
                want: 1,
            },
            TestCase {
                name: "valid downward".to_string(),
                x: Orientation::Downward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 3,
            },
            TestCase {
                name: "invalid downward".to_string(),
                x: Orientation::Downward,
                field: vec![
                    vec![false, true, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 2,
            },
            TestCase {
                name: "valid leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 0,
            },
            TestCase {
                name: "invalid leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, true],
                    vec![false, false, false],
                ],
                want: 3,
            },
            TestCase {
                name: "invalid bordering leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![vec![false, false], vec![false, false], vec![false, false]],
                want: 3,
            },
        ];

        let mut m = ControlledMino {
            x: 0,
            y: 0,
            mino: Box::new(mino::TMino::default()),
            ori: Orientation::Upward,
            grounded: false,
        };
        for case in cases {
            let height = case.field.len();
            let width = case.field[0].len();
            let mut f = field::Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.set_block_filled(i, j, case.field[i][j]);
                }
            }
            m.ori = case.x;
            m.right_rotate(&mut f);
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
            field: Vec<Vec<bool>>,
            want: i32,
        };

        let cases = vec![
            TestCase {
                name: "valid upward".to_string(),
                x: Orientation::Upward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 3,
            },
            TestCase {
                name: "invalid upward".to_string(),
                x: Orientation::Upward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, true, false],
                ],
                want: 0,
            },
            TestCase {
                name: "invalid bordering upward".to_string(),
                x: Orientation::Upward,
                field: vec![vec![false, false, false], vec![false, false, false]],
                want: 0,
            },
            TestCase {
                name: "valid rightward".to_string(),
                x: Orientation::Rightward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 0,
            },
            TestCase {
                name: "invalid rightward".to_string(),
                x: Orientation::Rightward,
                field: vec![
                    vec![false, false, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
                want: 1,
            },
            TestCase {
                name: "valid downward".to_string(),
                x: Orientation::Downward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 1,
            },
            TestCase {
                name: "invalid downward".to_string(),
                x: Orientation::Downward,
                field: vec![
                    vec![false, true, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 2,
            },
            TestCase {
                name: "valid leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, false, false],
                ],
                want: 2,
            },
            TestCase {
                name: "invalid leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![
                    vec![false, false, false],
                    vec![false, false, true],
                    vec![false, false, false],
                ],
                want: 3,
            },
            TestCase {
                name: "invalid bordering leftward".to_string(),
                x: Orientation::Leftward,
                field: vec![vec![false, false], vec![false, false], vec![false, false]],
                want: 3,
            },
        ];

        let mut m = ControlledMino {
            x: 0,
            y: 0,
            mino: Box::new(mino::TMino::default()),
            ori: Orientation::Upward,
            grounded: false,
        };
        for case in cases {
            let height = case.field.len();
            let width = case.field[0].len();
            let mut f = field::Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.set_block_filled(i, j, case.field[i][j]);
                }
            }
            m.ori = case.x;
            m.left_rotate(&mut f);
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
    fn test_right_rotate_with_srs() {
        struct TestCase {
            name: String,
            x: ControlledMino,
            field: Vec<Vec<bool>>,
            want: (i64, i64, Orientation), // (x, y, ori)
        };

        let mut cases = vec![
            TestCase {
                name: "TMino is pointing left and pattern 0".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                field: vec![
                    vec![true, false, false],
                    vec![false, false, false],
                    vec![true, false, true],
                ],
                want: (0, 0, Orientation::Downward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 1".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![false, true, true],
                ],
                want: (-1, 0, Orientation::Rightward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 2".to_string(),
                x: ControlledMino {
                    x: 1,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Leftward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, false],
                    vec![true, false, false],
                    vec![false, false, false],
                ],
                want: (0, 1, Orientation::Upward),
            },
            TestCase {
                name: "TMino is pointing down and pattern 3".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Downward,
                    grounded: false,
                },
                field: vec![
                    vec![false, true, true],
                    vec![false, false, false],
                    vec![true, false, true],
                    vec![false, false, true],
                    vec![false, false, true],
                ],
                want: (0, 2, Orientation::Leftward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 4".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                field: vec![
                    vec![true, false, false],
                    vec![false, false, false],
                    vec![false, true, true],
                    vec![false, false, true],
                    vec![false, true, true],
                ],
                want: (-1, 2, Orientation::Rightward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 1".to_string(),
                x: ControlledMino {
                    x: 1,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, true, false, true],
                    vec![false, false, true, false, true],
                    vec![false, false, false, false, true],
                    vec![false, false, true, false, true],
                ],
                want: (0, 0, Orientation::Downward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 2".to_string(),
                x: ControlledMino {
                    x: -2,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                field: vec![
                    vec![false, true, true, true],
                    vec![false, true, true, true],
                    vec![false, false, false, false],
                    vec![false, true, true, true],
                ],
                want: (0, 0, Orientation::Downward),
            },
            TestCase {
                name: "IMino is pointing up and pattern 3".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 1,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                field: vec![
                    vec![true, false, false, false],
                    vec![false, false, false, false],
                    vec![false, true, true, true],
                    vec![false, true, true, true],
                    vec![false, true, true, true],
                ],
                want: (-2, 1, Orientation::Rightward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 4".to_string(),
                x: ControlledMino {
                    x: -2,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                field: vec![
                    vec![false, true, true, true],
                    vec![false, true, true, true],
                    vec![false, true, true, true],
                    vec![false, false, false, false],
                ],
                want: (0, 1, Orientation::Downward),
            },
        ];

        for case in &mut cases {
            let height = case.field.len();
            let width = case.field[0].len();
            let mut f = field::Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.set_block_filled(i, j, case.field[i][j]);
                }
            }

            case.x.right_rotate_with_srs(&mut f);

            assert_eq!(
                (case.x.x, case.x.y, case.x.ori),
                case.want,
                "case {}: failed",
                case.name
            );
        }
    }

    #[test]
    fn test_left_rotate_with_srs() {
        struct TestCase {
            name: String,
            x: ControlledMino,
            field: Vec<Vec<bool>>,
            want: (i64, i64, Orientation), // (x, y, ori)
        };

        let mut cases = vec![
            TestCase {
                name: "TMino is pointing left and pattern 0".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Leftward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, true],
                    vec![false, false, false],
                    vec![true, false, true],
                ],
                want: (0, 0, Orientation::Downward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 1".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, false],
                    vec![false, false, false],
                    vec![true, true, false],
                ],
                want: (1, 0, Orientation::Leftward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 2".to_string(),
                x: ControlledMino {
                    x: -1,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, false],
                    vec![false, false, true],
                    vec![false, false, false],
                ],
                want: (0, 1, Orientation::Upward),
            },
            TestCase {
                name: "TMino is pointing down and pattern 3".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Downward,
                    grounded: false,
                },
                field: vec![
                    vec![false, true, true],
                    vec![false, false, false],
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![true, false, false],
                ],
                want: (0, 2, Orientation::Rightward),
            },
            TestCase {
                name: "TMino is pointing up and pattern 4".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, true],
                    vec![false, false, false],
                    vec![true, true, false],
                    vec![true, false, false],
                    vec![true, true, false],
                ],
                want: (1, 2, Orientation::Leftward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 1".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Leftward,
                    grounded: false,
                },
                field: vec![
                    vec![true, false, true, true, true],
                    vec![true, false, true, true, true],
                    vec![true, false, false, false, false],
                    vec![true, false, true, true, true],
                ],
                want: (1, 0, Orientation::Downward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 2".to_string(),
                x: ControlledMino {
                    x: 2,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Leftward,
                    grounded: false,
                },
                field: vec![
                    vec![true, true, true, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                ],
                want: (0, 0, Orientation::Downward),
            },
            TestCase {
                name: "IMino is pointing up and pattern 3".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: -1,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Downward,
                    grounded: false,
                },
                field: vec![
                    vec![false, false, false, true],
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![true, true, true, false],
                    vec![true, true, true, false],
                ],
                want: (1, 1, Orientation::Rightward),
            },
            TestCase {
                name: "IMino is pointing right and pattern 4".to_string(),
                x: ControlledMino {
                    x: 2,
                    y: 0,
                    mino: Box::new(mino::IMino::default()),
                    ori: Orientation::Leftward,
                    grounded: false,
                },
                field: vec![
                    vec![true, true, true, false],
                    vec![true, true, true, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                ],
                want: (0, 1, Orientation::Downward),
            },
        ];

        for case in &mut cases {
            let height = case.field.len();
            let width = case.field[0].len();
            let mut f = field::Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.set_block_filled(i, j, case.field[i][j]);
                }
            }

            case.x.left_rotate_with_srs(&mut f);

            assert_eq!(
                (case.x.x, case.x.y, case.x.ori),
                case.want,
                "case {}: failed",
                case.name
            );
        }
    }

    #[test]
    fn test_move() {
        struct TestCase {
            name: String,
            x: ControlledMino,
            move_ori: Orientation,
            want: (i64, i64, bool),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
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
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Downward,
                want: (0, 3, true),
            },
            TestCase {
                name: "フィールド境界のため落下不可能".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 3,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Downward,
                want: (0, 3, true),
            },
            TestCase {
                name: "filled=falseの部分がフィールド外にはみ出す".to_string(),
                x: ControlledMino {
                    x: 0,
                    y: 0,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Rightward,
                    grounded: false,
                },
                move_ori: Orientation::Leftward,
                want: (-1, 0, false),
            },
            TestCase {
                name: "おじゃまブロックの生成で重なったときに上に移動".to_string(),
                x: ControlledMino {
                    x: 1,
                    y: 2,
                    mino: Box::new(mino::TMino::default()),
                    ori: Orientation::Upward,
                    grounded: false,
                },
                move_ori: Orientation::Upward,
                want: (1, 1, false),
            },
        ];

        let mut f = field::Field::new(field_height, field_width);
        for h in 0..f.get_height() {
            for w in 0..f.get_width() {
                f.set_block_filled(h, w, field_filled[h][w]);
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
