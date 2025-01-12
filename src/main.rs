use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Read, Write},
    thread::sleep,
    time::Duration,
};
use termion::{async_stdin, raw::IntoRawMode, terminal_size};

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

fn clear(mut stdout: impl Write) {
    write!(stdout, "{}", termion::clear::All).unwrap();
}

fn update(school_of_fish: &mut [Turbofish], columns: u16) {
    for fish in school_of_fish.iter_mut() {
        fish.tick();
        if fish.x < -10 || fish.x > columns as i16 + 10 {
            fish.speed *= -1;
        }
    }
}

fn render(mut stdout: impl Write, school_of_fish: &Vec<Turbofish>, columns: u16) {
    for fish in school_of_fish {
        let x = fish.x;
        let y = fish.y;
        if fish.blink {
            write!(stdout, "{}", termion::color::Fg(termion::color::LightBlack)).unwrap();
        }
        write!(
            stdout,
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
    let mut rng = thread_rng();
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdout = termion::cursor::HideCursor::from(stdout);

    let (columns, lines) = terminal_size().unwrap();

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

    clear(&mut stdout);
    stdout.flush().unwrap();

    loop {
        if let Some(Ok(b'q')) = stdin.next() {
            break;
        }

        clear(&mut stdout);
        update(&mut school_of_fish, columns);
        render(&mut stdout, &school_of_fish, columns);
        stdout.flush().unwrap();

        sleep(Duration::from_millis(100));
    }

    clear(&mut stdout);
    stdout.flush().unwrap();
}
