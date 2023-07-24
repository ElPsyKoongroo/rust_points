use rand::Rng;
mod coord;
mod dyv_mt;
mod dyv_st;
mod punto;
mod test;

use dyv_mt::DyVMT;
use dyv_st::DyV;
use punto::*;

const POINT_FILES: &str = "point_files/";

use std::{
    io::{BufRead, Write},
    path::{Path, PathBuf},
    time::Instant,
};
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
static MEDIA: u128 = 10;

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
    let mut media;
    for points in 1..5 {
        media = 0;
        for _ in 0..MEDIA {
            let mut dyv = DyVMT::new(&puntos);
            let start = Instant::now();
            let res = dyv.start();
            //let points = dyv.get_points();
            let end = Instant::now();

            media += end.duration_since(start).as_millis();
            println!("{res}, {:?}", points);
        }
        println!("Media: {} ms with {}", media / MEDIA, points);
    }
}

fn genera_puntos_file() {
    for i in 0..10 {
        let mut puntos = genera_random(1_000, 10_000_000.1, -10_000_000.0);
        puntos.sort();
        write_points_with_name(format!("point_files/puntos_rand_small_{}.tsp", i), &puntos);
    }
}

fn main() {
    for i in 0..10 {
        let puntos = read_points_from_file(&format!("point_files/puntos_rand_small_{}.tsp", i));
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        println!("{:?}", res);
    }

    bench()
}
