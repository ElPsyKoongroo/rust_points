use rand::Rng;

const FIXED_POINTS: usize = 30;
const POINT_FILES: &str = "point_files/";

use std::{
    cmp::Ordering,
    io::{BufRead, Write},
    path::{Path, PathBuf},
    time::Instant,
};

const MAX: f64 = f64::MAX;

#[derive(Clone, Debug, Default, Copy)]
struct Punto {
    pub x: f64,
    pub y: f64,
}

type BestPoint = f64;

impl Punto {
    #[inline]
    fn distancia(&self, a: &Punto) -> f64 {
        ((a.x - self.x).powi(2) + (a.y - self.y).powi(2)).sqrt()
    }

    #[allow(unused)]
    #[inline]
    pub fn distancia3(&self, a: &Punto, b: &Punto) -> f64 {
        self.distancia(a) + self.distancia(b)
    }
}

impl PartialOrd for Punto {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
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

struct DyV<'a> {
    puntos: &'a [Punto],
    best_option: BestPoint,
    points: [usize; 3],
}

impl<'a> DyV<'a> {
    pub fn new(puntos: &'a [Punto]) -> DyV {
        DyV {
            puntos,
            best_option: MAX,
            points: [0; 3],
        }
    }

    pub fn start(&mut self) -> BestPoint {
        self.divide_venceras(self.puntos[0].x, self.puntos[self.puntos.len() - 1].x);

        self.best_option
    }

    fn calcula_fixed(&mut self, start: usize, end: usize) {
        // let mut distancia: f64;
        for i in start..end {
            let punto_i = &self.puntos[i];

            for j in start..end {
                if j == i {
                    continue;
                }

                let punto_j = &self.puntos[j];
                let distancia_ij = punto_i.distancia(punto_j);

                if distancia_ij >= self.best_option {
                    continue;
                }

                for k in i + 1..end {
                    if k == j {
                        continue;
                    }

                    let punto_k = &self.puntos[k];
                    let distancia = distancia_ij + punto_j.distancia(punto_k);

                    if distancia < self.best_option {
                        self.best_option = distancia;
                        self.points = [i, j, k];
                    }
                }
            }
        }

        /*
        if actual_option < self.best_option {
            self.best_option = actual_option;
            self.points = points;
        }
        */
    }

    fn divide_venceras(&mut self, start: f64, end: f64) {
        let (start_index, end_index) = get_points_between(self.puntos, start, end);

        if end_index - start_index < FIXED_POINTS {
            self.calcula_fixed(start_index, end_index + 1);
            return;
        }

        let mitad: f64 = (start + end) / 2.0;
        self.divide_venceras(start, mitad);
        self.divide_venceras(mitad, end);

        if self.best_option < end - start {
            self.recheck_actual_best(end, start);
        } else {
        };
    }

    fn recheck_actual_best(&mut self, end: f64, start: f64) {
        let mitad: f64 = (start + end) / 2.0;
        let (new_start, new_end) = get_points_between(
            self.puntos,
            mitad - self.best_option,
            mitad + self.best_option,
        );

        self.calcula_fixed(new_start, new_end + 1);
    }
}

fn get_points_between(puntos: &[Punto], start: f64, end: f64) -> (usize, usize) {
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

#[allow(unused)]
fn genera_random(num_puntos: usize, upper_bound: f64, lower_bound: f64) -> Vec<Punto> {
    let mut puntos = Vec::with_capacity(num_puntos);
    for _ in 0..num_puntos {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(lower_bound..upper_bound);
        let y: f64 = rng.gen_range(lower_bound..upper_bound);
        puntos.push(Punto { x, y })
    }
    puntos
}

#[allow(unused)]
fn write_points(puntos: &[Punto]) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("puntos.tsp").unwrap());
    file.write_all("NODE_COORD_SECTION\n".as_bytes()).unwrap();

    for punto in puntos {
        file.write_all(format!("1 {} {}\n", punto.x, punto.y).as_bytes())
            .unwrap();
    }
}

