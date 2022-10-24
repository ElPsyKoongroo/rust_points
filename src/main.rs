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

enum Distance {
    NotFound,
    Found(f64),
}

fn calcula_fixed(
    puntos: &[Punto],
    start: usize,
    end: usize,
    distancia_mejor: &mut f64,
    mejores_puntos: &mut [Punto],
) -> f64 {
    if end - start + 1 < 3 {
        return *distancia_mejor;
    }
    let end = end + 1;
    let mut distancia_minima: f64 = MAX;

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

                if distancia < distancia_minima {
                    distancia_minima = distancia;
                    if distancia_minima < *distancia_mejor {
                        *distancia_mejor = distancia_minima;
                        (*mejores_puntos)[0] = puntos[i].clone();
                        (*mejores_puntos)[1] = puntos[j].clone();
                        (*mejores_puntos)[2] = puntos[k].clone();
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    /*
    for i in start..end {
        for j in start..end {
            if j == i {
                continue;
            }

            for k in start..end {
                if i >= k || j == k {
                    continue;
                }

                println!("{} {} {}", i,j,k);
                let distancia: f64 = puntos[i].distancia3(&puntos[j], &puntos[k]);

                if distancia < distancia_minima {
                    distancia_minima = distancia;
                    if distancia_minima < *distancia_mejor {
                        *distancia_mejor = distancia_minima;
                        (*mejores_puntos)[0] = puntos[i].clone();
                        (*mejores_puntos)[1] = puntos[j].clone();
                        (*mejores_puntos)[2] = puntos[k].clone();
                    }
                }
            }
        }
    }*/

    distancia_minima
}

fn get_points_between(puntos: &[Punto], start: f64, end: f64) -> (usize, usize) {
    let start_index = puntos
        .iter()
        .position(|&p| p.x > start)
        .unwrap_or_else(|| N_POINTS); // 722
    let end_index = puntos.iter().rposition(|&p| p.x < end).unwrap();
    (start_index, end_index)
}

fn divide_benceras(puntos: &[Punto]) -> Vec<Punto> {
    let mut distancia_mejor = MAX;
    let mut mejores_puntos: [Punto; 3] = [Punto::default(); 3];
    divide_venceras(
        puntos,
        puntos[0].x,
        puntos[puntos.len() - 1].x,
        &mut distancia_mejor,
        &mut mejores_puntos,
    );
    return mejores_puntos.to_vec();
}

fn repes(puntos: &[Punto], start: usize, end: usize) -> bool {
    return !(start..end - 1).any(|i| puntos[i].x != puntos[i + 1].x);
}

fn divide_venceras(
    puntos: &[Punto],
    start: f64,
    end: f64,
    distancia_mejor: &mut f64,
    mejores_puntos: &mut [Punto],
) -> Distance {
    let mitad: f64 = (start + end) / 2.0;
    let (start_index, end_index) = get_points_between(puntos, start, end);

    if end_index - start_index + 1 < 3 {
        return Distance::NotFound;
    }

    if repes(puntos, start_index, end_index) {
        return Distance::NotFound;
    }

    let izq = divide_venceras(puntos, start, mitad, distancia_mejor, mejores_puntos);
    let drc = divide_venceras(puntos, mitad, end, distancia_mejor, mejores_puntos);

    let distancia_min = match (izq, drc) {
        (Distance::NotFound, Distance::NotFound) => {
            return Distance::Found(calcula_fixed(
                puntos,
                start_index,
                end_index,
                distancia_mejor,
                mejores_puntos,
            ));
        }
        (Distance::Found(e), Distance::NotFound) => e,
        (Distance::NotFound, Distance::Found(e)) => e,
        (Distance::Found(i), Distance::Found(e)) if i > e => e,
        (Distance::Found(i), Distance::Found(e)) if i < e => i,
        _ => MAX,
    };

    let (new_start, new_end) =
        get_points_between(puntos, mitad - *distancia_mejor, mitad + *distancia_mejor);

    let aux;
    if distancia_min < end - start {
        aux = calcula_fixed(puntos, new_start, new_end, distancia_mejor, mejores_puntos);
        if aux < distancia_min {
            return Distance::Found(aux);
        } else {
            return Distance::Found(distancia_min);
        }
    }

    aux = calcula_fixed(
        puntos,
        start_index,
        end_index,
        distancia_mejor,
        mejores_puntos,
    );
    if aux < distancia_min {
        return Distance::Found(aux);
    } else {
        return Distance::Found(distancia_min);
    }
}

#[allow(unused)]
fn genera_random(num_puntos: usize, upper_bound: f64, lower_bound: f64) -> Vec<Punto> {
    let mut puntos = Vec::with_capacity(num_puntos);
    let mut file = std::io::BufWriter::new(std::fs::File::create("puntos.tsp").unwrap());
    file.write_all("NODE_COORD_SECTION\n".as_bytes()).unwrap();
    for _ in 0..num_puntos {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(lower_bound..upper_bound);
        let y: f64 = rng.gen_range(lower_bound..upper_bound);
        file.write_all(format!("1 {} {}\n", x, y).as_bytes())
            .unwrap();
        puntos.push(Punto { x, y })
    }
    puntos
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

static N_POINTS: usize = 20_000;
static MEDIA: u128 = 1;

fn main() {
    // let mut puntos = genera_random(N_POINTS, 1000.0, 0.0);
    let mut puntos = read_points_from_file("puntos.tsp");
    puntos.sort();
    println!("GO!");
    let puntos = puntos;
    let mut media = 0;
    for _ in 0..MEDIA {
        let start = Instant::now();
        let res = divide_benceras(&puntos);
        let end = Instant::now();

        res.iter().for_each(|p| println!("\n{:?}", p));
        println!("{}", res[0].distancia3(&res[1], &res[2]));
        println!("{}", end.duration_since(start).as_millis());

        media += end.duration_since(start).as_millis();
    }
    println!("Media: {}", media / MEDIA);
}
