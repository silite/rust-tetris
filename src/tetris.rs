#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::{
    direction::Direction,
    shape::{Pos, Shape},
};
use std::mem;

#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    control_shape: Shape,
    fixed_shape: Vec<Shape>,
    lost: bool,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            control_shape: Self::get_new_shape(width),
            fixed_shape: vec![],
            lost: false,
        }
    }

    fn get_new_shape(width: i32) -> Shape {
        &Shape::new_random() + Pos(width / 2 - 1, 0)
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get_target_position(&self, pos: Pos) -> Option<&'static str> {
        if self.control_shape.has_position(pos) {
            Some(self.control_shape.typ())
        } else {
            self.fixed_shape
                .iter()
                .find(|shape| shape.has_position(pos))
                .map(|shape| shape.typ())
        }
    }

    fn gen_new_control_shape(&mut self) -> Shape {
        mem::replace(&mut self.control_shape, Self::get_new_shape(self.width))
    }

    fn is_out_bound(&self, shape: &Shape) -> bool {
        !shape
            .position()
            .all(|Pos(x, y)| x >= 0 && x < self.width && y >= 0 && y < self.height)
    }

    fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shape
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    fn is_line_full(&self, target_y: i32) -> bool {
        self.fixed_shape
            .iter()
            .flat_map(|shape| shape.position())
            .filter(|Pos(x, y)| *y == target_y)
            .count() as i32
            == self.width
    }

    fn remove_line(&mut self, target_y: i32) {
        for shape in self.fixed_shape.iter_mut() {
            shape.remove_line(target_y);
        }
    }

    fn remove_full_line(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y);
            }
        }
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }

        let translated_shape = &self.control_shape + Pos(0, 1);

        if self.is_out_bound(&translated_shape) || self.is_colliding(&translated_shape) {
            let pending_fixed_shape = self.gen_new_control_shape();
            self.fixed_shape.push(pending_fixed_shape);

            self.remove_full_line();

            if self.is_colliding(&self.control_shape) {
                self.lost = true;
            }
        } else {
            self.control_shape = translated_shape;
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        let translated_shape = &self.control_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };
        if self.is_out_bound(&translated_shape) || self.is_colliding(&translated_shape) {
            return;
        }
        self.control_shape = translated_shape;
    }

    pub fn rotate(&mut self) {
        let rotate_shape = self.control_shape.rotate();
        if self.is_out_bound(&rotate_shape) || self.is_colliding(&rotate_shape) {
            return;
        }
        self.control_shape = rotate_shape;
    }
}

#[test]
fn test() {
    let mut res = Tetris::new(20, 30);
    println!("{:#?}", res);
}
