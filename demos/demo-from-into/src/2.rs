#[derive(Debug)]
enum KeyPress {
    Down,
    Up,
}

struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

#[derive(Debug)]
enum InputEvent {
    Key(u16, KeyPress),
    Mouse,
}

impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        InputEvent::Key(ev.keycode, ev.state)
    }
}

fn main() {
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };

    let input_ev = InputEvent::from(key_ev);
    dbg!(input_ev);

    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    let input_ev: InputEvent = key_ev.into();
    dbg!(input_ev);
}
