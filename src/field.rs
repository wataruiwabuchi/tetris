/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定
use crate::mino;

// フィールドの各ブロック
struct FieldBlock {
    filled: bool,    // ブロックにミノが存在するか
    color: [f32; 4], // ブロックの色
}

// テトリスのフィールド
struct Field {
    height: usize,
    width: usize,
    blocks: Vec<Vec<FieldBlock>>,
}

impl Field {
    /// Fieldのコンストラクタ
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

    // 横列ごとにminoが揃っているかを判定し揃っている列のインデクスを返す
    // 削除したかという情報と削除した列の情報を返す
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

struct ControlledMino {
    x: usize,
    y: usize,
    mino: mino::Mino,
    ori: Orientation,
}

#[cfg(test)]
mod tests {
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
}
