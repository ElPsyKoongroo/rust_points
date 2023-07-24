use crate::punto::*;

const FIXED_POINTS: usize = 126;
const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyV<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    points: [usize; 3],
    pub fixed_points: usize,
}

#[allow(unused)]
impl<'a> DyV<'a> {
    #[allow(unused)]
    pub fn new_with_fixed(puntos: &'a [Punto], fixed_points: usize) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            points: [0; 3],
            fixed_points,
        }
    }

    #[allow(unused)]
    pub fn new(puntos: &'a [Punto]) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            points: [0; 3],
            fixed_points: FIXED_POINTS,
        }
    }

    pub fn start(&mut self) -> BestPoint {
        self.divide_venceras(
            self.puntos[0].x,
            self.puntos[self.puntos.len() - 1].x,
            0,
            &self.puntos,
        );

        self.best_option
    }

    fn calcula_fixed(&mut self, start: usize, end: usize) {
        for i in start..end {
            let punto_i = &self.puntos[i];

            for j in (start..i).chain(i + 1..end) {
                let punto_j = &self.puntos[j];
                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for k in i + 1..end {
                    if k == j {
                        continue;
                    }

                    let punto_k = &self.puntos[k];
                    let distancia_jk = punto_j.distancia(punto_k);

                    if distancia_jk < mejor {
                        mejor = distancia_jk;
                        self.points = [i, j, k];
                    }
                }
                self.best_option = distancia_ij + mejor;
            }
        }
    }

    fn divide_venceras(&mut self, start: f64, end: f64, offset: usize, s_slice: &[Punto]) {
        // let (mut start_index, mut end_index) = self.get_points_between(start, end, s_slice);

        let start_index = 0;
        let end_index = s_slice.len() - 1;

        if s_slice.len() < self.fixed_points {
            return self.calcula_fixed(offset, end_index + offset + 1);
        }

        assert!(end_index <= self.puntos.len());

        let mitad: f64 = (start + end) / 2.0;
        let mitad_index = match s_slice.binary_search_by(|p| p.x.partial_cmp(&mitad).unwrap()) {
            Ok(index) => index,
            Err(index) => index,
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

    fn recheck_actual_best(&mut self, end: f64, start: f64, offset: usize, s_slice: &[Punto]) {
        let mitad: f64 = (start + end) / 2.0;
        let (mut new_start, mut new_end) =
            self.get_points_between(mitad - self.best_option, mitad + self.best_option, s_slice);

        new_start += offset;
        new_end += offset;

        self.calcula_fixed(new_start, new_end + 1);
    }

    fn get_points_between(&mut self, start: f64, end: f64, puntos: &[Punto]) -> (usize, usize) {
        let start_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&start).unwrap()) {
            Ok(index) => index,
            Err(index) => index,
        };
        let end_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&end).unwrap()) {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        return (start_index, end_index);
    }
}


