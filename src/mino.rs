// TODO : $B%a%s%PJQ?t$r;2>H$9$k4X?t$N=hM}$r6&DL2=$9$kJ}K!$rC5$9(B
// $B8=>u$G$O(Bget_size$B$J$I$NA4$/F1$8F0:n$r9T$&4X?t$r$9$Y$F$N%_%N$KBP$7$F<BAu$7$F$$$k(B
// trait$B$N%G%U%)%k%H<BAu$G$3$NItJ,$r6&DL2=$G$-$l$PNI$$$,(Btrait$B$+$i$O%a%s%PJQ?t$K%"%/%;%9$G$-$J$$$N$G$=$NItJ,$K<BAu$9$k$H%(%i!<$,=P$k(B

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
