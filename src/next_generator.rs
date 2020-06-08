/// nextを生成する
// 一応インタフェース化はするつもりだが戦略などが変化することもないはずなので必要ないかも
use crate::mino;
use rand::prelude::*;

pub trait NextGenerator {
    fn next(&mut self) -> Box<dyn mino::Mino>;
}

/// 7種類を１セットとして生成する
pub struct DefaultNextGenerator {
    // TODO; traitのvecを作成する方法は二つある片方は簡潔だが遅い，もう片方は複雑だが速い
    // https://doc.rust-jp.rs/book/second-edition/ch17-02-trait-objects.html
    pub buffer: Vec<Box<dyn mino::Mino>>,
    pub rand_gen: Box<dyn FnMut() -> usize>,
}

impl DefaultNextGenerator {
    ///  bufferの中身が0の場合に実行
    /// 7個1セットのミノを生成してnextのbufferに詰める
    fn generate(&mut self) {
        // TODO: ランダムに生成する
        // TODO: TMino以外も生成する
        // let rand_value = (self.rand_gen)() % self.field.get_height();
        self.buffer.push(Box::new(mino::TMino::default()));
    }
}

impl NextGenerator for DefaultNextGenerator {
    /// 次のミノを取得する
    // TODO:
    // 現在の実装ではbufferの中身が0になってから生成しているのでnextとして画面に表示できない．
    fn next(&mut self) -> Box<dyn mino::Mino> {
        match self.buffer.pop() {
            Some(top) => top,
            None => {
                self.generate();
                self.buffer.pop().unwrap()
            }
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
            buffer: vec![],
            rand_gen: rand_gen,
        };

        // TODO: もう少しちゃんとしたテストをしたほうがいいかも？
        // 取り出した値がすべてmino::Minoになっているかの判定はしてもいいかも？
        for _ in 0..10 {
            nx.generate();
        }
    }
}
