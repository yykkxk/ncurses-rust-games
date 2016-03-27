extern crate ncurses;
mod snake;

use ncurses::*;
use snake::*;
use std::thread::sleep as std_sleep;
use std::time::Duration as StdDuration;

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr, true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(250);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr, &mut max_y, &mut max_x);

    let mut snake2 = Snake::new_with_bounds(max_x as u32, max_y as u32);

    loop {
        // mvaddch((snake2.bounds.1 - 1) as i32, (snake2.bounds.0 - 1) as i32, '@' as u64);
        get_key(&mut snake2);

        if snake2.paused {
            diaplay_str("Game is paused...\n
            ====\n
            This is a 'Snake' game written by Ding Bin.\n
            ====\n
            Press Key `N` to Begin a new Game\n
            Press Key `P` to Pause the Game\n
            Press Key `R` to Resume a paused Game\n
            Press `Ctrl + C` to Stop the Game\n
            ====\n");
        } else {
            display_snake(&snake2);
            match snake2.move_on() {
                Ok(_) => {
                    // std_sleep(StdDuration::from_secs(1));
                },
                Err(SnakeDead::HitTail(message))| Err(SnakeDead::HitWall(message)) => {
                    diaplay_str(message);
                    std_sleep(StdDuration::from_secs(2));
                    break;
                },
            }
        }

        // get_key(&mut snake2);
    }

    endwin();
}

fn diaplay_str(s: &str) {
    erase();
    addstr(s);
    refresh();
}

fn display_char(p: Point, ch: char) {
    mvaddch(p.1, p.0, ch as u64);
}

fn get_key(sn: &mut Snake) {
    match getch() {
        KEY_UP if !sn.paused => {
            if sn.direction != Direction::Down {
                sn.direction = Direction::Up;
            }
        },
        KEY_DOWN if !sn.paused => {
            if sn.direction != Direction::Up {
                sn.direction = Direction::Down;
            }
        },
        KEY_LEFT if !sn.paused => {
            if sn.direction != Direction::Right {
                sn.direction = Direction::Left;
            }
        },
        KEY_RIGHT if !sn.paused => {
            if sn.direction != Direction::Left {
                sn.direction = Direction::Right;
            }
        },
        112 => {
            sn.paused = true;
        },
        114 if sn.paused => {
            sn.paused = false;
        },
        110 => {
            *sn = Snake::new_with_bounds(sn.bounds.0, sn.bounds.1);
            sn.paused = false;
        },
        _ => {},
    }
}

fn display_snake(sn: &Snake) {
    erase();

    display_char(sn.egg, 'o');
    display_char(sn.head, '+');
    for p in sn.body.iter() {
        display_char(*p, '-');
    }
}
