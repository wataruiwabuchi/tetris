// おじゃまブロックの生成器
// 戦略を切り替えられるといいかも？
// 完全ランダム，一列あたりに何個の空きがあるか，同じ列が空く確率を上げる等
// 掘りテトの一定時間で出現するのかなどはGameMaster側で管理？
use crate::field;
use std::collections::HashSet;

pub trait GarbageBlockGenerator {
    fn generate(
        &mut self,
        field_width: usize,
        num_garbage_lines: usize,
        color: [f32; 4],
    ) -> Vec<Vec<field::FieldBlock>>;
}

// TOJの掘りテトを意識したもの
// 同じ列を空ける確率を操作しない
// 一列あたり空くのは一か所とは限らない（とはいっても何か制限は必要かも）
pub struct HoritetoGarbageBlockGenerator {
    rand_gen: Box<dyn FnMut() -> usize>,
}

impl HoritetoGarbageBlockGenerator {
    pub fn new(rand_gen: Box<dyn FnMut() -> usize>) -> HoritetoGarbageBlockGenerator {
        HoritetoGarbageBlockGenerator { rand_gen: rand_gen }
    }
}

impl GarbageBlockGenerator for HoritetoGarbageBlockGenerator {
    fn generate(
        &mut self,
        field_width: usize,
        num_garbage_lines: usize,
        color: [f32; 4],
    ) -> Vec<Vec<field::FieldBlock>> {
        let mut garbage_lines: Vec<Vec<field::FieldBlock>> = Vec::new();
        for _ in 0..num_garbage_lines {
            let mut line: Vec<field::FieldBlock> = (0..field_width)
                .map(|_| field::FieldBlock {
                    filled: true,
                    color: color,
                })
                .collect();
            let num_hole = ((self.rand_gen)() % (field_width / 2) + 1).min(field_width);
            let mut hole_ids = HashSet::new();
            for _ in 0..field_width {
                let idx = (self.rand_gen)() % field_width;
                hole_ids.insert(idx);
                line[idx].filled = false;
                if hole_ids.len() >= num_hole {
                    break;
                }
            }
            garbage_lines.push(line);
        }
        return garbage_lines;
    }
}

#[cfg(test)]
mod horitetogarbageblockgenerator_tests {
    use super::*;

    #[test]
    fn test_generate() {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<usize>());
        let mut gbg = HoritetoGarbageBlockGenerator { rand_gen: rand_gen };

        let field_width = 10;
        let num_garbage_lines = 1000;
        let garbage_lines = gbg.generate(field_width, num_garbage_lines, [0.0; 4]);

        assert!(garbage_lines.len() == num_garbage_lines);

        // 一列あたりの穴の数Nが1 <= N <= field_width / 2になっているかをテスト
        for line in garbage_lines {
            let count_hole = line.iter().fold(0, |acc, x| acc + !(x.filled) as usize);
            assert!(count_hole >= 1 && count_hole <= field_width / 2);
        }
    }
}
