extern crate rand;

use self::rand::Rng;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new_with_xy(x: i32, y: i32) -> Point {
        Point(x, y)
    }
}

#[derive(Debug)]
pub enum GameOver {
    Win(&'static str),
    Lose(&'static str),
}

#[derive(Debug, PartialEq)]
pub enum BodyDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct SpaceInvaders {
    pub head: Point,
    pub body: HashSet<Point>,
    pub bullets: HashSet<Point>,
    pub eggs: HashSet<Point>,
    pub body_direction: BodyDirection,
    pub max_x: u16,
    pub max_y: u16,
    pub max_bullets: u16,
    pub count_bullets: u16,
    pub paused: bool,
}

impl SpaceInvaders {
    pub fn new_with_bounds(max_x: u16, max_y: u16) -> SpaceInvaders {
        SpaceInvaders {
            head: Point::new_with_xy((max_x / 2) as i32, (max_y - 1) as i32),
            body: Self::init_body(20, 60),
            bullets: HashSet::new(),
            eggs: HashSet::new(),
            body_direction: BodyDirection::Right,
            max_x: max_x,
            max_y: max_y,
            max_bullets: 200,
            count_bullets: 0,
            paused: true,
        }
    }

    pub fn set_max_bullets(&mut self, max: u16) {
        self.max_bullets = max;
    }

    pub fn gen_egg(&mut self) {
        let index = rand::thread_rng().gen_range(0, self.body.len());
        let mut count = 0;
        for i in self.body.iter() {
            if count == index {
                self.eggs.insert(Point(i.0, 1));
                break;
            }
            count += 1;
        }
    }

    pub fn move_on(&mut self) -> Result<&'static str, GameOver> {
        self.del_body();
        self.del_eggs();

        self.new_body();
        self.new_bullets();
        self.del_eggs();
        self.new_eggs();

        if self.body.len() == 0 {
            Err(GameOver::Win("You win!!!\n"))
        } else if self.count_bullets >= self.max_bullets {
            Err(GameOver::Lose("You lose!!!\nNo bullets left...\n"))
        } else if self.eggs.contains(&self.head) {
            Err(GameOver::Lose("You lose!!!\nYou were shoot...\n"))
        }
        else {
            Ok("continue...")
        }
    }

    fn init_body(begin: u16, end: u16) -> HashSet<Point> {
        let mut body = HashSet::new();
        for i in begin..end {
            body.insert(Point(i as i32, 0));
        }
        body
    }


    fn new_body(&mut self) {
        let ra = rand::thread_rng().gen_range(1, 4);
        for _ in 0..ra {
            match self.body_direction {
                BodyDirection::Right => self.body_right(),
                BodyDirection::Left => self.body_left(),
            }
        }
    }

    fn body_right(&mut self) {
        let mut tmp_body = HashSet::new();
        for i in self.body.iter() {
            tmp_body.insert(Point(i.0 + 1, 0));
            if i.0 == ((self.max_x - 1) as i32) {
                self.body_direction = BodyDirection::Left;
            }
        }
        self.body = tmp_body;
    }

    fn body_left(&mut self) {
        let mut tmp_body = HashSet::new();
        for i in self.body.iter() {
            tmp_body.insert(Point(i.0 - 1, 0));
            if i.0 == 0 {
                self.body_direction = BodyDirection::Right;
            }
        }
        self.body = tmp_body;
    }

    fn new_bullets(&mut self) {
        let mut tmp_bullets = HashSet::new();
        {
            for i in self.bullets.iter() {
                if i.1 >= 0 {
                    let point = Point(i.0, i.1 - 1);
                    tmp_bullets.insert(point);
                }
            }
        }
        self.bullets = tmp_bullets;
    }

    fn new_eggs(&mut self) {
        let mut tmp_eggs = HashSet::new();
        {
            for i in self.eggs.iter() {
                if i.1 < (self.max_y as i32) {
                    let point = Point(i.0, i.1 + 1);
                    tmp_eggs.insert(point);
                }
            }
        }
        self.eggs = tmp_eggs;
    }

    pub fn shoot(&mut self) {
        self.bullets.insert(Point(self.head.0, self.head.1 -1));
        self.count_bullets += 1;
    }

    fn del_body(&mut self) {
        self.body = self.body.difference(&self.bullets).cloned().collect::<HashSet<_>>();
    }

    fn del_eggs(&mut self) {
        let tmp_eggs = self.eggs.clone();
        let tmp_bullets = self.bullets.clone();

        self.eggs = tmp_eggs.difference(&tmp_bullets).cloned().collect::<HashSet<_>>();
        self.bullets = tmp_bullets.difference(&tmp_eggs).cloned().collect::<HashSet<_>>();
    }
}
