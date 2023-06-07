
// //TODO unsend unsync
// //TODO higher orders
// //TODO skip methods

// #[derive(Debug)]
// pub struct Emtux1Factory<T0>(Mutex<T0>);

// impl<T0> Emtux1Factory<T0> {
//     pub fn new(val: T0) -> Self {
//         Self(Mutex::new(val))
//     }

//     pub fn build<'a>(&'a self) -> Emtux1<'a, T0> {
//         Emtux1(&self.0)
//     }
// }

// pub struct Emtux1<'a, T0>(&'a Mutex<T0>);

// impl<'a, T0> Emtux1<'a, T0> {
//     pub fn lock0(&mut self) -> Result<MutexGuard<T0>, PoisonError<MutexGuard<T0>>> {
//         self.0.lock()
//     }
// }

// #[derive(Debug)]
// pub struct Emtux2Factory<T0, T1>(Mutex<T0>, Mutex<T1>);

// impl<T0, T1> Emtux2Factory<T0, T1> {
//     pub fn new(val0: T0, val1: T1) -> Self {
//         Self(Mutex::new(val0), Mutex::new(val1))
//     }

//     pub fn build<'a>(&'a self) -> Emtux2<'a, T0, T1> {
//         Emtux2(&self.0, &self.1)
//     }
// }

// pub struct Emtux2<'a, T0, T1>(&'a Mutex<T0>, &'a Mutex<T1>);

// impl<'a, T0, T1> Emtux2<'a, T0, T1> {
//     pub fn lock0(&mut self) -> Result<MutexGuard<T0>, PoisonError<MutexGuard<T0>>> {
//         self.0.lock()
//     }

//     pub fn lock0_and(
//         &mut self,
//     ) -> (
//         Result<MutexGuard<T0>, PoisonError<MutexGuard<T0>>>,
//         Emtux1<T1>,
//     ) {
//         (self.0.lock(), Emtux1(self.1))
//     }

//     pub fn lock1(&mut self) -> Result<MutexGuard<T1>, PoisonError<MutexGuard<T1>>> {
//         self.1.lock()
//     }
// }
