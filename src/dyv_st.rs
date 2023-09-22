use crate::punto::*;

const FIXED_POINTS: usize = 180;
const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyV<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    best_points: [Punto; 3],
    pub fixed_points: usize,
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
        }
    }

    #[allow(unused)]
    pub fn new(puntos: &'a [Punto]) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            best_points: [Punto::default(), Punto::default(), Punto::default()],
            fixed_points: FIXED_POINTS,
        }
    }

    pub fn start(&mut self) -> BestPoint {
        self.divide_venceras(0, self.puntos);

        self.best_option
    }

    fn calcula_fixed(&mut self, slice: &[Punto]) {
        use std::cell::Cell;

        for (i, punto_i) in slice.iter().enumerate() {
            let b_option = self.best_option;
            let (_, skipped_slice) = slice.split_at(i + 1);

            for punto_j in skipped_slice
                .iter()
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

                for punto_k in skipped_slice
                    .iter()
                    .filter(|&punto_k| !punto_k.total_cmp(punto_j))
                    .filter(|&punto_k| {
                        let temp = best_y_diff.get();
                        (punto_k.y - punto_i.y).abs() < temp && (punto_k.y - punto_j.y).abs() < temp
                    })
                {
                    /*
                    if (punto_k.y - punto_j.y).abs() >= self.best_option {
                        continue;
                    }
                    */

                    let distancia_jk = punto_j.distancia(punto_k);
                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        best_y_diff.set((punto_k.y - punto_j.y).abs());
                        self.best_option = distancia_jk + distancia_ij;
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

    fn divide_venceras(&mut self, offset: usize, s_slice: &[Punto]) {
        if s_slice.len() < self.fixed_points {
            return self.calcula_fixed(s_slice);
        }

        let mitad_index = s_slice.len() / 2;
        let (first_half, second_half) =  s_slice.split_at(mitad_index);
        self.divide_venceras(offset, first_half);
        self.divide_venceras(mitad_index + offset, second_half);

        self.recheck_actual_best(offset, s_slice);
    }

    fn recheck_actual_best(&mut self, offset: usize, s_slice: &[Punto]) {
        let mitad = s_slice[s_slice.len() / 2].x;
        let (new_start, new_end) =
            self.get_points_between(mitad - self.best_option, mitad + self.best_option, s_slice);
      
        self.calcula_fixed(&s_slice[new_start..new_end + 1]);
    }

    fn get_points_between(&mut self, start: f64, end: f64, puntos: &[Punto]) -> (usize, usize) {
        let start_index = match puntos.binary_search_by(|p| p.x.partial_cmp(&start).unwrap()) {
            Ok(index) | Err(index) => index,
        };

        let end_index = match puntos[start_index..].binary_search_by(|p| p.x.partial_cmp(&end).unwrap()) {
            Ok(index) => index,
            Err(index) => index - 1,
        };

        (start_index, end_index+start_index)
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
