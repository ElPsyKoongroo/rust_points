#![warn(clippy::all)]
#![feature(portable_simd)]
use rand::Rng;
mod coord;
mod dyv_mt;
mod dyv_st;
mod dyv_simd;
mod punto;
mod test;
mod dyv_it;

#[allow(unused_imports)]
use dyv_mt::DyVMT;
use dyv_st::DyV;
use dyv_simd::DyVSIMD;
use punto::*;

static N_POINTS: usize = 2_000_000;
static MEDIA: u128 = 30;
const POINT_FILES: &str = "point_files/";

use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use crate::dyv_it::DyVIT;
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
fn genera_random_with_dost<I: rand::distributions::Distribution<f64>>(
    dist: I,
    num_puntos: usize,
    upper_bound: f64,
    lower_bound: f64,
) -> Vec<Punto> {
    let mut puntos = Vec::with_capacity(num_puntos);
    for _ in 0..num_puntos {
        let mut rng = rand::thread_rng();
        let x: f64 = dist.sample(&mut rng);
        let y: f64 = dist.sample(&mut rng);
        puntos.push(Punto { x, y })
    }
    puntos
}

#[allow(unused)]
fn write_points(puntos: &[Punto]) {
    let mut file = BufWriter::new(File::create("puntos.tsp").unwrap());
    file.write_all("NODE_COORD_SECTION\n".as_bytes()).unwrap();

    for punto in puntos {
        file.write_all(format!("1 {} {}\n", punto.x, punto.y).as_bytes())
            .unwrap();
    }
}

fn read_points_from_file<I: AsRef<Path>>(file_name: I) -> Vec<Punto> {
    let mut points = Vec::with_capacity(N_POINTS);
    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(File::open(file_name).unwrap());

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


#[allow(unused)]
fn write_points_with_name<I: AsRef<Path>>(name: I, puntos: &[Punto]) {
    let mut file = BufWriter::new(File::create(name).unwrap());
    file.write_all("NODE_COORD_SECTION\n".as_bytes()).unwrap();

    for punto in puntos {
        file.write_all(format!("1 {} {}\n", punto.x, punto.y).as_bytes())
            .unwrap();
    }
}

fn bench() {
    let file_path = PathBuf::from(POINT_FILES).join("puntos_big_2m.tsp");
    let mut puntos = read_points_from_file(&file_path);
    puntos.sort();
    println!("Testing {} GO!", file_path.display());
    let mut media;
    for points in 1..=2 {
        media = 0;
        for _ in 0..MEDIA {
            let mut dyv = DyVSIMD::new(&puntos);
            let start = Instant::now();
            let res = dyv.start();
            let end = Instant::now();

            let actual = end.duration_since(start).as_millis();
            println!("\t{} ms {:?} {}", actual, dyv.get_points(), res);

            media += actual;
            //println!("{res}, {:?}", points);
        }
        println!("Media: {} ms with {}", media / MEDIA, points);
    }
}

#[allow(dead_code)]
fn prueba() {
    for file_n in 1..=9 {
        let file = format!("puntos_rand_{file_n}.tsp");
        let mut puntos = read_points_from_file(PathBuf::from(POINT_FILES).join(&file));
        puntos.sort();
        println!("GO!");
        println!("FILE: {}", &file);
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        print!("\t solution: {res}");
    }
}

#[allow(dead_code)]
fn genera_puntos_file() {
    let mut rng = rand::thread_rng();
    for i in 0..25 {
        let n_points = rng.gen::<usize>() % 50_000 + 100_000;
        let dist = rand::distributions::Uniform::new(10_000_000.1, 10_000_000.1);
        let mut puntos = genera_random_with_dost(dist, n_points, 10_000_000.1, -10_000_000.0);
        puntos.sort();
        write_points_with_name(format!("point_files/puntos_rand_small_{}.tsp", i), &puntos);
    }

    let dist = rand::distributions::Uniform::new(10_000_000.1, 10_000_000.1);
    let mut puntos = genera_random_with_dost(dist, 50_000, 10_000_000.1, -10_000_000.0);
    puntos.sort();

    for i in 25..50 {
        let n_points = rng.gen::<usize>() % 1_000 + 500;
        let dist = rand::distributions::Uniform::new(10_000_000.1, 10_000_000.1);
        let mut puntos = genera_random_with_dost(dist, n_points, 10_000_000.1, -10_000_000.0);
        puntos.sort();
        write_points_with_name(format!("point_files/puntos_rand_small_{}.tsp", i), &puntos);
    }
}

fn main() {
    /*
    for i in 0..10 {
        let puntos = read_points_from_file(&format!("point_files/puntos_rand_small_{}.tsp", i));
        let mut dyv = DyV::new(&puntos);
        let res = dyv.start();
        println!("{:?}, {}", dyv.get_points(), res);
    }
    */

    bench()
    //prueba()
}
