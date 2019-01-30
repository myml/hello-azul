extern crate azul;

use azul::{prelude::*, widgets::label::Label};

struct DataModel {
    hello: String,
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.hello))
            .dom()
            .with_callback(On::MouseUp, Callback(click));

        return Dom::new(NodeType::Div).with_child(label);
    }
}

fn click(_state: &mut AppState<DataModel>, _event: &mut CallbackInfo<DataModel>) -> UpdateScreen {
    let mut data = _state.data.lock().unwrap();
    data.hello = data.hello.chars().rev().collect::<String>();
    println!("click!{}", data.hello);
    return Redraw;
}

fn main() {
    let mut app = App::new(
        DataModel {
            hello: "Hello,世界".into(),
        },
        AppConfig::default(),
    );
    // load font
    const FONT_FILE: &[u8] = include_bytes!("./wqy-microhei.ttc");
    let font_id = FontId::ExternalFont("wqy".into());
    app.add_font(font_id, &mut FONT_FILE.clone()).unwrap();
    // load css
    let style = css::override_native(include_str!("./main.css")).unwrap();

    let option = WindowCreateOptions::default();
    let window = Window::new(option, style).unwrap();
    app.run(window).unwrap();
}
