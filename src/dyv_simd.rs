use crate::punto::*;

const FIXED_POINTS: usize = 98;
const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyVSIMD<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    best_points: [Punto; 3],
    pub fixed_points: usize,
    f_cf: bool,
}

#[allow(unused)]
impl<'a> DyVSIMD<'a> {
    #[allow(unused)]
    pub fn new_with_fixed(puntos: &'a [Punto], fixed_points: usize) -> Self {
        Self {
            puntos,
            best_option: MAX,
            best_points: [Punto::default(), Punto::default(), Punto::default()],
            fixed_points,
            f_cf: false,
        }
    }

    #[allow(unused)]
    pub fn new(puntos: &'a [Punto]) -> Self {
        Self {
            puntos,
            best_option: puntos[0].distancia3(&puntos[1], &puntos[2]),
            best_points: [Punto::default(), Punto::default(), Punto::default()],
            fixed_points: FIXED_POINTS,
            f_cf: false,
        }
    }

    pub fn start(&mut self) -> BestPoint {
        self.divide_venceras_it();
        self.best_option
    }

    #[inline]
    fn get_next_point(
        &'a self,
        puntos: &'a [Punto],
        punto_i: &'a Punto,
        mut start: usize,
    ) -> Option<usize> {
        use std::ops::Not;
        use std::simd::cmp::SimdPartialOrd;
        use std::simd::prelude::SimdFloat;
        use std::simd::{f64x4, f64x8};

        let vec_punto_i = f64x4::splat(punto_i.y);
        let vec_distancia = f64x4::splat(self.best_option);

        let mut it = puntos[start..].array_chunks::<8>();
        let max_x = punto_i.x + self.best_option;

        while let Some(chunk) = it.next() {
            //for chunk in it.by_ref() {
            if chunk[0].x >= max_x {
                return None;
            }

            let vector_y = f64x4::from_array([chunk[0].y, chunk[1].y, chunk[2].y, chunk[3].y]);
            let res = (vector_y - vec_punto_i).abs().simd_le(vec_distancia);
            match res.first_set() {
                None => start += 4,
                Some(i) => return Some(start + i),
            }
            // Unroll

            let vector_y = f64x4::from_array([chunk[4].y, chunk[5].y, chunk[6].y, chunk[7].y]);
            let res = (vector_y - vec_punto_i).abs().simd_le(vec_distancia);
            match res.first_set() {
                None => start += 4,
                Some(i) => return Some(start + i),
            }
        }

        it.remainder()
            .iter()
            .position(|punto_y| {
                (punto_y.y - punto_i.y).abs() < self.best_option
                    && (punto_y.x - punto_i.x).abs() < self.best_option
            })
            .map(|val| val + start)
    }

    #[inline(always)]
    fn calcula_fixed_range(&mut self, slice: &'a [Punto], mid: usize) {
        let (f_mid, s_half) = slice.split_at(mid);
        for (i, punto_i) in f_mid.iter().enumerate() {
            let mut j = i + 1;

            while let Some(punto_j_index) = self.get_next_point(slice, punto_i, j) {
                let punto_j: &'a Punto = &slice[punto_j_index];

                j = punto_j_index + 1;

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in s_half.iter() {
                    if (punto_k.y - punto_i.y).abs() >= self.best_option
                        && (punto_k.y - punto_j.y).abs() >= self.best_option
                    {
                        continue;
                    }

                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        if punto_k.x_eq(punto_j) {
                            continue;
                        }
                        distancia_jk += distancia_ij;
                        self.best_option = distancia_jk;
                        mejor = self.best_option - distancia_ij;
                        self.best_points = [*punto_j, *punto_i, *punto_k];
                    }

                    if distancia_jik < self.best_option {
                        self.best_option = distancia_jik;
                        self.best_points = [*punto_i, *punto_j, *punto_k];
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn calcula_fixed(&mut self, slice: &'a [Punto]) {
        let mut i = 0;
        for punto_i in slice.iter() {
            let mut j = i + 1;

            while let Some(punto_j_index) = self.get_next_point(slice, punto_i, j) {
                let punto_j: &'a Punto = unsafe { slice.get_unchecked(punto_j_index) }; //&slice[punto_j_index];

                j = 1 + punto_j_index;

                if (punto_j.x - punto_i.x) >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in slice.iter().skip(i + 1)
                //.filter(|punto_k| !punto_k.x_eq(punto_j))
                {
                    if (punto_k.y - punto_i.y).abs() >= self.best_option
                        && (punto_k.y - punto_j.y).abs() >= self.best_option
                    {
                        continue;
                    }

                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        if punto_k.x_eq(punto_j) {
                            continue;
                        }
                        distancia_jk += distancia_ij;
                        self.best_option = distancia_jk;
                        mejor = self.best_option - distancia_ij;
                        self.best_points = [*punto_j, *punto_i, *punto_k];
                    }

                    if distancia_jik < self.best_option {
                        self.best_option = distancia_jik;
                        self.best_points = [*punto_i, *punto_j, *punto_k];
                    }
                }
            }
            i += 1;
        }
    }

    fn divide_venceras_it(&mut self) {
        let v = self.puntos.len() / self.fixed_points;

        for chunk in self.puntos.chunks(self.fixed_points) {
            //let end = (i + 1) * self.fixed_points;
            //let slice = &self.puntos.get(self.fixed_points * i..end).unwrap();
            self.calcula_fixed(chunk)
        }

        // Merge respuestas
        let mut start = 0;
        while start < self.puntos.len() {
            let mut end = start + self.fixed_points * 2;
            if start + self.fixed_points * 2 > self.puntos.len() {
                end = self.puntos.len()
            }
            let slice = self.puntos.get(start..end).unwrap();
            self.recheck_actual_best(slice);
            start += self.fixed_points
        }
    }

    fn recheck_actual_best(&mut self, s_slice: &'a [Punto]) {
        let mitad_index = s_slice.len() / 2;
        let mitad = s_slice[mitad_index].x;
        let (new_start, new_end) =
            Self::get_points_between(mitad - self.best_option, mitad + self.best_option, s_slice);

        let mid = mitad_index - new_start;
        self.calcula_fixed_range(&s_slice[new_start..new_end + 1], mid);
    }

    fn get_points_between(start: f64, end: f64, puntos: &[Punto]) -> (usize, usize) {
        let start_index = match puntos.binary_search_by(|p| p.x_comparef64(&start)) {
            Ok(index) | Err(index) => index,
        };

        let end_index = match puntos.binary_search_by(|p| p.x_comparef64(&end)) {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        (start_index, end_index)
    }

    pub fn get_points(&self) -> [usize; 3] {
        let mut indexes = [0; 3];
        for (i, point) in self.best_points.iter().enumerate() {
            // SAFETY:
            // El punto que estoy buscando siempre va a existir
            let index = self.puntos.binary_search_by(|p| p.cmp(point)).unwrap();
            indexes[i] = index;
        }
        indexes
    }
}