fn read_points_from_file<I: AsRef<Path>>(file_name: I) -> Vec<Punto> {
    let mut points = Vec::with_capacity(N_POINTS);
    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(std::fs::File::open(file_name).unwrap());

    while buffer.trim() != "NODE_COORD_SECTION" {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
    }

    while reader.read_line(&mut buffer).unwrap() != 0 {
        let values: Vec<&str> = buffer.trim().split(' ').collect();
        points.push(Punto {
            x: values[1].parse().unwrap(),
            y: values[2].parse().unwrap(),
        });
        buffer.clear()
    }

    points
}

static N_POINTS: usize = 800_000;
static MEDIA: u128 = 20;

#[allow(unused)]
fn write_points_with_name<I: AsRef<Path>>(name: I, puntos: &[Punto]) {
    let mut file = std::io::BufWriter::new(std::fs::File::create(name).unwrap());
    file.write_all("NODE_COORD_SECTION\n".as_bytes()).unwrap();

    for punto in puntos {
        file.write_all(format!("1 {} {}\n", punto.x, punto.y).as_bytes())
            .unwrap();
    }
}

fn bench() {
    let mut puntos = read_points_from_file(PathBuf::from(POINT_FILES).join("puntos_800000.tsp"));
    puntos.sort();
    println!("GO!");
    let mut media = 0;
    for _ in 0..MEDIA {
        let mut dyv = DyV::new(&puntos);

        let start = Instant::now();
        let res = dyv.start();
        let end = Instant::now();

        println!("{:?}, {:?}", res, dyv.points);
        println!("{}", end.duration_since(start).as_millis());

        media += end.duration_since(start).as_millis();
    }
    println!("Media: {} ms", media / MEDIA);
}

fn main() {
    // for i in 0..10 {
    //     let puntos = read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
    //     let mut dyv = DyV::new(&puntos);
    //     let res = dyv.start();
    //     println!("{:?}", res);
    // }

    bench()
}

#[cfg(test)]
mod tests {
    use super::read_points_from_file;
    use super::DyV;

    #[test]
    fn test_1() {
        let puntos = read_points_from_file("point_files/puntos_800000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.03781606866923333);
    }

    #[test]
    fn test_2() {
        let puntos = read_points_from_file("point_files/puntos_500000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.05537844995897111);
    }

    // [30_000, 50_000, 80_000, 150_000,
    #[test]
    fn test_3() {
        let puntos = read_points_from_file("point_files/puntos_150000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.144052106804805);
    }
    #[test]
    fn test_4() {
        let puntos = read_points_from_file("point_files/puntos_80000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.322497589363708);
    }

    #[test]
    fn test_5() {
        let puntos = read_points_from_file("point_files/puntos_50000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.30805303890637226);
    }
    #[test]
    fn test_6() {
        let puntos = read_points_from_file("point_files/puntos_30000.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 0.27095504920223346);
    }

    #[test]
    fn random_tests_part_1() {
        let answers = [
            0.08015304030013183,
            0.05353786710188051,
            0.08537619304985208,
            0.05132736906745347,
            0.02924335899771857,
        ];

        let mut threads = vec![];

        for i in 0..5 {
            let t = std::thread::spawn(move || {
                let puntos = read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
                let mut dyv = DyV::new(&puntos);
                let res = dyv.start();
                assert_eq!(res, answers[i]);
            });

            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }
    }

    #[test]
    fn random_tests_part_2() {
        let answers = [
            0.045820223238880894,
            0.04419541844017895,
            0.0990423678758528,
            0.05702137712944691,
            0.033750012246621455,
        ];

        let mut threads = vec![];
        for i in 5..10 {
            threads.push(std::thread::spawn(move || {
                let puntos = read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
                let mut dyv = DyV::new(&puntos);
                let res = dyv.start();
                assert_eq!(res, answers[i - 5]);
            }));
        }

        threads.into_iter().for_each(|t| {
            t.join().unwrap();
        });
    }

    #[test]
    fn d657() {
        let puntos = read_points_from_file("point_files/d657.tsp");
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        assert_eq!(res, 35.9210244842761);
    }
}
