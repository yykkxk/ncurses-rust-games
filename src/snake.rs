extern crate rand;

use self::rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new_with_xy(x: i32, y: i32) -> Point {
        Point(x, y)
    }

    fn rand(width: u32, height: u32) -> Point {
        let x = rand::thread_rng().gen_range(0, width);
        let y = rand::thread_rng().gen_range(0, height);
        Point(x as i32, y as i32)
    }
}

#[derive(Debug, PartialEq)]
// #[allow(dead_code)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub enum SnakeDead {
    HitWall(&'static str),
    HitTail(&'static str),
}

#[derive(Debug)]
pub struct Snake {
    pub head: Point,
    pub body: Vec<Point>,
    pub direction: Direction,
    pub egg: Point,
    pub bounds: (u32, u32),
    pub paused: bool,
}

impl Snake {
    pub fn new_with_bounds(max_x: u32, max_y: u32) -> Snake {
        Snake {
            head: Point::new_with_xy(10, 0),
            body: Vec::new(),
            direction: Direction::Down,
            egg: Point((max_x as i32)/2 , (max_y as i32)/2),
            bounds: (max_x, max_y),
            paused: true,
        }
    }

    pub fn move_on(&mut self) -> Result<&'static str, SnakeDead> {
        if self.next_head() == self.egg {
            self.grow_once();
        } else {
            self.move_once();
        }

        if self.hit_tail() {
            Err(SnakeDead::HitTail("hit tail!!!"))
        } else if self.hit_wall() {
            Err(SnakeDead::HitWall("hit wall!!!"))
        } else {
            Ok("Ok")
        }
    }

    fn hit_tail(&self) -> bool {
        if self.body.len() > 2 && self.body.contains(&self.head) {
            true
        } else {
            false
        }
    }

    fn hit_wall(&self) -> bool {
        let Point(x, y) = self.head;
        if x < 0 || y < 0 || x >= (self.bounds.0 as i32) || y >= (self.bounds.1 as i32) {
            true
        } else {
            false
        }
    }

    fn next_head(&self) -> Point {
        match self.direction {
            Direction::Down => Point(self.head.0, self.head.1 + 1),
            Direction::Up => Point(self.head.0, self.head.1 - 1),
            Direction::Left => Point(self.head.0 - 1, self.head.1),
            Direction::Right => Point(self.head.0 + 1, self.head.1),
        }
    }

    fn next_egg(&self) -> Point {
        let mut new_egg;
        loop {
            new_egg = Point::rand(self.bounds.0, self.bounds.1);
            if new_egg != self.head || !self.body.contains(&new_egg) {
                return new_egg;
            }
        }
    }

    fn grow_once(&mut self) {
        self.body.push(self.head);
        self.head = self.egg;

        self.egg = self.next_egg();
    }

    fn move_once(&mut self) {
        self.body.push(self.head);
        self.head = self.next_head();

        // let ln = self.body.len();
        self.body.remove(0);
        // self.body.pop();
    }
}
