use std::sync;
use std::sync::Mutex;
use std::thread;

use emtux::vec::EmtuxVecError;
use emtux::{
    vec::{EmtuxVec, EmtuxVecView},
    *,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::mpsc::Sender;

#[test]
fn test_get() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(vec.get(5).unwrap().clone(), 5);

    let mut view = vec.get_view();

    assert_eq!(view.get(5).unwrap().clone(), 5);
}

#[test]
fn test_vec_functions() {
    let mut vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert!(!vec.is_empty());
    assert_eq!(vec.len(), 10);
    vec.push(10);
    assert_eq!(vec.len(), 11);
    vec.clear();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_into_iter() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let v2: Vec<_> = vec.into_iter().map(|x| x.unwrap()).collect();

    assert_eq!(v2, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[test]
#[allow(unreachable_code)]
fn test_poison() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0]);

    let vec = Box::leak(Box::new(vec));

    let mut view_a = vec.get_view();
    let swaps1 = thread::spawn(move || {
        let _a = view_a.get(0).unwrap();
        panic!("Test Error");

        *_a = 1;
    });

    swaps1.join().expect_err("Thread should have panicked");

    match vec.get(0) {
        Err(EmtuxVecError::Poison(p)) => {
            assert_eq!(p.to_string(), "poisoned lock: another task failed inside")
        }
        _ => panic!("Expected poison error"),
    }
}

#[test]
fn test_out_of_bounds() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    match vec.get(11) {
        Err(EmtuxVecError::IndexOutsideBounds) => {}
        _ => panic!("Expected out of bounds error"),
    };

    let mut view = vec.get_view();

    match view.get(11) {
        Err(EmtuxVecError::IndexOutsideBounds) => {}
        _ => panic!("Expected out of bounds error"),
    };
    {}
}

#[test]
fn test_duplicate_index() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let [first, second] = vec.get_many([1, 1]);

    assert_eq!(first.unwrap().clone(), 1);

    match second {
        Err(EmtuxVecError::DuplicateIndex) => {}
        _ => panic!("Expected out of bounds error"),
    };
}

#[test]
fn test_concurrency() {
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let vec = Box::leak(Box::new(vec));

    let mut view_a = vec.get_view();
    let mut view_b = vec.get_view();

    let swaps1 = thread::spawn(move || {
        let mut rng: StdRng = SeedableRng::seed_from_u64(123);

        for _ in 1..=4 {
            swap_values(&mut view_a, &mut rng, "Thread 1");
        }
    });

    let swaps2 = thread::spawn(move || {
        let mut rng: StdRng = SeedableRng::seed_from_u64(456);
        for _ in 1..=4 {
            swap_values(&mut view_b, &mut rng, "Thread 2")
        }
    });

    swaps1.join().unwrap();
    swaps2.join().unwrap();

    let mut results: Vec<_> = vec.iter().map(|x| *x.unwrap()).collect();

    let results_string = results
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", results_string);

    results.sort();

    assert_eq!(results, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

fn swap_values<'a, R: Rng>(e: &mut EmtuxVecView<usize>, rand: &mut R, thread_name: &'static str) {
    //sender.send(thread_name.to_string()).unwrap();
    let i0 = rand.gen_range(0..e.len());
    let i1 = rand.gen_range(0..e.len());

    let [v0, v1] = e.get_many([i0, i1]);

    let mut mg0 = v0.unwrap();

    let mut mg1 = match v1 {
        Ok(x) => x,
        Err(e) => match e {
            EmtuxVecError::IndexOutsideBounds => panic!("{thread_name}: Index out of bounds"),
            EmtuxVecError::DuplicateIndex => {
                println!("{thread_name}: No need to swap {i0} with {i1}");
                return;
            }
            EmtuxVecError::Poison(_) => panic!("{thread_name}: Lock Poison"),
        },
    };

    println!(
        "{thread_name}: Swapping {} at {} with {} at {}",
        *mg0, i0, *mg1, i1
    );

    std::mem::swap(&mut (*mg0), &mut (*mg1));
}

// #[test]
// fn cause_deadlock() {
//     let v = vec![Mutex::new(0), Mutex::new(1)];
//     let v = Box::new(v).leak();
//     let swaps_0_then_1 = thread::spawn(|| {
//         let mut mg0 = v[0].lock().unwrap();
//         thread::sleep(std::time::Duration::from_millis(100));
//         let mut mg1 = v[1].lock().unwrap();

//         std::mem::swap(&mut (*mg0), &mut (*mg1));
//     });

//     let swaps_1_then_0 = thread::spawn(|| {
//         let mut mg1 = v[1].lock().unwrap();
//         thread::sleep(std::time::Duration::from_millis(100));
//         let mut mg0 = v[0].lock().unwrap();

//         std::mem::swap(&mut (*mg0), &mut (*mg1));
//     });

//     swaps_0_then_1.join().unwrap();
//     swaps_1_then_0.join().unwrap();
// }

// #[test]
// fn do_not_cause_deadlock() {
//     let vec: EmtuxVec<usize> = EmtuxVec::from_iter([0, 1]);

//     let vec = Box::leak(Box::new(vec));

//     let mut view_a = vec.get_view();
//     let mut view_b = vec.get_view();

//     let swaps_0_then_1 = thread::spawn(move || {
//         let [r0, r1] = view_a.get_many([0,1]);
//         let mut mg0 = r0.unwrap();
//         let mut mg1 = r1.unwrap();

//         std::mem::swap(&mut (*mg0), &mut (*mg1));
//     });

//     let swaps_1_then_0 = thread::spawn(move || {
//         let [r1, r0] = view_b.get_many([1,0]);
//         let mut mg0 = r0.unwrap();
//         let mut mg1 = r1.unwrap();

//         std::mem::swap(&mut (*mg0), &mut (*mg1));
//         }
//     );

//     swaps_0_then_1.join().unwrap();
//     swaps_1_then_0.join().unwrap();
// }
