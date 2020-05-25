/// 21$B!_(B10$B$N%F%H%j%9$N%U%#!<%k%I$rI=8=(B
/// controller$B$+$i(Bstep$B$,8F$S=P$5$l$=$N$?$S$KMn2<=hM}$d:o=|=hM}$r9T$&M=Dj(B

// $B%U%#!<%k%I$N3F%V%m%C%/(B
struct FieldBlock {
    filled: bool,    // $B%V%m%C%/$K%_%N$,B8:_$9$k$+(B
    color: [f32; 4], // $B%V%m%C%/$N?'(B
}

// $B%F%H%j%9$N%U%#!<%k%I(B
struct Field {
    blocks: Vec<Vec<FieldBlock>>,
}

impl Field {
    const HEIGHT: usize = 21;
    const WIDTH: usize = 10;
}

impl Field {
    /// Field$B$N%3%s%9%H%i%/%?(B
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
