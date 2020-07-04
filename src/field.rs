use crate::mino;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;
/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定

// フィールドの各ブロック
pub struct FieldBlock {
    // TODO: filledも本来はprivateにしたほうがいい？
    // 自由にミノを配置したりを考えるとpubのほうが使いやすい？
    pub filled: bool,    // ブロックにミノが存在するか
    pub color: [f32; 4], // ブロックの色
}

// テトリスのフィールド
pub struct Field {
    height: usize,
    width: usize,
    blocks: VecDeque<Vec<FieldBlock>>,
}

impl Field {
    /// Fieldのコンストラクタ
    pub fn new(height: usize, width: usize) -> Field {
        let mut blocks: VecDeque<Vec<FieldBlock>> = VecDeque::new();
        for _ in 0..height {
            let mut tmp_vec: Vec<FieldBlock> = Vec::new();
            for _ in 0..width {
                tmp_vec.push(FieldBlock {
                    filled: false,
                    color: [0 as f32; 4],
                });
            }
            blocks.push_back(tmp_vec);
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

    // TODO: set関数の実装
    // 現状だと参照しかするつもりのない関数から呼ぶ場合もmutにしなければならない
    // set関数を実装しgetからmutをなくせば，getしか呼び出さない関数にはmutをつけない状態で渡しておくことができる
    // こちらのほうが適切なアクセス管理ができていると考えられる
    // TODO: 返り値をoptionに変更
    // index out of
    // memoryを防ぐための境界判定の処理を一か所にまとめていたほうが実装忘れによるエラーを防止できる気がする
    pub fn get_block(&mut self, row: usize, col: usize) -> &mut FieldBlock {
        &mut self.blocks[row][col]
    }

    /// 横列ごとにminoが揃っているかを判定し揃っている列のインデクスを返す
    /// アニメーション処理などが入ることを考慮して実際に消す処理とは分離してある
    // TODO : map, all, anyあたりを使うともっと簡潔に書けるらしいので修正
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

    /// 指定されたインデックスのlineを削除
    pub fn delete_lines(&mut self, deleted_ids: Vec<usize>) {
        // TODO: 実装に納得がいっていない，もっときれいな実装はないか
        let set_deleted_ids: HashSet<_> = deleted_ids.iter().copied().collect();
        let ids: HashSet<_> = (0..self.height).collect();
        let mut left_ids: Vec<_> = Vec::from_iter(ids.difference(&set_deleted_ids)); // 残すlineのindices

        left_ids.sort_by_key(|v| Reverse(*v));

        // 消すlineを詰めながらblockの中身をコピー
        let mut cur_line = (self.height - 1) as i32;
        for i in left_ids.iter() {
            println!("{}", **i);
            // フィールドの下部からコピー
            for j in 0..self.width {
                self.blocks[cur_line as usize][j].filled = self.blocks[**i][j].filled;
                self.blocks[cur_line as usize][j].color[0] = self.blocks[**i][j].color[0];
                self.blocks[cur_line as usize][j].color[1] = self.blocks[**i][j].color[1];
                self.blocks[cur_line as usize][j].color[2] = self.blocks[**i][j].color[2];
                self.blocks[cur_line as usize][j].color[3] = self.blocks[**i][j].color[3];
            }
            cur_line -= 1;
        }

        // 消したことによって空きができたフィールドの上部に空のblockを配置
        if cur_line > 0 {
            for i in 0..cur_line as usize + 1 {
                println!("{} test", i);
                for j in 0..self.width {
                    self.blocks[i][j].filled = false;
                    self.blocks[i][j].color[0] = 0.0;
                    self.blocks[i][j].color[1] = 0.0;
                    self.blocks[i][j].color[2] = 0.0;
                    self.blocks[i][j].color[3] = 0.0;
                }
            }
        }
    }

    /// fieldにlineを挿入する関数
    /// 主におじゃまブロックの生成時に使用することを想定している
    /// 挿入後にフィールドの上部にはみ出すブロックが存在する場合はErrを返す
    pub fn insert_lines(
        &mut self,
        inserted_lines: Vec<Vec<FieldBlock>>,
    ) -> Result<&'static str, &'static str> {
        for i in 0..inserted_lines.len() {
            if self.blocks[i]
                .iter()
                .fold(0, |acc, x| acc + x.filled as usize)
                > 0
            {
                return Err("挿入不可能");
            }
        }

        for inserted_line in inserted_lines {
            self.blocks.pop_front();
            self.blocks.push_back(inserted_line);
        }

        return Ok("Success");
    }
}

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

    // TODO : fieldのblocksに対するset関数を実装してここのfieldのmutをなくす
    pub fn right_rotate(&mut self, field: &mut Field) {
        let before_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Rightward,
            Orientation::Rightward => Orientation::Downward,
            Orientation::Downward => Orientation::Leftward,
            Orientation::Leftward => Orientation::Upward,
        };

        let rendered_mino = self.render();
        let mut invalid_movement = false;
        let y = self.y as i64;
        let x = self.x as i64;
        for i in 0..self.mino.get_size() {
            for j in 0..self.mino.get_size() {
                if !rendered_mino[i][j] {
                    continue;
                }

                if (y + i as i64) < 0 || (y + i as i64) >= field.get_height() as i64 {
                    invalid_movement = true;
                    break;
                }
                if (x + j as i64) < 0 || (x + j as i64) >= field.get_width() as i64 {
                    invalid_movement = true;
                    break;
                }

                if field.get_block(y as usize + i, x as usize + j).filled {
                    invalid_movement = true;
                    break;
                }
            }
        }

        if !invalid_movement {
            return;
        }

        // 回転不可能な場合
        self.ori = before_ori;
    }

