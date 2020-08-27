#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;
use user_lib::console::getchar;
use alloc::{collections::VecDeque,vec::*,string::String};
use user_lib::sys_exit;

const W:u8=0x77u8;
const S:u8=0x73u8;
const D:u8=0x64u8;
const A:u8=0x61u8;
const Q:u8=0x71u8;
pub static mut count:usize=10;
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let res = alloc::fmt::format(alloc::__export::format_args!($($arg)*));
        res
    }}
}

pub struct Game {
    snake: Snake,
    food_pos: (u32, u32),
    ended: bool,
    pub score: u32,
}

pub struct Snake {
    direction: Direction,
    parts: Vec<(u32, u32)>,
    ordered_parts: VecDeque<(u32, u32)>,
}
#[derive(PartialEq, Copy, Clone)]

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub enum Move {
    Ok,
    Crash,
}


#[no_mangle]
pub fn main()->usize{
	println!("hello");
	print!("{}[2J", 27 as char);
	let mut game = Game::new();
        game.start();
	0
}


impl Game {
    pub fn new() -> Self {
	let mut initial_snake_parts = Vec::<(u32, u32)>::new();
        initial_snake_parts.push((0, 0));
        initial_snake_parts.push((1, 0));
        initial_snake_parts.push((2, 0));

        let mut initial_ordered_snake_parts = VecDeque::<(u32, u32)>::new();
        initial_ordered_snake_parts.push_back((0, 0));
        initial_ordered_snake_parts.push_back((1, 0));
        initial_ordered_snake_parts.push_back((2, 0));

        Self{
            snake: Snake {
                direction: Direction::Right,
                parts: initial_snake_parts,
                ordered_parts: initial_ordered_snake_parts,
            },
            food_pos: (5, 0),
            ended: false,
            score: 0,
        }
    }
    pub fn draw(self: &mut Game) {
	print!("\x1b[H");
	let terminal_size=(71,33);
        let real_terminal_size=(143, 34);
	let mut frame = Vec::<u8>::new();
	let right_side_padding = &" ".repeat((real_terminal_size.0 - terminal_size.0 * 2) as usize);
        
	for y in 0..terminal_size.1 {
            for x in 0..terminal_size.0 {
                if self.snake.parts.contains(&(x, y)) {
                    frame.extend_from_slice(b"\x1b[97m\x1b[107m  \x1b[0m"); // A white square
                    continue;
                }

                if (x, y) == self.food_pos {
                    frame.extend_from_slice(b"\x1b[92m\x1b[102m  \x1b[0m"); // A light-green square
                } else {
                    frame.extend_from_slice(b"  ");
                }
            }
            frame.extend_from_slice(right_side_padding.as_bytes());
        }

        // Add the status line at the bottom
        let status_text = format!("Score: {}", self.score);
        frame.extend_from_slice(b"\x1b[104m\x1b[30m");
        frame.extend_from_slice(
            " ".repeat(
                ((real_terminal_size.0 as usize - status_text.len()) as f64 / 2f64)
                    as usize,
            )
            .as_bytes(),
        );
        frame.extend_from_slice(status_text.as_bytes());
        frame.extend_from_slice(
            " ".repeat(
                ((real_terminal_size.0 as usize - status_text.len()) as f64 / 2f64) as usize,
            )
            .as_bytes(),
        );
        frame.extend_from_slice(b"\x1b[0m");
        print!("{}", String::from_utf8(frame).unwrap());
    }
    
    fn generate_food_pos(self: &mut Game) -> (u32, u32) {
        let terminal_size = (71,33);
        loop {
            let food_pos: (u32, u32) = (
               	rand(self.food_pos)
            );

            if self.snake.parts.contains(&food_pos) {
                continue;
            }
            return food_pos;
        }
    }
    fn move_snake(self: &mut Game) -> Move {
        let terminal_size=(71,33);
	let mut new_head_pos = *self.snake.ordered_parts.back().unwrap();
        let (dx, dy) = match self.snake.direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        };
        let width = terminal_size.0 as i16;
        let height = terminal_size.1 as i16;
        new_head_pos.0 = (((new_head_pos.0 as i16 + dx) + width) % width) as u32;
        new_head_pos.1 = (((new_head_pos.1 as i16 + dy) + height) % height) as u32;

        // If the head is on food, eat it
        if *self.snake.ordered_parts.back().unwrap() == self.food_pos {
            self.score += 1;
            self.food_pos = self.generate_food_pos();
        } else {
            // Only remove the last part if no food was eaten
            let last_part_pos = self.snake.ordered_parts.pop_front().unwrap();
            self.snake.parts.remove(self.snake.parts.iter().position(|(x,y)| *x==last_part_pos.0 && *y==last_part_pos.1).unwrap());
	}

        // See if the snake crashed
        if self.snake.parts.contains(&new_head_pos) {
            print!("{}", 7 as char);
            return Move::Crash;
        }

        self.snake.ordered_parts.push_back(new_head_pos);
        self.snake.parts.push(new_head_pos);
        Move::Ok
    }
    pub fn start(self: &mut Game) {
	
	loop {
            self.handle_input();
	    self.draw();
	    //sleep();
	}
    }
    fn handle_input(self: &mut Game) {
        let mut new_direction = self.snake.direction;
	let c=getchar();
        match c{
              W|S|D|A=>{
		if c==W{
			new_direction = Direction::Up;	
		}else if c==S{
			new_direction = Direction::Down;
		}else if c==D{
			new_direction = Direction::Right;
		}else{
			new_direction = Direction::Left;
		}
              }
              Q=>{
              	sys_exit(0);
              }
               _=>{}
        }
        self.snake.direction = new_direction;
        if let Move::Crash = self.move_snake() {
            self.ended = true;
        }

    }
}

pub fn sleep(){
	for i in 0..10000{}
}

pub fn rand(seed:(u32,u32))->(u32,u32){
	let terminal_size = (71,33);
	let r1:u32 = seed.0 * 110351 + 12345;
	let r2:u32 = seed.0 * 110351 + 12345;
	let r1r:u32=(r1 << 16) | ((r1 >> 16) & 0xFFFF);
	let r2r:u32=(r2<<16 | ((r2>>16)&0xFFFF));
	return (r1r%terminal_size.0+1,r2r%terminal_size.1+1);
}
