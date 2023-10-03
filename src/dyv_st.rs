use crate::punto::*;

const FIXED_POINTS: usize = 143;
const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyV<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    best_points: [Punto; 3],
    pub fixed_points: usize,
    f_cf: bool,
}

#[allow(unused)]
impl<'a> DyV<'a> {
    #[allow(unused)]
    pub fn new_with_fixed(puntos: &'a [Punto], fixed_points: usize) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            best_points: [Punto::default(), Punto::default(), Punto::default()],
            fixed_points,
            f_cf: false,
        }
    }

    #[allow(unused)]
    pub fn new(puntos: &'a [Punto]) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            best_points: [Punto::default(), Punto::default(), Punto::default()],
            fixed_points: FIXED_POINTS,
            f_cf: false,
        }
    }

    pub fn start(&mut self) -> BestPoint {
        self.divide_venceras(self.puntos);
        self.best_option
    }

    pub fn start_it(&mut self) -> BestPoint {
        self.divide_venceras_it();
        self.best_option
    }

    fn calcula_fixed_range(&mut self, slice: &[Punto], max: usize) {
        use std::cell::Cell;

        for (i, punto_i) in slice[..max].iter().enumerate() {
            let b_option = self.best_option;
            //let (_, slice2) = slice.split_at(i+1);

            for punto_j in slice
                .iter()
                .skip(i + 1)
                .filter(|&punto_j| (punto_j.y - punto_i.y).abs() < b_option)
            {
                if (punto_j.x - punto_i.x).abs() >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mejor = self.best_option - distancia_ij;

                let best_y_diff = Cell::new(self.best_option);

                for punto_k in slice
                    .iter()
                    .skip(i + 1)
                    .filter(|&punto_k| !punto_k.total_cmp(punto_j))
                    .filter(|&punto_k| {
                        let temp = best_y_diff.get();
                        (punto_k.y - punto_i.y).abs() < temp && (punto_k.y - punto_j.y).abs() < temp
                    })
                {
                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        distancia_jk += distancia_ij;
                        best_y_diff.set((punto_k.y - punto_j.y).abs());
                        self.best_option = distancia_jk;
                        self.best_points = [*punto_j, *punto_i, *punto_k];
                    }

                    if distancia_jik < self.best_option {
                        best_y_diff.set((punto_k.y - punto_i.y).abs());
                        self.best_option = distancia_jik;
                        self.best_points = [*punto_i, *punto_j, *punto_k];
                    }
                }
            }
        }
    }

    fn calcula_fixed(&mut self, slice: &[Punto]) {
        use std::cell::Cell;

        for (i, punto_i) in slice.iter().enumerate() {
            let b_option = self.best_option;
            //let (_, slice2) = slice.split_at(i+1);

            for punto_j in slice
                .iter()
                .skip(i + 1)
                .filter(|&punto_j| (punto_j.y - punto_i.y).abs() < b_option)
            {
                if (punto_j.x - punto_i.x).abs() >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mejor = self.best_option - distancia_ij;

                let best_y_diff = Cell::new(self.best_option);

                for punto_k in slice
                    .iter()
                    .skip(i + 1)
                    .filter(|&punto_k| !punto_k.total_cmp(punto_j))
                    .filter(|&punto_k| {
                        let temp = best_y_diff.get();
                        (punto_k.y - punto_i.y).abs() < temp && (punto_k.y - punto_j.y).abs() < temp
                    })
                {
                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        distancia_jk += distancia_ij;
                        best_y_diff.set((punto_k.y - punto_j.y).abs());
                        self.best_option = distancia_jk;
                        self.best_points = [*punto_j, *punto_i, *punto_k];
                    }

                    if distancia_jik < self.best_option {
                        best_y_diff.set((punto_k.y - punto_i.y).abs());
                        self.best_option = distancia_jik;
                        self.best_points = [*punto_i, *punto_j, *punto_k];
                    }
                }
            }
        }
    }

    fn divide_venceras_it(&mut self) {
        for chunk in self.puntos.chunks(self.fixed_points) {
            self.calcula_fixed(chunk)
        }

        for chunk in self.puntos.windows(self.fixed_points * 3).step_by(self.fixed_points*2) {
            self.recheck_actual_best(chunk)
        }
        /*
        for i in 0..(v - 2) {
            let end = (i + 2) * FIXED_POINTS;
            let slice = &self.puntos.get(self.fixed_points * i..end).unwrap();
            self.recheck_actual_best(slice)
        }
        */
    }

    fn divide_venceras(&mut self, s_slice: &[Punto]) {
        let len = s_slice.len();

        if len < self.fixed_points {
            return self.calcula_fixed(s_slice);
        }

        let mitad_index = len / 2;
        let (first_half, second_half) = s_slice.split_at(mitad_index);
        self.divide_venceras(first_half);
        self.divide_venceras(second_half);

        self.recheck_actual_best(s_slice);
    }

    fn recheck_actual_best(&mut self, s_slice: &[Punto]) {
        let mitad_index = s_slice.len() / 2;
        let mitad = s_slice[mitad_index].x;
        let (new_start, new_end) =
            Self::get_points_between(mitad - self.best_option, mitad + self.best_option, s_slice);

        self.calcula_fixed_range(&s_slice[new_start..new_end + 1], mitad_index - new_start);
    }

    fn get_points_between(start: f64, end: f64, puntos: &[Punto]) -> (usize, usize) {
        let start_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&start).unwrap()) {
            Ok(index) | Err(index) => index,
        };

        let end_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&end).unwrap()) {
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
