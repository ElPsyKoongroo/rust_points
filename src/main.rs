use rand::Rng;
mod coord;
mod dyv_mt;
mod dyv_st;
mod punto;

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
            let points = dyv.get_points();
            let end = Instant::now();

            media += end.duration_since(start).as_millis();
            println!("{res}, {:?}", points);
        }
        println!("Media: {} ms with {}", media / MEDIA, points);
    }
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

    mod multi_thread {

        use super::super::DyVMT;
        use super::super::read_points_from_file;
        #[test]
        fn test_1() {
            let puntos = read_points_from_file("point_files/puntos_800000.tsp");
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 0.03781606866923333);
        }

        #[test]
        fn test_2() {
            let puntos = read_points_from_file("point_files/puntos_500000.tsp");
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 0.05537844995897111);
        }

        // [30_000, 50_000, 80_000, 150_000,
        #[test]
        fn test_3() {
            let puntos = read_points_from_file("point_files/puntos_150000.tsp");
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 0.144052106804805);
        }
        #[test]
        fn test_4() {
            let puntos = read_points_from_file("point_files/puntos_80000.tsp");
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 0.322497589363708);
        }

        #[test]
        fn test_5() {
            let puntos = read_points_from_file("point_files/puntos_50000.tsp");
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 0.30805303890637226);
        }
        #[test]
        fn test_6() {
            let puntos = read_points_from_file("point_files/puntos_30000.tsp");
            let mut dyv = DyVMT::new(&puntos);
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
                    let puntos =
                        read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
                    let mut dyv = DyVMT::new(&puntos);
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
                    let puntos =
                        read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
                    let mut dyv = DyVMT::new(&puntos);
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
            let mut dyv = DyVMT::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 35.9210244842761);
        }
    }

    mod single_thread {
        use super::super::DyV;
        use super::super::read_points_from_file;
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
                    let puntos =
                        read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
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
                    let puntos =
                        read_points_from_file(&format!("point_files/puntos_rand_{}.tsp", i));
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
}
