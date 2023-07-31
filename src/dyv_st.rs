
use crate::punto::*;

const FIXED_POINTS: usize = 180;
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
        self.divide_venceras(0, &self.puntos);

        self.best_option
    }

    fn calcula_fixed(&mut self, slice: &[Punto], start: usize, end: usize) {
        let mut k = start;
        let mut j = start;
        for (i, punto_i) in slice.iter().enumerate() {
            //let punto_i = &self.puntos[i];

            j = start;
            for punto_j in slice.iter().skip(i+1) {
                //let punto_j = &self.puntos[j];
                j += 1;


                if (punto_j.y - punto_i.y).abs() >= self.best_option {
                    continue;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in slice.iter().skip(i+1).filter(|&punto_k| punto_k != punto_j) {
                    k += 1;
                    // if punto_k == punto_j {
                    //     continue;
                    // }

                    //let punto_k = &self.puntos[k];

                    if (punto_k.y - punto_j.y).abs() >= self.best_option
                        || (punto_k.y - punto_i.y).abs() >= self.best_option
                    {
                        continue;
                    }

                    let mut distancia_jk = punto_j.distancia(punto_k);
                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
                        distancia_jk += distancia_ij;
                        self.best_option = distancia_jk;
                        self.points = [i+start, j+i, k+i];
                    }

                    if distancia_jik < self.best_option {
                        self.best_option = distancia_jik;
                        self.points = [j+i, i, k+i];
                    }
                }

                /*
                let best_jk = (i + 1..end)
                    .filter(|&n| n != j)
                    .map(|k| (punto_j.distancia(&self.puntos[k]), k))
                    .min_by(|a, b| a.0.total_cmp(&b.0))
                    .unwrap_or_else(|| (f64::MAX, 0));


                if best_jk.0 < mejor {
                    //mejor = best_jk;
                    self.best_option = distancia_ij + best_jk.0;
                    self.points = [i, j, best_jk.1];
                }
                */
            }
        }
    }

    fn divide_venceras(&mut self, offset: usize, s_slice: &[Punto]) {
        // let (mut start_index, mut end_index) = self.get_points_between(start, end, s_slice);

        let end_index = s_slice.len() - 1;

        if s_slice.len() < self.fixed_points {
            return self.calcula_fixed(s_slice, offset, end_index + offset + 1);
        }

        //assert!(end_index <= self.puntos.len());

        let mitad_index = end_index / 2;
        self.divide_venceras(offset, &s_slice[0..mitad_index]);
        self.divide_venceras(mitad_index + offset, &s_slice[mitad_index..end_index]);

        self.recheck_actual_best(offset, s_slice, s_slice[mitad_index].x);
    }

    fn recheck_actual_best(&mut self, offset: usize, s_slice: &[Punto], mitad: f64) {
        let (new_start, new_end) =
            self.get_points_between(mitad - self.best_option, mitad + self.best_option, s_slice);

        /*
        if new_end - new_start > FIXED_POINTS {
            let mitad_index = (s_slice.len() - 1) / 2;
            self.divide_venceras(offset, &s_slice[new_start..mitad_index]);
            self.divide_venceras(mitad_index + offset, &s_slice[mitad_index..new_end]);

            let (new_start, new_end) = self.get_points_between(
                mitad - self.best_option,
                mitad + self.best_option,
                s_slice,
            );

            self.calcula_fixed(new_start + offset, new_end + offset + 1);
            return;
        }
        */
        self.calcula_fixed(&s_slice[new_start..new_end+1], new_start + offset, new_end + offset + 1);
        /*
        if new_end - new_start >= FIXED_POINTS {
            self.divide_venceras(offset, &s_slice[new_start..mitad]);
            self.divide_venceras(mitad + offset, &s_slice[mitad..new_end]);
        } else {
            new_start += offset;
            new_end += offset;
            self.calcula_fixed(new_start, new_end + 1);
        }
            */
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

    pub fn get_points(&self) -> [usize; 3] {
        self.points
    }
}
