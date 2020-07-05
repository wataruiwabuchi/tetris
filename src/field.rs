use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter::FromIterator;
/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定

// フィールドの各ブロック
#[derive(Clone)]
pub struct FieldBlock {
    pub filled: bool,    // ブロックにミノが存在するか
    pub color: [f32; 4], // ブロックの色
}

// テトリスのフィールド
#[derive(Clone)]
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

    pub fn get_block(&self, row: usize, col: usize) -> &FieldBlock {
        &self.blocks[row][col]
    }

    pub fn set_block_filled(&mut self, row: usize, col: usize, filled: bool) {
        self.blocks[row][col].filled = filled;
    }

    pub fn set_block_color(&mut self, row: usize, col: usize, color: [f32; 4]) {
        self.blocks[row][col].color = color;
    }

    /// 横列ごとにminoが揃っているかを判定し揃っている列のインデクスを返す
    /// アニメーション処理などが入ることを考慮して実際に消す処理とは分離してある
    pub fn is_filled_each_row(&self) -> Option<Vec<usize>> {
        // 一列埋まっている列のインデックスのVecを取得
        let filled_row_ids: Vec<usize> = self
            .blocks
            .iter()
            .map(|x| x.iter().all(|x| x.filled))
            .enumerate()
            .filter(|x| x.1)
            .map(|x| x.0)
            .collect();

        if filled_row_ids.len() == 0 {
            return None;
        }

        return Some(filled_row_ids);
    }

    /// 指定されたインデックスのlineを削除
    pub fn delete_lines(&mut self, deleted_ids: Vec<usize>) {
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
    fn test_clone() {
        let f1 = Field::new(5, 4);
        let _f2 = f1.clone();
    }

    #[test]
    fn test_set_block() {
        let mut f = Field::new(5, 4);
        let c: [f32; 4] = [0.0; 4];
        f.set_block_filled(0, 0, true);
        f.set_block_color(0, 0, c);
        f.set_block_filled(0, 0, f.get_block(0, 0).filled);
        f.set_block_color(0, 0, f.get_block(0, 0).color);
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