    // TODO : fieldのblocksに対するset関数を実装してここのfieldのmutをなくす
    // TODO : delta[0]の場合はdefaultと同じなので関数を統合してもいいはず
    // TODO : left_rotate_with_srsと処理が重なる部分が多いので処理の共通化を検討
    pub fn right_rotate_with_srs(&mut self, field: &mut Field) {
        let before_ori = self.ori;

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
            match before_ori {
                Orientation::Upward => [[0, 0], [-2, 0], [1, 0], [-2, -1], [1, 2]],
                Orientation::Rightward => [[0, 0], [-1, 0], [2, 0], [-1, 2], [2, -1]],
                Orientation::Downward => [[0, 0], [2, 0], [-1, 0], [2, 1], [-1, -2]],
                Orientation::Leftward => [[0, 0], [1, 0], [-2, 0], [1, -2], [-2, 1]],
            }
        } else {
            // Iミノ以外
            match before_ori {
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

            let rendered_mino = self.render();
            let mut invalid_movement = false;
            for i in 0..self.mino.get_size() {
                for j in 0..self.mino.get_size() {
                    if !rendered_mino[i][j] {
                        continue;
                    }

                    let y = moved_y + i as i64;
                    let x = moved_x + j as i64;

                    if y < 0 || y >= field.get_height() as i64 {
                        invalid_movement = true;
                        break;
                    }
                    if x < 0 || x >= field.get_width() as i64 {
                        invalid_movement = true;
                        break;
                    }

                    if field.get_block(y as usize, x as usize).filled {
                        invalid_movement = true;
                        break;
                    }
                }
            }

            if !invalid_movement {
                self.y = moved_y;
                self.x = moved_x;
                return;
            }
        }

        // 回転不可能な場合
        self.ori = before_ori;
    }

    // TODO : fieldのblocksに対するset関数を実装してここのfieldのmutをなくす
    pub fn left_rotate(&mut self, field: &mut Field) {
        let before_ori = self.ori;

        self.ori = match &self.ori {
            Orientation::Upward => Orientation::Leftward,
            Orientation::Rightward => Orientation::Upward,
            Orientation::Downward => Orientation::Rightward,
            Orientation::Leftward => Orientation::Downward,
        };

        let rendered_mino = self.render();
        let mut invalid_movement = false;
        let y = self.y as i64;
        let x = self.x as i64;
        for i in 0..self.mino.get_size() {
            for j in 0..self.mino.get_size() {
                if !rendered_mino[i][j] {
                    continue;
                }

                if (y + i as i64) < 0 || (y + i as i64) >= field.get_height() as i64 {
                    invalid_movement = true;
                    break;
                }
                if (x + j as i64) < 0 || (x + j as i64) >= field.get_width() as i64 {
                    invalid_movement = true;
                    break;
                }

                if field.get_block(y as usize + i, x as usize + j).filled {
                    invalid_movement = true;
                    break;
                }
            }
        }

        if !invalid_movement {
            return;
        }

        // 回転不可能な場合
        self.ori = before_ori;
    }

