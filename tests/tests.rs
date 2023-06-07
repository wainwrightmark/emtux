// use core::panic;
// use std::ops::DerefMut;

// use exempt::*;

// #[test]
// fn get_zero_drop_then_get_one() {
//     let factory = Emtux2Factory::new(0usize, 1usize);

//     let mut qt = factory.build();

//     let a = qt.lock0().unwrap();

//     assert!(a.eq(&0));
//     drop(a);
//     let b = qt.lock1().unwrap();

//     assert!(b.eq(&1));
// }

// #[test]
// fn get_zero_and_get_one() {
//     let factory = Emtux2Factory::new(0usize, 1usize);
//     let mut qt = factory.build();

//     let (a, mut qt1) = qt.lock0_and();
//     let mut a = a.unwrap();

//     let b = qt1.lock0().unwrap();

//     assert!(a.eq(&0));
//     assert!(b.eq(&1));
// }

// #[test]
// fn test() {

//     let factory = Emtux2Factory::new(0usize, 1usize);
//     let mut ea = factory.build();
//     let mut eb = factory.build();

//     let swaps1 = async {
//         for _ in 1..=8 {
//             swap_values(&mut ea)
//         }
//         ()
//     };

//     let swaps2 = async {
//         for _ in 1..=8 {
//             swap_values(&mut eb)
//         }
//         ()
//     };
//     panic!("uncomment code");
//     //async_std::task::block_on(async { futures_util::join!(swaps1, swaps2) });
// }

// fn swap_values<'a>( e: &mut Emtux2<usize, usize>){
//     let (a0, mut e2) = e.lock0_and();

//     let mut a0 = a0.unwrap();

//     let a1 = e2.lock0();

//     let mut a1 = a1.unwrap();

//     let swap = *a0;

//     *a0 = *a1;
//     *a1 = swap;

// }
