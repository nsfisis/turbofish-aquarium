use rand::Rng as _;
use std::io::{Read as _, Write as _};
use termion::raw::IntoRawMode as _;

const N_FISH: usize = 64;

struct Turbofish {
    x: i16,
    y: u16,
    speed: i16,
    blink: bool,
}

impl Turbofish {
    fn text(&self, offset: i16) -> String {
        let s = if self.speed < 0 { "<>::" } else { "::<>" };
        let s = if offset <= -4 {
            ""
        } else if offset < 0 {
            &s[(-offset as usize)..]
        } else if offset >= 4 {
            ""
        } else if offset > 0 {
            &s[0..(4 - offset as usize)]
        } else {
            s
        };
        s.to_string()
    }

    fn tick(&mut self) {
        self.x += self.speed;
        self.blink = !self.blink;
    }
}

fn clear(mut output: impl std::io::Write) {
    write!(output, "{}", termion::clear::All).unwrap();
}

fn update(school_of_fish: &mut [Turbofish], columns: u16) {
    for fish in school_of_fish.iter_mut() {
        fish.tick();
        if fish.x < -10 || fish.x > columns as i16 + 10 {
            fish.speed *= -1;
        }
    }
}

fn render(mut output: impl std::io::Write, school_of_fish: &Vec<Turbofish>, columns: u16) {
    for fish in school_of_fish {
        let x = fish.x;
        let y = fish.y;
        if fish.blink {
            write!(output, "{}", termion::color::Fg(termion::color::LightBlack)).unwrap();
        }
        write!(
            output,
            "{}{}{}",
            termion::cursor::Goto(x.clamp(0, columns as i16 - 1) as u16 + 1, y + 1),
            fish.text(if x < 0 {
                x
            } else if (x as u16) + 4 >= columns {
                x + 4 - columns as i16
            } else {
                0
            }),
            termion::style::Reset,
        )
        .unwrap();
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut input = termion::async_stdin().bytes();
    let output = std::io::stdout().into_raw_mode().unwrap();
    let mut output = termion::cursor::HideCursor::from(output);

    let (columns, lines) = termion::terminal_size().unwrap();

    let mut school_of_fish = Vec::with_capacity(N_FISH);
    for _i in 0..N_FISH {
        let x = rng.gen_range(0..(columns as i16 - 1));
        let y = rng.gen_range(0..(lines - 1));
        let speed = [-2, -1, 1, 2][rng.gen_range(0..4)];
        school_of_fish.push(Turbofish {
            x,
            y,
            speed,
            blink: rng.gen(),
        });
    }

    clear(&mut output);
    output.flush().unwrap();

    loop {
        if let Some(Ok(b'q')) = input.next() {
            break;
        }

        clear(&mut output);
        update(&mut school_of_fish, columns);
        render(&mut output, &school_of_fish, columns);
        output.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    clear(&mut output);
    output.flush().unwrap();
}
