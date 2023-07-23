use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, Copy)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    #[inline]
    pub fn distancia(&self, a: &Coord) -> f64 {
        ((a.x - self.x).powi(2) + (a.y - self.y).powi(2)).sqrt()
    }

    #[allow(unused)]
    #[inline]
    pub fn distancia3(&self, a: &Coord, b: &Coord) -> f64 {
        self.distancia(a) + self.distancia(b)
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let distancia = self.distancia(other);
        if distancia > 0.0 {
            Some(Ordering::Greater)
        } else if distancia == 0.0 {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        return self.distancia(other) == 0.0
    }
}

impl Eq for Coord {}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl Hash for Coord {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut bits = self.x.to_bits();
        // seed ^= hasher(v) + 0x9e3779b9 + (seed<<6) + (seed>>2); // Cortesia de la libreria BOOST
        bits ^= self.y.to_bits() + 0x9e3779b9 + (bits<<6);

        bits.hash(state)
    }
}
