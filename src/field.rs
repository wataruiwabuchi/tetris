/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定

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
}
