mod termbox2;

use termbox2::{
    TB_WHITE, TB_BLUE, TB_INPUT_ESC,
    TB_INPUT_MOUSE, TB_ITALIC, TB_GREEN,
    TB_BLACK, TB_BOLD, TB_YELLOW, TB_EVENT_KEY, TB_KEY_ESC, TB_OK
};

#[derive(Copy, Clone)]
struct Key {
    x: u8,
    y: i8,
    ch: char,
}

impl Key {
    fn esc() -> [Key; 4] {
        [
            Self { x: 31, y: 1, ch: 'E' },
            Self { x: 32, y: 1, ch: 'S' },
            Self { x: 33, y: 1, ch: 'C' },
            Self { x: 34, y: 1, ch: 0 as char }
        ]
    }
}

fn print_tb(str: &str, mut x: i32, y: i32, fg: u16, bg: u16) {
    let mut l = str.len();
    let mut uni: u32 = 0;

    while l != 0 {
        unsafe {
            str.chars().for_each(|f| {
                termbox2::tb_utf8_char_to_unicode(
                    &mut uni, std::slice::from_raw_parts(
                        &f,
                        std::mem::size_of::<char>()
                    ).as_ptr() as *const i8,
                );
                termbox2::tb_set_cell(x, y, uni, fg, bg);

                x += 1;
                l -= 1;
            });
        }
    }
}

fn draw_key(k: Key, fg: u16, bg: u16) {
    unsafe {
        termbox2::tb_set_cell(
            (k.x + 2).into(), (k.y + 4).into(),
            k.ch as u32, fg, bg
        );
    }
}

fn main() {
    unsafe {
        termbox2::tb_init();
        termbox2::tb_set_input_mode((TB_INPUT_ESC | TB_INPUT_MOUSE)
            .try_into()
            .unwrap()
        );
        termbox2::tb_clear();

        let mut ev = termbox2::tb_event {
            type_: 0,
            mod_: 0,
            key: 0,
            ch: 0,
            w: 0,
            h: 0,
            x: 0,
            y: 0,
        };

        let mut i = 0;
        let mut l = 0;

        print_tb(
            "Hello world", 30, 0,
            (TB_WHITE | TB_BOLD) as u16,
            TB_BLACK as u16
        );
        print_tb(
            "Goodbye world", 29, 1,
            (TB_GREEN | TB_ITALIC) as u16,
            TB_BLACK as u16
        );

        while i < 3 {
            draw_key(Key::esc()[i], TB_WHITE as u16, TB_BLUE as u16);
            i += 1
        }

        i = 0;

        while i < 3 {
            termbox2::tb_set_cell(
                27, i as i32, 0x2588,
                TB_YELLOW as u16,
                TB_YELLOW as u16
            );

            while i == 2 && l < 16 {
                termbox2::tb_set_cell(
                    28 + l, 2 as i32, 0x2588,
                    TB_YELLOW as u16,
                    TB_YELLOW as u16
                );

                if l == 15 {
                    i = 2;

                    while i != 0 {
                        i -= 1;

                        termbox2::tb_set_cell(
                            28 + l, i as i32, 0x2588,
                            TB_YELLOW as u16,
                            TB_YELLOW as u16
                        );
                    }
                }
                l += 1;
            }
            i += 1;
        }

        print_tb("Press", 27, 5, TB_WHITE as u16, TB_BLACK as u16);
        print_tb("to exit", 37, 5, TB_WHITE as u16, TB_BLACK as u16);
        termbox2::tb_present();

        while termbox2::tb_poll_event(&mut ev) == TB_OK as i32 {
            match ev.type_ as u32 {
                TB_EVENT_KEY => {
                    if ev.key == TB_KEY_ESC as u16 {
                        termbox2::tb_shutdown();
                    }
                },
                _ => {}
            }
        }
    }
}
