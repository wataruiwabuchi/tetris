// TODO : メンバ変数を参照する関数の処理を共通化する方法を探す
// 現状ではget_heightなどの全く同じ動作を行う関数をすべてのミノに対して実装している
// traitのデフォルト実装でこの部分を共通化できれば良いがtraitからはメンバ変数にアクセスできないのでその部分に実装するとエラーが出る

pub struct Mino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Mino {
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn get_shape(&self) -> Vec<Vec<bool>> {
        // TODO: readonlyな参照を返すだけでもいいかも？(そうすると参照が返ってこないか？)
        let mut r = vec![vec![false; self.get_size()]; self.get_size()];
        for i in 0..self.get_size() {
            for j in 0..self.get_size() {
                r[i][j] = self.shape[i][j];
            }
        }
        r
    }
    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}
