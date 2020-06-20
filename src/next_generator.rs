/// nextを生成する
// 一応インタフェース化はするつもりだが戦略などが変化することもないはずなので必要ないかも
use crate::mino;
use rand::prelude::*;
use std::collections::VecDeque;

pub trait NextGenerator {
    fn next(&mut self) -> Box<dyn mino::Mino>;
    fn get_next(&self, idx: usize) -> Option<&Box<dyn mino::Mino>>;
}

/// 7種類を１セットとして生成する
pub struct DefaultNextGenerator {
    // TODO; traitのvecを作成する方法は二つある片方は簡潔だが遅い，もう片方は複雑だが速い
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

    ///  bufferの中身が0の場合に実行
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
    // TODO:
    // 現在の実装ではbufferの中身が0になってから生成しているのでnextとして画面に表示できない．
    fn next(&mut self) -> Box<dyn mino::Mino> {
        if self.buffer.len() <= 7 {
            self.generate();
        }
        self.buffer.pop_front().unwrap()
    }

    // TODO: idx = 0が次に取り出すminoとする
    // 内部的にはVec(stack)で実装しているので少し変なことになっている
    // queueで実装したほうが自然かも？
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

    // TODO: nextの生成が7個1セットでできているかもテストしたい
    // rustは型の同一性判定が難しいらしいのでうまい実装ができなかった
    // 現在思いついているあまり良くない方法(testのためだけに実装することになってしまう)
    // 1: 型判定用のfieldをstructに追加
    // 2: 同一性判定のためのtraitを実装
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

        // ミノは同じ数になるように生成しているのでそれをテスト
        // 構造体の種類を直接判定する方法が見つからなかったのでshapeをbitにみたてて同一性を判定
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
