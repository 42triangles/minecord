use std::ops;
use std::iter;
use std::usize;

use rand::prelude::*;
use rand;

use arrayvec::ArrayVec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Empty,
    Mined,
}

impl Default for Field {
    fn default() -> Self {
        Field::Empty
    }
}

#[derive(Clone, Debug)]
pub struct Minefield {
    width: usize,
    data: Vec<Field>,
}

impl Minefield {
    pub fn new(width: usize, height: usize) -> Self {
        if width.checked_mul(height).unwrap_or(usize::MAX) == usize::MAX {
            // usize::MAX is used in the implementation of `neighbours`
            panic!("Dimensional overload. Kaboom.");
        }

        Minefield {
            width,
            data: iter::repeat(Field::Empty).take(width * height).collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn pos_from_index(&self, index: usize) -> [usize;2] {
        [index % self.width, index / self.width]
    }

    fn index_from_pos(&self, pos: [usize;2]) -> usize {
        pos[0] + pos[1] * self.width
    }

    pub fn mine<R: Rng>(&mut self, rng: &mut R, amount: usize) {
        for i in rand::seq::index::sample(rng, self.data.len(), amount).into_iter() {
            self.data[i] = Field::Mined;
        }
    }

    pub fn safest_field<R: Rng>(&self, rng: &mut R) -> ([usize;2], usize) {
        rand::seq::index::sample(rng, self.data.len(), self.data.len())
            .into_iter()
            .map(|i| self.pos_from_index(i))
            .filter_map(|i| {
                self.number(i).map(|n| (i, n))
            })
            .min_by_key(|i| i.1)
            .expect("No places without mines means instadeath, ya know?")
    }

    fn valid_pos(&self, pos: [usize;2]) -> bool {
        pos[0] < self.width && pos[1] < self.height()
    }

    pub fn neighbours(&self, pos: [usize;2]) -> impl Iterator<Item = [usize;2]> {
        [usize::MAX, 0, 1]
            .iter()
            .cloned()
            .flat_map(|dx| [usize::MAX, 0, 1].iter().cloned().map(move |dy| (dx, dy)))
            .filter_map(|(dx, dy)| {
                if dx == 0 && dy == 0 {
                    return None;
                }

                let pos = [pos[0].wrapping_add(dx), pos[1].wrapping_add(dy)];

                if self.valid_pos(pos) {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect::<ArrayVec<[[usize;2];8]>>()
            .into_iter()
    }

    pub fn number(&self, pos: [usize;2]) -> Option<usize> {
        if self[pos] == Field::Mined {
            return None;
        }

        Some(
            self.neighbours(pos)
                .filter(|&p| self[p] == Field::Mined)
                .count()
        )
    }
}

impl ops::Index<[usize;2]> for Minefield {
    type Output = Field;

    fn index(&self, idx: [usize;2]) -> &Field {
        &self.data[self.index_from_pos(idx)]
    }
}

impl ops::IndexMut<[usize;2]> for Minefield {
    fn index_mut(&mut self, idx: [usize;2]) -> &mut Field {
        let idx = self.index_from_pos(idx);
        &mut self.data[idx]
    }
}
