// TODO : メンバ変数を参照する関数の処理を共通化する方法を探す
// 現状ではget_sizeなどの全く同じ動作を行う関数をすべてのミノに対して実装している
// traitのデフォルト実装でこの部分を共通化できれば良いがtraitからはメンバ変数にアクセスできないのでその部分に実装するとエラーが出る

pub trait Mino {
    fn new() -> Self;
    fn get_size(&self) -> usize;
    fn get_shape(&self) -> &Vec<Vec<bool>>;
    fn get_color(&self) -> [f32; 4];
}

pub struct TMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Mino for TMino {
    fn new() -> Self {
        TMino {
            size: 3,
            shape: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, false, false],
            ],
            color: [0.5, 0.0, 0.5, 1.0],
        }
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_shape(&self) -> &Vec<Vec<bool>> {
        &self.shape
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
