extern crate ncurses;
mod space_invaders;

use ncurses::*;
use space_invaders::*;
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

    let mut space_invaders2 = SpaceInvaders::new_with_bounds(max_x as u16, max_y as u16);
    space_invaders2.set_max_bullets(150);

    loop {
        get_key(&mut space_invaders2);

        if space_invaders2.paused {
            diaplay_str("Game is paused...\n
            ====\n
            This is a 'Space Invaders' game written by Ding Bin.\n
            ====\n
            Press Key `N` to Begin a new Game\n
            Press Key `P` to Pause the Game\n
            Press Key `R` to Resume a paused Game\n
            Press `Ctrl + C` to Stop the Game\n

            Press Key `Left` or `Right` to move the head.\n
            Press Key `Up` to shoot a bullet.\n
            ====\n");
        } else {
            display_space_invaders(&space_invaders2);
            match space_invaders2.move_on() {
                Ok(_) => {
                    // std_sleep(StdDuration::from_secs(1));
                },
                Err(GameOver::Win(message))| Err(GameOver::Lose(message)) => {
                    diaplay_str(message);
                    std_sleep(StdDuration::from_secs(2));
                    break;
                },
            }
        }
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

fn get_key(sp: &mut SpaceInvaders) {
    match getch() {
        KEY_UP if !sp.paused => {
            sp.shoot();
            sp.gen_egg();
        },
        KEY_DOWN if !sp.paused => {
            sp.gen_egg();
        },
        KEY_LEFT if !sp.paused => {
            if sp.head.0 > 0 {
                sp.head = Point(sp.head.0 - 1, sp.head.1);
            }
        },
        KEY_RIGHT if !sp.paused => {
            if sp.head.0 < ((sp.max_x - 1) as i32) {
                sp.head = Point(sp.head.0 + 1, sp.head.1);
            }
        },
        112 => {
            sp.paused = true;
        },
        114 if sp.paused => {
            sp.paused = false;
        },
        110 => {
            *sp = SpaceInvaders::new_with_bounds(sp.max_x, sp.max_y);
            sp.paused = false;
        },
        _ => {},
    }
}

fn display_space_invaders(sp: &SpaceInvaders) {
    erase();

    display_char(sp.head, 'o');
    for p in sp.body.iter() {
        display_char(*p, '+');
    }
    for p in sp.bullets.iter() {
        display_char(*p, '^');
    }
    for p in sp.eggs.iter() {
        display_char(*p, '+');
    }
}
