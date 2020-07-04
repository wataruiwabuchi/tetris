/// nextを生成する
// 一応インタフェース化はするつもりだが戦略などが変化することもないはずなので必要ないかも
use crate::mino;
use std::collections::VecDeque;

pub trait NextGenerator {
    fn next(&mut self) -> Box<dyn mino::Mino>;
    fn get_next(&self, idx: usize) -> Option<&Box<dyn mino::Mino>>;
}

/// 7種類を１セットとして生成する
pub struct DefaultNextGenerator {
    // https://doc.rust-jp.rs/book/second-edition/ch17-02-trait-objects.html
    buffer: VecDeque<Box<dyn mino::Mino>>,
    rand_gen: Box<dyn FnMut() -> usize>,
}

impl DefaultNextGenerator {
    pub fn new(rand_gen: Box<dyn FnMut() -> usize>) -> DefaultNextGenerator {
        DefaultNextGenerator {
            buffer: VecDeque::new(),
            rand_gen: rand_gen,
        }
    }

    /// 7個1セットのミノを生成してnextのbufferに詰める
    fn generate(&mut self) {
        let num_mino_type = 7;
        let mut indices: Vec<usize> = (0..num_mino_type).collect();
        for _ in 0..num_mino_type {
            let i1 = (self.rand_gen)() % num_mino_type;
            let i2 = (self.rand_gen)() % num_mino_type;
            let tmp = indices[i1];
            indices[i1] = indices[i2];
            indices[i2] = tmp;
        }

        for i in indices {
            match i {
                0 => {
                    self.buffer.push_back(Box::new(mino::TMino::default()));
                }
                1 => {
                    self.buffer.push_back(Box::new(mino::SMino::default()));
                }
                2 => {
                    self.buffer.push_back(Box::new(mino::ZMino::default()));
                }
                3 => {
                    self.buffer.push_back(Box::new(mino::LMino::default()));
                }
                4 => {
                    self.buffer.push_back(Box::new(mino::JMino::default()));
                }
                5 => {
                    self.buffer.push_back(Box::new(mino::IMino::default()));
                }
                6 => {
                    self.buffer.push_back(Box::new(mino::OMino::default()));
                }
                _ => {}
            }
        }
    }
}

impl NextGenerator for DefaultNextGenerator {
    /// 次のミノを取得する
    /// bufferからは取り除かれる
    fn next(&mut self) -> Box<dyn mino::Mino> {
        if self.buffer.len() <= 7 {
            self.generate();
        }
        self.buffer.pop_front().unwrap()
    }

    /// nextミノのbufferへの参照を取得する
    /// idx=0が次のnext
    /// bufferからは取り除かれない
    /// nextを画面にrenderingするために作成した
    fn get_next(&self, idx: usize) -> Option<&Box<dyn mino::Mino>> {
        if self.buffer.len() > idx {
            Some(&self.buffer[idx])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod defaultnextgenerator_tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<usize>());
        let mut nx = DefaultNextGenerator {
            buffer: VecDeque::new(),
            rand_gen: rand_gen,
        };

        for _ in 0..10 {
            nx.generate();
        }
    }

    #[test]
    fn test_get_next() {
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<usize>());
        let mut nx = DefaultNextGenerator {
            buffer: VecDeque::new(),
            rand_gen: rand_gen,
        };

        nx.generate();
        let mut count_some = 0;
        let mut count_none = 0;
        for i in 0..10 {
            match nx.get_next(i) {
                Some(_) => {
                    count_some += 1;
                }
                None => {
                    count_none += 1;
                }
            }
        }

        assert!(!(count_some == 0 || count_none == 0));
    }

    #[test]
    fn test_next() {
        use std::collections::HashMap;

        let num_iter = 100;
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<usize>());
        let mut ng = DefaultNextGenerator {
            buffer: VecDeque::new(),
            rand_gen: rand_gen,
        };

        // ミノは7個1セットで生成しているのでテスト
        // 構造体の種類を直接判定する方法が見つからなかった
        // shapeをbitにみたてて同一性を判定
        let mut count_next_mino = HashMap::new();
        for _ in 0..7 * num_iter {
            let next_mino = ng.next();
            let mut key = 0;
            for i in 0..next_mino.get_size() {
                for j in 0..next_mino.get_size() {
                    if next_mino.get_shape()[i][j] {
                        key += 1 << i * next_mino.get_size() + j;
                    }
                }
            }
            let count = count_next_mino.entry(key).or_insert(0);
            *count += 1;
        }

        for v in count_next_mino.values() {
            assert_eq!(*v, num_iter)
        }
    }
}
