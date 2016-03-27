// mod snake;
// use snake::{Snake, Point};

extern crate rand;

use self::rand::Rng;

fn main() {
    // let mut game = Application::new(Snake::new());
    // game.run();
    // let mut v = Vec::new();
    // v.push(Point(1, 2));
    // println!("{:?}", v);
    // v.push(Point(1, 3));
    // println!("{:?}", v);
    // v.push(Point(1, 4));
    // println!("{:?}", v);
    //
    // // v.pop();
    // println!("{:?}", v);
    //
    // println!("{:?}", 'r' as u32);

    for _ in 0..50 {
        let ra = rand::thread_rng().gen_range(1, 6);
        print!("{:?}", ra);
    }
}
