use rand::Rng;

use std::{
    cmp::Ordering,
    f64::MAX,
    io::{BufRead, Read, Write},
    time::Instant,
};

#[derive(Clone, Debug, Default, Copy)]
struct Punto {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone)]
struct BestPoint {
    distancia: f64,
    points: [usize; 3],
}

#[derive(Clone, Debug)]
enum ExhaustiveResult {
    NotFound(BestPoint),
    Found,
    Nothing,
    NothingRecheck(BestPoint),
}

impl Punto {
    #[inline]
    fn distancia(&self, a: &Punto) -> f64 {
        ((a.x - self.x).powi(2) + (a.y - self.y).powi(2)).sqrt()
    }

    #[inline]
    pub fn distancia3(&self, a: &Punto, b: &Punto) -> f64 {
        self.distancia(a) + self.distancia(b)
    }
}

impl PartialOrd for Punto {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.x.partial_cmp(&other.x)
        //Some(self.cmp(other))
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

fn calcula_fixed(
    puntos: &[Punto],
    start: usize,
    end: usize,
    best_option: &mut BestPoint,
) -> ExhaustiveResult {
    if end - start + 1 < 3 {
        return ExhaustiveResult::Nothing;
    }
    let end = end + 1;

    let mut actual_option = BestPoint {
        distancia: MAX,
        points: [0; 3],
    };
    /*
    let mut i = start;
    let mut j = start;
    let mut k = start;

    while i < end {
        j = start;
        while j < end {
            if j == i {
                j += 1;
                continue;
            }

            k = start;
            while k < end {
                if i >= k || j == k {
                    k += 1;
                    continue;
                }
                let distancia: f64 = puntos[i].distancia3(&puntos[j], &puntos[k]);

                if distancia < actual_option.distancia {
                    actual_option.distancia = distancia;
                    if distancia < best_option.distancia {
                        (*best_option).distancia = distancia;
                        (*best_option).points = [i,j,k];
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }*/
    
    let mut distancia;
    for i in start..end {
        for j in start..end {
            if j == i {
                continue;
            }

            for k in (i + 1)..end {
                if j == k {
                    continue;
                }

                distancia = puntos[i].distancia3(&puntos[j], &puntos[k]);

                if distancia < actual_option.distancia {
                    actual_option.distancia = distancia;
                    actual_option.points = [i, j, k];
                }
            }
        }
    }
    if actual_option.distancia < best_option.distancia {
        (*best_option).distancia = actual_option.distancia;
        (*best_option).points = actual_option.points;
        return ExhaustiveResult::Found;
    }

    return ExhaustiveResult::NotFound(actual_option); //best_option.clone();
}

fn get_points_between(puntos: &[Punto], start: f64, end: f64) -> (usize, usize) {
    let start_index = puntos
        .iter()
        .position(|&p| p.x > start)
        .unwrap_or_else(|| N_POINTS); // 722
    let end_index = puntos.iter().rposition(|&p| p.x < end).unwrap();
    (start_index, end_index)
}

fn divide_benceras(puntos: &[Punto]) -> BestPoint {
    let mut best_points = BestPoint {
        distancia: MAX,
        points: [0; 3],
    };
    divide_venceras(
        puntos,
        puntos[0].x,
        puntos[puntos.len() - 1].x,
        &mut best_points,
    );

    return best_points;
}

fn repes(puntos: &[Punto], start: usize, end: usize) -> bool {
    return !(start..end - 1).any(|i| puntos[i].x != puntos[i + 1].x);
}

fn divide_venceras(
    puntos: &[Punto],
    start: f64,
    end: f64,
    best_option: &mut BestPoint,
) -> ExhaustiveResult {
    let mitad: f64 = (start + end) / 2.0;
    let (start_index, end_index) = get_points_between(puntos, start, end);

    /*
    if end_index - start_index + 1 < 3 {
        return ExhaustiveResult::Nothing;
    }
    */

    if repes(puntos, start_index, end_index) {
        return ExhaustiveResult::Nothing;
    }

    if end_index - start_index + 1 <= 6 {
        return calcula_fixed(puntos, start_index, end_index, best_option);
    }

    let izq = divide_venceras(puntos, start, mitad, best_option);
    let drc = divide_venceras(puntos, mitad, end, best_option);

    let distancia_minima = match (izq, drc) {
        (ExhaustiveResult::Nothing, ExhaustiveResult::Nothing) => {
            return calcula_fixed(puntos, start_index, end_index, best_option)
        }
        (ExhaustiveResult::Found, _) => recheck(puntos, best_option, end, start, mitad), //best_option.clone(),
        (_, ExhaustiveResult::Found) => recheck(puntos, best_option, end, start, mitad),
        (ExhaustiveResult::NotFound(mut r_izq), ExhaustiveResult::NotFound(mut r_drc)) => {
            if r_izq.distancia < r_drc.distancia {
                recheck(puntos, &mut r_izq, end, start, mitad)
            } else {
                recheck(puntos, &mut r_drc, end, start, mitad)
            }
        }
        _ => recheck(puntos, best_option, end, start, mitad),
    };

    match distancia_minima {
        ExhaustiveResult::Found => return ExhaustiveResult::Found,
        ExhaustiveResult::NotFound(a) => return ExhaustiveResult::NotFound(a),
        ExhaustiveResult::NothingRecheck(a) => {
            match calcula_fixed(puntos, start_index, end_index, best_option) {
                ExhaustiveResult::Found => return ExhaustiveResult::Found,
                ExhaustiveResult::NotFound(aux) => {
                    if aux.distancia < a.distancia {
                        return ExhaustiveResult::NotFound(aux);
                    } else {
                        return ExhaustiveResult::NotFound(a);
                    };
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn recheck(
    puntos: &[Punto],
    distancia_minima: &mut BestPoint,
    end: f64,
    start: f64,
    mitad: f64,
) -> ExhaustiveResult {
    if distancia_minima.distancia < end - start {
        let (new_start, new_end) = get_points_between(
            puntos,
            mitad - distancia_minima.distancia,
            mitad + distancia_minima.distancia,
        );

        match calcula_fixed(puntos, new_start, new_end, distancia_minima) {
            ExhaustiveResult::Found => return ExhaustiveResult::Found,
            ExhaustiveResult::NotFound(aux) => {
                if aux.distancia < distancia_minima.distancia {
                    return ExhaustiveResult::NotFound(aux);
                } else {
                    return ExhaustiveResult::Found;
                };
            }
            _ => unreachable!(),
        };
    }

    ExhaustiveResult::NothingRecheck(distancia_minima.clone())
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

static N_POINTS: usize = 30_000;
static MEDIA: u128 = 30;

fn main() {
    //let mut puntos = genera_random(N_POINTS, 800.0, 0.0);
    //puntos.sort();
    //write_points(&puntos);
    let puntos = read_points_from_file("puntos.tsp");
    println!("GO!");
    let puntos = puntos;
    let mut media = 0;
    for _ in 0..MEDIA {
        let start = Instant::now();
        let res = divide_benceras(&puntos);
        let end = Instant::now();

        println!("{:?}", res);
        //res.iter().for_each(|p| println!("\n{:?}", p));
        //println!("{}", res[0].distancia3(&res[1], &res[2]));
        println!("{}", end.duration_since(start).as_millis());

        media += end.duration_since(start).as_millis();
    }
    println!("Media: {}", media / MEDIA);
}
