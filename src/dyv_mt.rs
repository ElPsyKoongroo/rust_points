use std::sync::RwLock;

use crate::punto::*;

const FIXED_POINTS: usize = 126;

const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyVMT<'a> {
    puntos: &'a [Punto],
    best_option: RwLock<BestPoint>,
    points: RwLock<[usize; 3]>,
    pub fixed_points: usize,
}

#[allow(unused)]
impl<'a> DyVMT<'a> {
    #[allow(unused)]
    pub fn new_with_fixed(puntos: &'a [Punto], fixed_points: usize) -> DyVMT {
        DyVMT {
            puntos,
            best_option: RwLock::new(MAX),
            points: RwLock::new([0; 3]),
            fixed_points,
        }
    }

    #[allow(unused)]
    pub fn new(puntos: &'a [Punto]) -> DyVMT {
        DyVMT {
            puntos,
            best_option: RwLock::new(MAX),
            points: RwLock::new([0; 3]),
            fixed_points: FIXED_POINTS,
        }
    }

    pub fn start(&mut self) -> BestPoint {
        let size = self.puntos.len();
        let mitad: f64 = (self.puntos[0].x + self.puntos[size - 1].x) / 2.0;

        let mitad_index = match self
            .puntos
            .binary_search_by(|p| p.x.partial_cmp(&mitad).unwrap())
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        std::thread::scope(|s| {
            let a = s.spawn(|| {
                self.divide_venceras(
                    self.puntos[0].x,
                    self.puntos[mitad_index - 1].x,
                    0,
                    &self.puntos[0..mitad_index],
                )
            });

            let b = s.spawn(|| {
                self.divide_venceras(
                    self.puntos[mitad_index].x,
                    self.puntos[size - 1].x,
                    mitad_index,
                    &self.puntos[mitad_index..size - 1],
                )
            });

            let _ = b.join();
            let _ = a.join();
        });

        /*
        self.divide_venceras(
            self.puntos[0].x,
            self.puntos[self.puntos.len() - 1].x,
            0,
            &self.puntos,
        );
        */

        let best = *self.best_option.read().unwrap();
        self.recheck_actual_best(
            self.puntos[mitad_index].x + best,
            self.puntos[mitad_index].x - best,
            0,
            self.puntos,
        );
        *self.best_option.read().unwrap()
    }

    fn calcula_fixed(&self, start: usize, end: usize) {
        let mut best_option_cache = *self.best_option.read().unwrap();
        let mut points = [0, 0, 0];

        for i in start..end {
            let punto_i = &self.puntos[i];

            for j in (start..i).chain(i + 1..end) {
                let punto_j = &self.puntos[j];
                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= best_option_cache {
                    continue;
                }

                let mut mejor = best_option_cache - distancia_ij;
                for k in i + 1..end {
                    if k == j {
                        continue;
                    }

                    let punto_k = &self.puntos[k];
                    let distancia_jk = punto_j.distancia(punto_k);

                    if distancia_jk < mejor {
                        mejor = distancia_jk;
                        points = [i, j, k];
                    }
                }
                best_option_cache = distancia_ij + mejor;
            }
        }

        let mut best_option_lock = self.best_option.write().unwrap();
        if best_option_cache < *best_option_lock {
            *best_option_lock = best_option_cache;
            *self.points.write().unwrap() = points;
        }
    }

    fn divide_venceras(&self, start: f64, end: f64, offset: usize, s_slice: &[Punto]) {
        // let (mut start_index, mut end_index) = self.get_points_between(start, end, s_slice);

        let start_index = 0;
        let end_index = s_slice.len() - 1;

        if s_slice.len() < self.fixed_points {
            return self.calcula_fixed(offset, end_index + offset + 1);
        }

        //assert!(end_index <= self.puntos.len());

        let mitad: f64 = (start + end) / 2.0;
        let mitad_index = match s_slice.binary_search_by(|p| p.x.partial_cmp(&mitad).unwrap()) {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        // let offset = start_index + offset;
        self.divide_venceras(start, mitad, offset, &s_slice[start_index..mitad_index]);

        self.divide_venceras(
            mitad,
            end,
            mitad_index + offset,
            &s_slice[mitad_index..end_index],
        );

        self.recheck_actual_best(end, start, offset, s_slice);
    }

    fn recheck_actual_best(&self, end: f64, start: f64, offset: usize, s_slice: &[Punto]) {
        let mitad: f64 = (start + end) / 2.0;
        let best_option = *self.best_option.read().unwrap();
        let (new_start, new_end) =
            self.get_points_between(mitad - best_option, mitad + best_option, s_slice);

        self.calcula_fixed(new_start + offset, new_end + offset + 1);
    }

    fn get_points_between(&self, start: f64, end: f64, puntos: &[Punto]) -> (usize, usize) {
        let start_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&start).unwrap()) {
            Ok(index) => index,
            Err(index) => index,
        };
        let end_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&end).unwrap()) {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        (start_index, end_index)
    }

    pub fn get_points(&self) -> [usize; 3] {
        *self.points.read().unwrap()
    }
}