    // TODO : fieldのblocksに対するset関数を実装してここのfieldのmutをなくす
    // TODO : delta[0]の場合はdefaultと同じなので関数を統合してもいいはず
    pub fn left_rotate_with_srs(&mut self, field: &mut Field) {
        let before_ori = self.ori;

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
            match before_ori {
                Orientation::Upward => [[0, 0], [-1, 0], [2, 0], [-1, 2], [2, -1]],
                Orientation::Rightward => [[0, 0], [2, 0], [-1, 0], [2, 1], [-1, -2]],
                Orientation::Downward => [[0, 0], [1, 0], [-2, 0], [1, -2], [-2, 1]],
                Orientation::Leftward => [[0, 0], [-2, 0], [1, 0], [-2, -1], [1, 2]],
            }
        } else {
            // Iミノ以外
            match before_ori {
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

            let rendered_mino = self.render();
            let mut invalid_movement = false;
            for i in 0..self.mino.get_size() {
                for j in 0..self.mino.get_size() {
                    if !rendered_mino[i][j] {
                        continue;
                    }

                    let y = moved_y + i as i64;
                    let x = moved_x + j as i64;

                    if y < 0 || y >= field.get_height() as i64 {
                        invalid_movement = true;
                        break;
                    }
                    if x < 0 || x >= field.get_width() as i64 {
                        invalid_movement = true;
                        break;
                    }

                    if field.get_block(y as usize, x as usize).filled {
                        invalid_movement = true;
                        break;
                    }
                }
            }

            if !invalid_movement {
                self.y = moved_y;
                self.x = moved_x;
                return;
            }
        }

        // 回転不可能な場合
        self.ori = before_ori;
    }

    // moveは予約語らしいので使えない
    // ミノを移動させる
    pub fn move_mino(&mut self, field: &Field, ori: Orientation) {
        let size = self.mino.get_size();
        let rendered_mino = self.render();
        let mut moved_x = self.get_x() as i64;
        let mut moved_y = self.get_y() as i64;

        match ori {
            Orientation::Upward => moved_y -= 1,
            Orientation::Rightward => moved_x += 1,
            Orientation::Downward => moved_y += 1,
            Orientation::Leftward => moved_x -= 1,
        }

        let mut movable = true;
        for i in 0..size {
            for j in 0..size {
                if !rendered_mino[i][j] {
                    continue;
                }

                let x_in_field = j as i64 + moved_x;
                let y_in_field = i as i64 + moved_y;

                // フィールドの境界チェック
                // 移動先のブロックが埋まっていないかをチェック
                if x_in_field < 0
                    || x_in_field >= field.get_width() as i64
                    || y_in_field < 0
                    || y_in_field >= field.get_height() as i64
                    || field.blocks[y_in_field as usize][x_in_field as usize].filled
                {
                    movable = false;
                    break;
                }
            }
        }

        if movable {
            self.x = moved_x;
            self.y = moved_y;
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
}

#[cfg(test)]
mod field_tests {
    use super::*;

    #[test]
    fn test_new() {
        // blockがすべて埋まっていないかをテスト
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
                // 一部が埋まっている
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
            // blockがすべて埋まっていないかをテスト
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

    #[test]
    fn test_delete_lines() {
        // TODO: 色に関するテストも追加
        let test_height = 5;
        let test_width = 4;

        let input_field = vec![
            vec![true, true, true, true],
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![true, true, true, true],
            vec![true, false, false, false],
        ];

        struct TestCase {
            name: String,
            x: Vec<usize>,
            want: Vec<Vec<bool>>,
        };

        let cases = vec![
            TestCase {
                name: "delete all".to_string(),
                x: (0..test_height).collect(),
                want: vec![vec![false; test_width]; test_height],
            },
            TestCase {
                name: "delete all with shuffled indices".to_string(),
                x: vec![3, 2, 0, 4, 1],
                want: vec![vec![false; test_width]; test_height],
            },
            TestCase {
                name: "not delete".to_string(),
                x: vec![],
                want: vec![
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, true, true, false],
                    vec![true, true, true, true],
                    vec![true, false, false, false],
                ],
            },
            TestCase {
                name: "hand craft1".to_string(),
                x: vec![0, 3],
                want: vec![
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![false, true, true, false],
                    vec![true, false, false, false],
                ],
            },
            TestCase {
                name: "hand craft2".to_string(),
                x: vec![4, 3],
                want: vec![
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, true, true, false],
                ],
            },
        ];

        for case in cases {
            let mut f = Field::new(test_height, test_width);
            for h in 0..f.get_height() {
                for w in 0..f.get_width() {
                    f.blocks[h][w].filled = input_field[h][w];
                }
            }

            f.delete_lines(case.x);

            let mut y = vec![vec![false; test_width]; test_height];
            for h in 0..f.get_height() {
                for w in 0..f.get_width() {
                    y[h][w] = f.blocks[h][w].filled;
                }
            }
            assert_eq!(y, case.want, "case {}: failed", case.name)
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
            let mut f = Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.get_block(i, j).filled = case.field[i][j];
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
            let mut f = Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.get_block(i, j).filled = case.field[i][j];
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
            let mut f = Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.get_block(i, j).filled = case.field[i][j];
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
            let mut f = Field::new(height, width);
            for i in 0..height {
                for j in 0..width {
                    f.get_block(i, j).filled = case.field[i][j];
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
