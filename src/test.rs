#[cfg(test)]
mod tests {

    mod multi_thread {

        use crate::dyv_mt::DyVMT;
        use crate::read_points_from_file;
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
                let _ = t.join();
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
        use crate::dyv_st::DyV;
        use crate::read_points_from_file;
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
                0.08015304030013182,
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
                    assert!(res.eq(&answers[i]));
                });

                threads.push(t);
            }

            for t in threads {
                let _ = t.join();
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
                let _ = t.join();
            });
        }

        #[test]
        fn d657() {
            let puntos = read_points_from_file("point_files/d657.tsp");
            let mut dyv = DyV::new(&puntos);
            let res = dyv.start();
            assert_eq!(res, 35.9210244842761);
        }

        #[test]
        fn small_tests() {
            let answers = [
                73498.11815595091,
                128619.57413192868,
                166528.24868796268,
                138966.38577736917,
                110984.69938339421,
                81177.56393881867,
                82380.39676356448,
                232030.34848398055,
                113190.95385331071,
                135306.3101400795,
            ];

            for i in 0..10 {
                let puntos = read_points_from_file(&format!("point_files/puntos_rand_small_{}.tsp", i));
                let mut dyv = DyV::new(&puntos);
                let res = dyv.start();
                assert_eq!(res, answers[i]);
            }
        }
    }
}
