#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{collections::HashSet, ops::Add};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    typ: &'static str,
    position: HashSet<Pos>,
    // 旋转锚点
    anchor: Pos,
}

macro_rules! impl_shape_constructor {
    ($( $new:ident: $typ:literal [ $( $pos_list:expr ),* ] @ $anchor:expr; )*) => {
      $(
        fn $new() -> Self {
          Self {
            typ: $typ,
            position: [ $( ($pos_list) ),* ].iter().map(|item| Pos(item.0, item.1)).collect(),
            anchor: Pos($anchor.0, $anchor.1)
          }
        }
      )*
    };
}

impl Shape {
    impl_shape_constructor! {
      new_i: "🟦" [(0, 0), (0, 1), (0, 2), (0, 3)] @ (1, 0);
      new_o: "🟨" [(0, 0), (0, 1), (1, 0), (1, 1)] @ (0, 0);
      new_t: "🟫" [(0, 0), (1, 0), (2, 0), (1, 1)] @ (0, 0);
      new_j: "🟪" [(0, 0), (0, 1), (0, 2), (-1, 2)] @ (0, 1);
      new_l: "🟧" [(0, 0), (0, 1), (0, 2), (1, 2)] @ (0, 1);
      new_s: "🟩" [(0, 0), (1, 0), (0, 1), (-1, 1)] @ (0, 0);
      new_z: "🟥" [(0, 0), (-1, 0), (0, 1), (1, 1)] @ (0, 0);
    }

    pub fn new_random() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as usize;
        [
            Self::new_i,
            Self::new_j,
            Self::new_t,
            Self::new_j,
            Self::new_l,
            Self::new_s,
            Self::new_z,
        ][random]()
    }

    // '_ bound to self left time
    pub fn position(&self) -> impl Iterator<Item = Pos> + '_ {
        self.position.iter().copied()
    }

    pub fn typ(&self) -> &'static str {
        self.typ
    }

    pub fn has_position(&self, pos: Pos) -> bool {
        self.position.contains(&pos)
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.position.intersection(&other.position).count() > 0
    }

    pub fn rotate(&self) -> Shape {
        let Pos(ax, ay) = self.anchor;
        Shape {
            typ: self.typ,
            position: self
                .position()
                .map(|Pos(x, y)| Pos(-y + ax + ay, x - ax + ay))
                .collect(),
            anchor: self.anchor,
        }
    }

    pub fn remove_line(&mut self, target_y: i32) {
        self.position = self
            .position()
            .filter(|Pos(x, y)| *y != target_y)
            .map(|mut pos| {
                if pos.1 <= target_y {
                    pos.1 += 1;
                }
                pos
            })
            .collect();
    }
}

impl Add<Pos> for &Shape {
    type Output = Shape;
    fn add(self, rhs: Pos) -> Self::Output {
        Shape {
            typ: self.typ,
            position: self.position.iter().map(|&pos| pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}
