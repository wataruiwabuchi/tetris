use crate::field;
use crate::mino;
use crate::next_generator;
use crate::next_generator::NextGenerator;

// ゲーム進行や各要素を管理
// 各インタフェースだけでも先に決めておかないとこっちがつらいかも？
pub struct GameMaster {
    pub field: field::Field, // テトリスのフィールド
    //cm: field::ControlledMino<dyn mino::Mino>, // 操作しているミノ
    cm: Box<field::ControlledMino>, // 操作しているミノ
    //timer: Timer, // クロック生成器
    // TODO: gbgの実装
    //gbg : GarbageBlockGenerator, // おじゃまブロックの出現を管理
    //params : Parameter, // 各種パラメータ
    ng: Box<dyn next_generator::NextGenerator>, // ネクスト生成器
                                                // TODO: holdの実装
                                                //hold : HoldMino, // ホールド
                                                //controller: Controller, // ユーザインタフェース（コントローラー）
                                                //renderer : Renderer, // 無限ループするなら画面描写もこちらに持たせておいたほうがいい？（インタフェース化はしておく）
}

impl GameMaster {
    pub fn new(height: usize, width: usize, rand_gen: Box<dyn FnMut() -> usize>) -> GameMaster {
        let mut ng = next_generator::DefaultNextGenerator {
            buffer: vec![],
            rand_gen: rand_gen,
        };
        GameMaster {
            field: field::Field::new(height, width),
            cm: Box::new(field::ControlledMino::new(width / 2, ng.next())), // TODO: ContorolledMinoの幅を考慮する必要
            ng: Box::new(ng),
        }
    }
    pub fn tick(&mut self) {
        // TODO: loopの繰り返し回数で行うか時間を計測して行うかは議論の余地がある
        //self.field.get_block(0, 0).filled = true;

        // loop回数の場合はloop内の実行時間の影響を受ける
        // 時間の場合は何回処理を行ったかを記録しておく必要がある？
        //let mut clk = 0;
        // 落下
        self.cm.move_mino(&self.field, field::Orientation::Downward);

        if self.cm.get_grounded() {
            // ControlledMinoの位置を確定
            let rendered_mino = self.cm.render();
            for i in 0..rendered_mino.len() {
                for j in 0..rendered_mino[i].len() {
                    if i + self.cm.get_y() < self.field.get_height()
                        && j + self.cm.get_x() < self.field.get_width()
                    {
                        self.field
                            .get_block(i + self.cm.get_y(), j + self.cm.get_x())
                            .filled |= rendered_mino[i][j];
                    }
                }
            }

            // ControlledMinoの切り替え
            self.cm = Box::new(field::ControlledMino::new(
                self.field.get_width() / 2, // ControlledMinoの幅を考慮
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
}
