use std::sync;
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

    assert_eq!(vec.is_empty(), false);
    assert_eq!(vec.len(), 10);
    vec.push(10);
    assert_eq!(vec.len(), 11);
    vec.clear();
    assert_eq!(vec.is_empty(), true);
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

        ()
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
    let (sender, receiver) = sync::mpsc::channel::<String>();
    let vec: EmtuxVec<usize> = vec::EmtuxVec::from_iter([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let vec = Box::leak(Box::new(vec));

    let mut view_a = vec.get_view();
    let mut view_b = vec.get_view();

    let sender1 = sender.clone();
    let sender2 = sender.clone();

    let swaps1 = thread::spawn(move || {
        let mut rng: StdRng = SeedableRng::seed_from_u64(123);

        for _ in 1..=4 {
            swap_values(&mut view_a, &mut rng, "Thread 1", &sender1);
        }
        drop(sender1);
        ()
    });

    let swaps2 = thread::spawn(move || {
        let mut rng: StdRng = SeedableRng::seed_from_u64(456);
        for _ in 1..=4 {
            swap_values(&mut view_b, &mut rng, "Thread 2", &sender2)
        }
        drop(sender2);
        ()
    });

    drop(sender);

    swaps1.join().unwrap();
    swaps2.join().unwrap();

    for message in receiver.into_iter() {
        println!("{}", message)
    }

    let mut results: Vec<_> = vec.iter().map(|x| x.unwrap().clone()).collect();

    let results_string = results
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", results_string);

    results.sort();

    assert_eq!(results, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

fn swap_values<'a, R: Rng>(
    e: &mut EmtuxVecView<usize>,
    rand: &mut R,
    thread_name: &'static str,
    sender: &Sender<String>,
) {
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
                sender
                    .send(format!("{thread_name}: No need to swap {i0} with {i1}"))
                    .expect("Could not send");
                return;
            }
            EmtuxVecError::Poison(_) => panic!("{thread_name}: Lock Poison"),
        },
    };

    sender
        .send(format!(
            "{thread_name}: Swapping {} at {} with {} at {}",
            *mg0, i0, *mg1, i1
        ))
        .expect("Could not send");

    let swap = *mg0;
    *mg0 = *mg1;
    *mg1 = swap;
}
