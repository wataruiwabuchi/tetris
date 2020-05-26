// TODO : $B%a%s%PJQ?t$r;2>H$9$k4X?t$N=hM}$r6&DL2=$9$kJ}K!$rC5$9(B
// $B8=>u$G$O(Bget_height$B$J$I$NA4$/F1$8F0:n$r9T$&4X?t$r$9$Y$F$N%_%N$KBP$7$F<BAu$7$F$$$k(B
// trait$B$N%G%U%)%k%H<BAu$G$3$NItJ,$r6&DL2=$G$-$l$PNI$$$,(Btrait$B$+$i$O%a%s%PJQ?t$K%"%/%;%9$G$-$J$$$N$G$=$NItJ,$K<BAu$9$k$H%(%i!<$,=P$k(B

pub enum Orientation {
    Upward,
    Rightward,
    Downward,
    Leftward,
}

pub struct Mino {
    x: usize,
    y: usize,
    size: usize,
    shape: Vec<Vec<bool>>,
    color: [f32; 4],
}

impl Mino {
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn get_shape(&self) -> Vec<Vec<bool>> {
        // TODO: readonly$B$J;2>H$rJV$9$@$1$G$b$$$$$+$b!)(B($B$=$&$9$k$H;2>H$,JV$C$F$3$J$$$+!)(B)
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
