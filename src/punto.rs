use std::cmp::Ordering;

#[derive(Clone, Debug, Default, Copy)]
pub struct Punto {
    pub x: f64,
    pub y: f64,
}

pub type BestPoint = f64;

impl Punto {
    #[inline(always)]
    pub fn distancia(&self, a: &Punto) -> f64 {
        let diff_x = (a.x - self.x) * (a.x - self.x);
        let diff_y = (a.y - self.y) * (a.y - self.y);
        (diff_x + diff_y).sqrt()
    }

    #[allow(unused)]
    #[inline]
    pub fn distancia3(&self, a: &Punto, b: &Punto) -> f64 {
        self.distancia(a) + self.distancia(b)
    }

    pub fn total_cmp(&self, other: &Punto) -> bool {
        self.x == other.x && self.y == other.y
    }

    #[inline]
    pub fn x_comparef64(&self, other: &f64) -> Ordering {
        self.x.total_cmp(other)
            /*
        if self.x < other {
            Ordering::Less
        } else if self.x == other {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
            */
    }

    pub fn x_compare(&self, other: &Self) -> Ordering {
        self.x.total_cmp(&other.x)
    }

    pub fn x_eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl PartialOrd for Punto {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Punto {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Punto {}

impl Ord for Punto {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.total_cmp(&other.x)
    }
}
