use rand::Rng;

const FIXED_POINTS: usize = 24;

use std::{
    cmp::Ordering,
    f64::MAX,
    io::{BufRead, Write},
    time::Instant,
};

#[derive(Clone, Debug, Default, Copy)]
struct Punto {
    pub x: f64,
    pub y: f64,
}

type BestPoint = f64;

#[derive(Clone, Debug)]
enum ExhaustiveResult {
    NotFound(BestPoint),
    Found,
    Nothing,
    NothingRecheck,
}

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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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

    pub fn start(&mut self) -> &BestPoint {
        self.divide_venceras(self.puntos[0].x, self.puntos[self.puntos.len() - 1].x);

        return &self.best_option;
    }
    fn calcula_fixed(&mut self, start: usize, end: usize) -> ExhaustiveResult {
        let mut actual_option = self.best_option;
        let mut distancia: f64;

        let mut points: [usize; 3] = [0, 0, 0];
        for (i, punto_i) in (start..end).zip(self.puntos[start..end].iter()) {
            for (j, punto_j) in (start..end).zip(self.puntos[start..end].iter()) {
                let distancia_ij = punto_i.distancia(punto_j);
                if j == i || distancia_ij >= self.best_option {
                    continue;
                }

                for (k, punto_k) in (i + 1..end).zip(self.puntos[i + 1..end].iter()) {
                    if j == k {
                        continue;
                    }

                    distancia = distancia_ij + punto_j.distancia(punto_k);

                    if distancia < actual_option {
                        actual_option = distancia;
                        points = [i, j, k];
                    }
                }
            }
        }

        if actual_option < self.best_option {
            self.best_option = actual_option;
            self.points = points;
            return ExhaustiveResult::Found;
        }

        return ExhaustiveResult::NotFound(self.best_option);
    }

    fn divide_venceras(&mut self, start: f64, end: f64) -> ExhaustiveResult {
        let (start_index, end_index) = get_points_between(self.puntos, start, end);

        if end_index - start_index + 1 < 3 || repes(self.puntos, start_index, end_index) {
            return ExhaustiveResult::Nothing;
        }

        if end_index - start_index + 1 <= FIXED_POINTS {
            return self.calcula_fixed(start_index, end_index);
        }

        let mitad: f64 = (start + end) / 2.0;
        let izq = self.divide_venceras(start, mitad);
        let drc = self.divide_venceras(mitad, end);

        let distancia_minima = match (izq, drc) {
            (ExhaustiveResult::Nothing, ExhaustiveResult::Nothing) => self.recheck(end, start),

            (ExhaustiveResult::Found, _) | (_, ExhaustiveResult::Found) => {
                self.recheck_actual_best(end, start)
            }

            _ => self.recheck_actual_best(end, start),
        };

        match distancia_minima {
            ExhaustiveResult::Found => return ExhaustiveResult::Found,
            ExhaustiveResult::NotFound(_) => return ExhaustiveResult::Nothing,
            ExhaustiveResult::NothingRecheck => {
                match self.calcula_fixed(start_index, end_index + 1) {
                    ExhaustiveResult::Found => return ExhaustiveResult::Found,
                    _ => return ExhaustiveResult::NotFound(self.best_option),
                }
            }
            _ => unreachable!(),
        }
    }

    fn recheck_actual_best(&mut self, end: f64, start: f64) -> ExhaustiveResult {
        let mitad: f64 = (start + end) / 2.0;
        if self.best_option < end - start {
            let (new_start, new_end) = get_points_between(
                self.puntos,
                mitad - self.best_option,
                mitad + self.best_option,
            );

            match self.calcula_fixed(new_start, new_end + 1) {
                ExhaustiveResult::Found | ExhaustiveResult::NotFound(_) => {
                    return ExhaustiveResult::Found
                }
                /*
                ExhaustiveResult::NotFound(_) => return ExhaustiveResult::Found,

                    if aux < self.best_option {
                        return ExhaustiveResult::NotFound(aux);
                    } else {
                        return ExhaustiveResult::Found;
                    };
                }
                    */
                _ => unreachable!(),
            };
        }

        ExhaustiveResult::NothingRecheck
    }

    fn recheck(&mut self, end: f64, start: f64) -> ExhaustiveResult {
        let mitad: f64 = (start + end) / 2.0;
        let distancia_minima = self.best_option;
        if distancia_minima < end - start {
            let (new_start, new_end) = get_points_between(
                self.puntos,
                mitad - distancia_minima,
                mitad + distancia_minima,
            );
            match self.calcula_fixed(new_start, new_end + 1) {
                ExhaustiveResult::Found => return ExhaustiveResult::Found,
                /*
                ExhaustiveResult::NotFound(aux) => {
                    if aux < distancia_minima {
                        return ExhaustiveResult::NotFound(aux);
                    } else {
                        return ExhaustiveResult::Found;
                    };
                }
                */
                _ => unreachable!(),
            };
        }

        ExhaustiveResult::NothingRecheck
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

fn repes(puntos: &[Punto], start: usize, end: usize) -> bool {
    return (start..end - 1).all(|i| puntos[i].x == puntos[i + 1].x);
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

fn read_points_from_file(file_name: &str) -> Vec<Punto> {
    let mut points = Vec::with_capacity(N_POINTS);
    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(std::fs::File::open(file_name).unwrap());

    while buffer.trim() != "NODE_COORD_SECTION" {
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        println!("{}", buffer);
    }

    while reader.read_line(&mut buffer).unwrap() != 0 {
        let values: Vec<&str> = buffer.trim().split(" ").collect();
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

fn main() {
    /*
    let mut puntos = genera_random(N_POINTS, 800.0, 0.0);
    puntos.sort();
    write_points(&puntos);
    */

    let puntos = read_points_from_file("puntos.tsp");
    println!("GO!");
    //let puntos = puntos;
    let mut media = 0;
    for _ in 0..MEDIA {
        let mut dyv = DyV::new(&puntos);
        let start = Instant::now();
        let res = dyv.start();
        let end = Instant::now();

        println!("{:?}", res);
        println!("{}", end.duration_since(start).as_millis());

        media += end.duration_since(start).as_millis();
    }
    println!("Media: {}", media / MEDIA);
}
