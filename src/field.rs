/// 21$B!_(B10$B$N%F%H%j%9$N%U%#!<%k%I$rI=8=(B
/// controller$B$+$i(Bstep$B$,8F$S=P$5$l$=$N$?$S$KMn2<=hM}$d:o=|=hM}$r9T$&M=Dj(B

// $B%U%#!<%k%I$N3F%V%m%C%/(B
struct FieldBlock {
    filled: bool,    // $B%V%m%C%/$K%_%N$,B8:_$9$k$+(B
    color: [f32; 4], // $B%V%m%C%/$N?'(B
}

// $B%F%H%j%9$N%U%#!<%k%I(B
struct Field {
    height: usize,
    width: usize,
    blocks: Vec<Vec<FieldBlock>>,
}

impl Field {
    /// Field$B$N%3%s%9%H%i%/%?(B
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
        // block$B$,$9$Y$FKd$^$C$F$$$J$$$+$r%F%9%H(B
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
