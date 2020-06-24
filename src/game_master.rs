use crate::field;
use crate::garbage_block_generator;
use crate::mino;
use crate::next_generator;
use crate::next_generator::NextGenerator;
use std::time::Instant;

pub enum Hold {
    Holding(Box<dyn mino::Mino>),
    None,
}

#[derive(Copy, Clone)]
pub enum Keyboard {
    Rightrotate,
    Leftrotate,
    Softdrop,
    Harddrop,
    Rightmove,
    Leftmove,
    Hold,
    Other,
}

pub struct TetrisParams {
    drop_interval: u64,    // millisecondを想定
    move_interval: u64,    // millisecondを想定
    garbage_interval: u64, // millisecondを想定
}

impl Default for TetrisParams {
    fn default() -> Self {
        TetrisParams {
            drop_interval: 1500,
            move_interval: 50,
            garbage_interval: 10000,
        }
    }
}

// ゲーム進行や各要素を管理
// 各インタフェースだけでも先に決めておかないとこっちがつらいかも？
pub struct GameMaster {
    pub field: field::Field,            // テトリスのフィールド
    pub cm: Box<field::ControlledMino>, // 操作しているミノ
    gbg: Box<dyn garbage_block_generator::GarbageBlockGenerator>, // おじゃまブロック
    ng: Box<dyn next_generator::NextGenerator>, // ネクスト生成器
    hold: Hold,                         // ホールド
    holded: bool,                       // 連続でホールドを行うことを禁止
    start_time_in_milli: i32,
    count_drop: i32,
    count_move: i32,
    count_garbage: i32,
    enable_ghost: bool,
    enable_garbage: bool,
    ghost_color: [f32; 4],
    game_over: bool,
    params: TetrisParams,
}

impl GameMaster {
    pub fn new(
        height: usize,
        width: usize,
        rand_gen_ng: Box<dyn FnMut() -> usize>,
        rand_gen_gbg: Box<dyn FnMut() -> usize>,
        start_time_in_milli: i32,
        enable_ghost: bool,
        enable_garbage: bool,
    ) -> GameMaster {
        // TODO: 二つrand_genを受け取る必要はないはず
        // 共有する方法を考える
        // 乱数が必要な場合に引数として渡すのも一つ
        let mut ng = next_generator::DefaultNextGenerator::new(rand_gen_ng);
        let gbg = garbage_block_generator::HoritetoGarbageBlockGenerator::new(rand_gen_gbg);
        GameMaster {
            field: field::Field::new(height, width),
            cm: Box::new(field::ControlledMino::new((width / 2) as i64, ng.next())), // TODO: ContorolledMinoの幅を考慮する必要
            gbg: Box::new(gbg),
            ng: Box::new(ng),
            hold: Hold::None,
            holded: false,
            start_time_in_milli: start_time_in_milli,
            count_drop: 0,
            count_move: 0,
            count_garbage: 0,
            enable_ghost: enable_ghost,
            enable_garbage: enable_garbage,
            ghost_color: [0.5; 4],
            game_over: false,
            params: TetrisParams::default(),
        }
    }

