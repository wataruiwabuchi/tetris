// TODO : メンバ変数を参照する関数の処理を共通化する方法を探す
// 現状ではget_sizeなどの全く同じ動作を行う関数をすべてのミノに対して実装している
// traitのデフォルト実装でこの部分を共通化できれば良いがtraitからはメンバ変数にアクセスできないのでその部分に実装するとエラーが出る

pub trait Mino {
    fn get_size(&self) -> usize;
    fn get_shape(&self) -> &Vec<Vec<bool>>;
    fn get_color(&self) -> [f32; 4];
}

pub struct TMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for TMino {
    fn default() -> Self {
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
}

impl Mino for TMino {
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

pub struct SMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for SMino {
    fn default() -> Self {
        SMino {
            size: 3,
            shape: vec![
                vec![false, true, true],
                vec![true, true, false],
                vec![false, false, false],
            ],
            color: [0.0, 1.0, 0.0, 1.0],
        }
    }
}

impl Mino for SMino {
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

pub struct ZMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for ZMino {
    fn default() -> Self {
        ZMino {
            size: 3,
            shape: vec![
                vec![true, true, false],
                vec![false, true, true],
                vec![false, false, false],
            ],
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Mino for ZMino {
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

pub struct LMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for LMino {
    fn default() -> Self {
        LMino {
            size: 3,
            shape: vec![
                vec![false, false, true],
                vec![true, true, true],
                vec![false, false, false],
            ],
            color: [1.0, 0.65, 0.0, 1.0],
        }
    }
}

impl Mino for LMino {
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

pub struct JMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for JMino {
    fn default() -> Self {
        JMino {
            size: 3,
            shape: vec![
                vec![true, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ],
            color: [0.0, 0.0, 1.0, 1.0],
        }
    }
}

impl Mino for JMino {
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

pub struct IMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for IMino {
    fn default() -> Self {
        IMino {
            size: 4,
            shape: vec![
                vec![false, false, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
            ],
            color: [0.33, 0.73, 0.83, 1.0],
        }
    }
}

impl Mino for IMino {
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

pub struct OMino {
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Default for OMino {
    fn default() -> Self {
        OMino {
            size: 2,
            shape: vec![vec![true, true], vec![true, true]],
            color: [0.98, 0.82, 0.11, 1.0],
        }
    }
}

impl Mino for OMino {
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
