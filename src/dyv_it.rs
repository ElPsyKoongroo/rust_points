use crate::punto::*;

const FIXED_POINTS: usize = 108;
const MAX: f64 = f64::MAX;

#[allow(unused)]
pub struct DyVIT<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    best_points: [Punto; 3],
    pub fixed_points: usize,
    f_cf: bool,
}

#[allow(unused)]
impl<'a> DyVIT<'a> {
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
        puntos: &mut impl Iterator<Item = &'a Punto>,
        punto_i: &'a Punto,
        target: f64,
    ) -> Option<&'a Punto> {
        puntos.find(|sig| (sig.y - punto_i.y).abs() < target)
    }

    #[inline(always)]
    fn calcula_fixed_range(&mut self, slice: &'a [Punto], mid: usize) {
        let (f_mid, s_half) = slice.split_at(mid);
        for (i, punto_i) in f_mid.iter().enumerate() {

            let mut j_iter = slice[i + 1..].iter();
            while let Some(punto_j) = Self::get_next_point(&mut j_iter, punto_i, self.best_option)
            {
                if (punto_j.x - punto_i.x) >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in slice
                    .iter()
                    .skip(i + 1)
                    .filter(|punto_k| !punto_k.total_cmp(punto_j))
                {
                    if (punto_k.y - punto_i.y).abs() >= self.best_option && (punto_k.y - punto_j.y).abs() >= self.best_option {
                        continue;
                    }

                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
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
            let mut j_slice: &'a [Punto] = slice.get(i + 1..).unwrap();
            let mut j_iter = j_slice.iter();

            let max_x = punto_i.x + self.best_option;

            while let Some(punto_j) = Self::get_next_point(&mut j_iter, punto_i, self.best_option) 
            {
                if (punto_j.x - punto_i.x) >= self.best_option {
                    break;
                }

                let distancia_ij = punto_i.distancia(punto_j);

                
                if distancia_ij >= self.best_option {
                    continue;
                }

                let mut mejor = self.best_option - distancia_ij;

                for punto_k in slice
                    .iter()
                    .skip(i + 1)
                    .filter(|punto_k| !punto_k.total_cmp(punto_j))
                {
                    if (punto_k.y - punto_i.y).abs() >= self.best_option && (punto_k.y - punto_j.y).abs() >= self.best_option {
                        continue;
                    }

                    let mut distancia_jk = punto_j.distancia(punto_k);

                    let distancia_jik = distancia_ij + punto_i.distancia(punto_k);

                    if distancia_jk < mejor {
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
            self.calcula_fixed(chunk)
        }

        // Merge respuestas
        let mut start = 0;
        while start < self.puntos.len() {
            let mut end = start+self.fixed_points*2;
            if start+self.fixed_points*2 > self.puntos.len() {
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
