/// 21×10のテトリスのフィールドを表現
/// controllerからstepが呼び出されそのたびに落下処理や削除処理を行う予定

// フィールドの各ブロック
struct FieldBlock {
    filled: bool,    // ブロックにミノが存在するか
    color: [f32; 4], // ブロックの色
}

// テトリスのフィールド
struct Field {
    blocks: Vec<Vec<FieldBlock>>,
}

impl Field {
    const HEIGHT: usize = 21;
    const WIDTH: usize = 10;
}

impl Field {
    /// Fieldのコンストラクタ
    pub fn new() -> Field {
        let mut blocks: Vec<Vec<FieldBlock>> = Vec::new();
        for _ in 0..Field::HEIGHT {
            let mut tmp_vec: Vec<FieldBlock> = Vec::new();
            for _ in 0..Field::WIDTH {
                tmp_vec.push(FieldBlock {
                    filled: false,
                    color: [0 as f32; 4],
                });
            }
            blocks.push(tmp_vec);
        }
        Field { blocks: blocks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        assert_eq!(21, Field::HEIGHT);
        assert_eq!(10, Field::WIDTH);
    }
}
