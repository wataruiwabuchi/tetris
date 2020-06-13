use crate::field;
use crate::mino;
use crate::next_generator;
use crate::next_generator::NextGenerator;
use std::time::Instant;

pub struct TetrisParams {
    drop_interval: u64, // millisecondを想定
}

impl Default for TetrisParams {
    fn default() -> Self {
        TetrisParams {
            drop_interval: 1500,
        }
    }
}

// ゲーム進行や各要素を管理
// 各インタフェースだけでも先に決めておかないとこっちがつらいかも？
pub struct GameMaster {
    pub field: field::Field, // テトリスのフィールド
    //cm: field::ControlledMino<dyn mino::Mino>, // 操作しているミノ
    pub cm: Box<field::ControlledMino>, // 操作しているミノ
    //timer: Timer, // クロック生成器
    // TODO: gbgの実装
    //gbg : GarbageBlockGenerator, // おじゃまブロックの出現を管理
    //params : Parameter, // 各種パラメータ
    ng: Box<dyn next_generator::NextGenerator>, // ネクスト生成器
    // TODO: holdの実装
    //hold : HoldMino, // ホールド
    //controller: Controller, // ユーザインタフェース（コントローラー）
    //renderer : Renderer, // 無限ループするなら画面描写もこちらに持たせておいたほうがいい？（インタフェース化はしておく）
    start_time_in_milli: i32,
    count_drop: i32,
    params: TetrisParams,
}

impl GameMaster {
    pub fn new(
        height: usize,
        width: usize,
        rand_gen: Box<dyn FnMut() -> usize>,
        start_time_in_milli: i32,
    ) -> GameMaster {
        let mut ng = next_generator::DefaultNextGenerator {
            buffer: vec![],
            rand_gen: rand_gen,
        };
        GameMaster {
            field: field::Field::new(height, width),
            cm: Box::new(field::ControlledMino::new((width / 2) as i64, ng.next())), // TODO: ContorolledMinoの幅を考慮する必要
            ng: Box::new(ng),
            start_time_in_milli: start_time_in_milli,
            count_drop: 0,
            params: TetrisParams::default(),
        }
    }
    pub fn tick(&mut self, current_time_in_milli: i32) {
        let elapsed_time_in_milli = current_time_in_milli - self.start_time_in_milli;
        // loop回数の場合はloop内の実行時間の影響を受ける
        // 時間の場合は何回処理を行ったかを記録しておく必要がある？
        // 落下
        if elapsed_time_in_milli / self.params.drop_interval as i32 != self.count_drop {
            self.cm.move_mino(&self.field, field::Orientation::Downward);
            self.count_drop = elapsed_time_in_milli / self.params.drop_interval as i32;
        }

        if self.cm.get_grounded() {
            // ControlledMinoの位置を確定
            let rendered_mino = self.cm.render();
            for i in 0..rendered_mino.len() {
                for j in 0..rendered_mino[i].len() {
                    if i as i64 + self.cm.get_y() >= 0
                        && i as i64 + self.cm.get_y() < self.field.get_height() as i64
                        && j as i64 + self.cm.get_x() >= 0
                        && j as i64 + self.cm.get_x() < self.field.get_width() as i64
                    {
                        self.field
                            .get_block(i + self.cm.get_y() as usize, j + self.cm.get_x() as usize)
                            .filled |= rendered_mino[i][j];
                    }
                }
            }

            // ControlledMinoの切り替え
            self.cm = Box::new(field::ControlledMino::new(
                (self.field.get_width() / 2) as i64, // ControlledMinoの幅を考慮
                self.ng.next(),
            ));
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
    pub fn project_controlled_mino(&mut self) -> Vec<Vec<bool>> {
        let width = self.field.get_width();
        let height = self.field.get_height();
        let mut projected = vec![vec![false; width]; height];
        for i in 0..height {
            for j in 0..width {
                projected[i][j] = self.field.get_block(i, j).filled;
            }
        }

        let x = self.cm.get_x();
        let y = self.cm.get_y();
        let rendered_mino = self.cm.render();
        for i in 0..rendered_mino.len() {
            for j in 0..rendered_mino[i].len() {
                if i as i64 + y >= 0
                    && i as i64 + y < height as i64
                    && j as i64 + x >= 0
                    && j as i64 + x < width as i64
                {
                    projected[i + y as usize][j + x as usize] |= rendered_mino[i][j];
                }
            }
        }
        projected
    }
}
