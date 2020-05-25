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

    // $B2#Ns$4$H$K(Bmino$B$,B7$C$F$$$k$+$rH=Dj$7B7$C$F$$$kNs$N%$%s%G%/%9$rJV$9(B
    // $B:o=|$7$?$+$H$$$&>pJs$H:o=|$7$?Ns$N>pJs$rJV$9(B
    pub fn is_filled_each_row(&self) -> Option<Vec<usize>> {
        let mut filled_rows = Vec::new();

        for h in 0..self.get_height() {
            let mut not_filled = false;
            for w in 0..self.get_width() {
                if !self.blocks[h][w].filled {
                    not_filled = true;
                    break;
                }
            }

            if !not_filled {
                filled_rows.push(h);
            }
        }

        if filled_rows.len() == 0 {
            return None;
        }

        return Some(filled_rows);
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

    #[test]
    fn test_is_filled_each_row() {
        let test_height = 5;
        let test_width = 4;

        struct TestCase {
            name: String,
            x: Vec<Vec<bool>>,
            want: Option<Vec<usize>>,
        };

        let cases = vec![
            TestCase {
                name: "all false".to_string(),
                x: vec![vec![false; test_width]; test_height],
                want: None,
            },
            TestCase {
                name: "all true".to_string(),
                x: vec![vec![true; test_width]; test_height],
                want: Some(vec![0, 1, 2, 3, 4]),
            },
            TestCase {
                // $B0lIt$,Kd$^$C$F$$$k(B
                name: "hand craft".to_string(),
                x: vec![
                    vec![true, true, true, true],
                    vec![false, false, true, false],
                    vec![false, true, true, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                ],
                want: Some(vec![0, 3]),
            },
        ];

        for case in cases {
            // block$B$,$9$Y$FKd$^$C$F$$$J$$$+$r%F%9%H(B
            let mut f = Field::new(test_height, test_width);
            for h in 0..f.get_height() {
                for w in 0..f.get_width() {
                    f.blocks[h][w].filled = case.x[h][w];
                }
            }
            match f.is_filled_each_row() {
                Some(return_value) => {
                    assert_eq!(Some(return_value), case.want, "case {}: failed", case.name)
                }
                None => assert_eq!(None, case.want, "failed {}", case.name),
            }
        }
    }
}