    pub fn tick(&mut self, current_time_in_milli: i32, key: Keyboard) {
        let elapsed_time_in_milli = current_time_in_milli - self.start_time_in_milli;
        // loop回数の場合はloop内の実行時間の影響を受ける
        // 時間の場合は何回処理を行ったかを記録しておく必要がある？
        // 落下
        if elapsed_time_in_milli / self.params.drop_interval as i32 != self.count_drop {
            self.cm.move_mino(&self.field, field::Orientation::Downward);
            self.count_drop = elapsed_time_in_milli / self.params.drop_interval as i32;
        }

        // おじゃまブロックの生成
        if self.enable_garbage {
            if elapsed_time_in_milli / self.params.garbage_interval as i32 != self.count_garbage {
                let garbage_lines = self.gbg.generate(self.field.get_width(), 1, [0.0; 4]);
                match self.field.insert_lines(garbage_lines) {
                    Ok(_) => {
                        // TODO: おじゃまブロックを生成したときの接地処理が自信がない
                        for _ in 0..self.cm.get_y() {
                            let rendered_mino = self.cm.render();
                            let mut block_overlapping = false;
                            for i in 0..rendered_mino.len() {
                                for j in 0..rendered_mino[i].len() {
                                    let block_filled = self
                                        .field
                                        .get_block(
                                            i + self.cm.get_y() as usize,
                                            j + self.cm.get_x() as usize,
                                        )
                                        .filled;
                                    if rendered_mino[i][j] && block_filled {
                                        block_overlapping = true;
                                        self.cm.set_grounded(true);
                                        break;
                                    }
                                }
                            }

                            if !block_overlapping {
                                break;
                            }

                            self.cm.move_mino(&self.field, field::Orientation::Upward);
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        self.game_over = true;
                    }
                }

                self.count_garbage = elapsed_time_in_milli / self.params.garbage_interval as i32;
            }
        }

        if self.cm.get_grounded() {
            // ControlledMinoの位置を確定
            let rendered_mino = self.cm.render();
            for i in 0..rendered_mino.len() {
                for j in 0..rendered_mino[i].len() {
                    if !(i as i64 + self.cm.get_y() >= 0
                        && i as i64 + self.cm.get_y() < self.field.get_height() as i64
                        && j as i64 + self.cm.get_x() >= 0
                        && j as i64 + self.cm.get_x() < self.field.get_width() as i64)
                    {
                        continue;
                    }

                    if rendered_mino[i][j] {
                        self.field
                            .get_block(i + self.cm.get_y() as usize, j + self.cm.get_x() as usize)
                            .filled = true;
                        for k in 0..4 {
                            self.field
                                .get_block(
                                    i + self.cm.get_y() as usize,
                                    j + self.cm.get_x() as usize,
                                )
                                .color[k] = self.cm.get_mino().get_color()[k];
                        }
                    }
                }
            }

            // ControlledMinoの切り替え
            self.cm = Box::new(field::ControlledMino::new(
                (self.field.get_width() / 2) as i64, // ControlledMinoの幅を考慮
                self.ng.next(),
            ));

            self.holded = false;
        }

        match key {
            Keyboard::Rightrotate => self.cm.right_rotate(&mut self.field),
            Keyboard::Leftrotate => self.cm.left_rotate(&mut self.field),
            Keyboard::Softdrop => {
                if elapsed_time_in_milli / self.params.move_interval as i32 != self.count_move {
                    self.cm.move_mino(&self.field, field::Orientation::Downward);
                    self.count_move = elapsed_time_in_milli / self.params.move_interval as i32;
                }
            }
            Keyboard::Harddrop => {
                for _ in 0..self.field.get_height() {
                    self.cm.move_mino(&self.field, field::Orientation::Downward);
                }
            }
            Keyboard::Rightmove => {
                if elapsed_time_in_milli / self.params.move_interval as i32 != self.count_move {
                    self.cm
                        .move_mino(&self.field, field::Orientation::Rightward);
                    self.count_move = elapsed_time_in_milli / self.params.move_interval as i32;
                }
            }
            Keyboard::Leftmove => {
                if elapsed_time_in_milli / self.params.move_interval as i32 != self.count_move {
                    self.cm.move_mino(&self.field, field::Orientation::Leftward);
                    self.count_move = elapsed_time_in_milli / self.params.move_interval as i32;
                }
            }
            Keyboard::Hold => {
                if self.holded == false {
                    // 参考
                    // https://frozenlib.net/blog/2018-03-11_rust-pattern-match/
                    match self.hold {
                        Hold::Holding(ref mut m) => {
                            std::mem::swap(m, self.cm.get_mino());
                        }
                        Hold::None => {
                            // https://qiita.com/quasardtm/items/b54a48c1accd675e0bf1
                            let mut m: Box<dyn mino::Mino> = Box::new(mino::TMino::default());
                            std::mem::swap(&mut m, self.cm.get_mino());
                            self.hold = Hold::Holding(m);
                            self.cm = Box::new(field::ControlledMino::new(
                                (self.field.get_width() / 2) as i64, // 初期位置を調整
                                self.ng.next(),
                            ));
                        }
                    };
                    self.holded = true;
                }
            }
            Keyboard::Other => {}
        }

        // TODO: webassemblyを使うかでキーイベントも変化するかも
        // 自分が実現したい抽象化を考えるとその部分は分離しておきたい
        // 抽象化されているとしてどのような実装か
        // コントローラーにイベントとメソッドを登録
        // またはコントローラーの状態に押下されたキーを持たせておく
        // userの操作イベントを取得して移動やホールド処理
        // 接地してから位置を確定させる処理はこっちで実装？
        // clkに関係するのでこっちのほうがよさそう
        // fieldやcontrolledminoに位置確定という命令を投げる？

        // generate garbage

        // 画面描写
    }

    /// ControlledMinoをFieldに投影
    pub fn project_controlled_mino(&mut self) -> (Vec<Vec<bool>>, Vec<Vec<[f32; 4]>>) {
        let width = self.field.get_width();
        let height = self.field.get_height();
        let mut projected_filled = vec![vec![false; width]; height];
        let mut projected_color = vec![vec![[1.0; 4]; width]; height];
        for i in 0..height {
            for j in 0..width {
                projected_filled[i][j] = self.field.get_block(i, j).filled;
                for k in 0..4 {
                    projected_color[i][j][k] = self.field.get_block(i, j).color[k];
                }
            }
        }

        let x = self.cm.get_x();
        let y = self.cm.get_y();
        let grounded = self.cm.get_grounded();

        if self.enable_ghost {
            for _ in 0..height {
                self.cm.move_mino(&self.field, field::Orientation::Downward);
            }

            // ghostを表示
            let ghost_x = self.cm.get_x();
            let ghost_y = self.cm.get_y();
            let rendered_mino = self.cm.render();
            for i in 0..rendered_mino.len() {
                for j in 0..rendered_mino[i].len() {
                    if !(i as i64 + ghost_y >= 0
                        && i as i64 + ghost_y < height as i64
                        && j as i64 + ghost_x >= 0
                        && j as i64 + ghost_x < width as i64)
                    {
                        continue;
                    }

                    if rendered_mino[i][j] {
                        projected_filled[i + ghost_y as usize][j + ghost_x as usize] = true;
                        for k in 0..4 {
                            projected_color[i + ghost_y as usize][j + ghost_x as usize][k] =
                                self.ghost_color[k];
                        }
                    }
                }
            }

            self.cm.set_y(y);
            self.cm.set_grounded(grounded);
        }

        // 操作中のミノを表示
        // TODO: ほぼ同じコードが連続しているので削除したい
        let rendered_mino = self.cm.render();
        for i in 0..rendered_mino.len() {
            for j in 0..rendered_mino[i].len() {
                if !(i as i64 + y >= 0
                    && i as i64 + y < height as i64
                    && j as i64 + x >= 0
                    && j as i64 + x < width as i64)
                {
                    continue;
                }

                if rendered_mino[i][j] {
                    projected_filled[i + y as usize][j + x as usize] = true;
                    for k in 0..4 {
                        projected_color[i + y as usize][j + x as usize][k] =
                            self.cm.get_mino().get_color()[k];
                    }
                }
            }
        }

        (projected_filled, projected_color)
    }

    pub fn get_next(&self, idx: usize) -> Option<&Box<dyn mino::Mino>> {
        self.ng.get_next(idx)
    }

    pub fn get_hold(&self) -> &Hold {
        &self.hold
    }
}
