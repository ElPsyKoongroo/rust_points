

use std::simd::{cmp::SimdPartialEq, Mask};

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
            best_option: MAX,
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
        punto_i: f64,
        mut start: usize,
    ) -> Option<usize> {

        use std::simd::{f64x4, f64x8};
        use std::simd::prelude::SimdFloat;
        //use std::simd::prelude::;
        use std::simd::cmp::SimdPartialOrd;

        use std::ops::Not;

        let vec_punto_i = f64x4::splat(punto_i);
        let vec_distancia = f64x4::splat(self.best_option);
        let mut ys: [f64; 4] = [0.0; 4];
        let zero: Mask<i64, 4> = Mask::splat(false);

        for chunk in puntos[start..].chunks_exact(4) {
            for (i, punto) in chunk.iter().enumerate() {
                ys[i] = punto.y;
            }

            let vector_puntos = f64x4::from(ys);

            let res = (vector_puntos - vec_punto_i).abs().simd_lt(vec_distancia);


            let val = res.to_bitmask();
            if val == 0 {
                start += 4;
            } else {
                //let val = res.to_bitmask();
                for bit in 0..4 {
                    let bit_val = (val >> bit) & 1;
                    if bit_val != 0 {
                        return Some(start + bit);
                    }
                }
            }
        }

        puntos[start..]
            .iter()
            .position(|punto_y| (punto_y.y - punto_i).abs() < self.best_option)
            .map(|val| val + start)
        /*
        while puntos.len() > start {
            if (puntos[start].y - punto_i).abs() < self.best_option {
                return Some(start);
            }
            start += 1;
        }
        None
        */
    }

    #[inline(always)]
    fn calcula_fixed_range(&mut self, slice: &'a [Punto], mid: usize) {
        let (f_mid, s_half) = slice.split_at(mid);
        for (i, punto_i) in f_mid.iter().enumerate() {
            let mut j = i + 1;
            while let Some(punto_j_index) = self.get_next_point(slice, punto_i.y, j) {
                let punto_j: &'a Punto = &slice[punto_j_index];

                j = punto_j_index + 1;

                if (punto_j.x - punto_i.x) >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in slice.iter().skip(i + 1) {
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

            while let Some(punto_j_index) = self.get_next_point(slice, punto_i.y, j) {
                let punto_j: &'a Punto = &slice[punto_j_index];

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
