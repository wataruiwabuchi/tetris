use crate::field;
use rand::Rng;

// ゲーム進行や各要素を管理
// 各インタフェースだけでも先に決めておかないとこっちがつらいかも？
pub struct GameMaster {
    pub field: field::Field, // テトリスのフィールド
                             //cm : ControlledMino, // 操作しているミノ
                             //timer: Timer, // クロック生成器
                             // TODO: gbgの実装
                             //gbg : GarbageBlockGenerator, // おじゃまブロックの出現を管理
                             //params : Parameter, // 各種パラメータ
                             //ng : NextGenerator, // ネクスト生成器
                             // TODO: holdの実装
                             //hold : HoldMino, // ホールド
                             //controller: Controller, // ユーザインタフェース（コントローラー）
                             //renderer : Renderer, // 無限ループするなら画面描写もこちらに持たせておいたほうがいい？（インタフェース化はしておく）
}

impl GameMaster {
    pub fn new() -> GameMaster {
        GameMaster {
            field: field::Field::new(10, 5),
        }
    }
    pub fn tick(&mut self) {
        // TODO: loopの繰り返し回数で行うか時間を計測して行うかは議論の余地がある
        let mut rng = rand::thread_rng();
        let row: usize = rng.gen::<usize>() % self.field.get_height();
        let col: usize = rng.gen::<usize>() % self.field.get_width();
        self.field.get_block(row, col).filled = true;

        // loop回数の場合はloop内の実行時間の影響を受ける
        // 時間の場合は何回処理を行ったかを記録しておく必要がある？
        //let mut clk = 0;
        // 落下

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
