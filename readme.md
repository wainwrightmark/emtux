## Emtux

Concurrent collections and primitives that offer protection against deadlocks.

`emtux` introduces several types (with more on the way) that allow for concurrent mutable access to multiple resources without the risk of deadlocking. This is done by using the rust ownership system to ensure that locks are always taken in the same order, thus preventing the possibility of deadlock.

Consider the following example of swapping values in a `vec` from two different threads simultaneously.

```rust
let v = vec![Mutex::new(0), Mutex::new(1)];
let v = Box::new(v).leak();


let swaps_0_then_1 = thread::spawn(|| {
    let mut mg0 = v[0].lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    let mut mg1 = v[1].lock().unwrap();

    std::mem::swap(&mut (*mg0), &mut (*mg1));
});

let swaps_1_then_0 = thread::spawn(|| {
    let mut mg1 = v[1].lock().unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    let mut mg0 = v[0].lock().unwrap();

    std::mem::swap(&mut (*mg0), &mut (*mg1));
});

swaps_0_then_1.join().unwrap();
swaps_1_then_0.join().unwrap();
```

This code will usually deadlock because both mutexes are locked at the same time and neither thread can continue.
If you remove the `thread::sleep` invocations, it could still potentially deadlock but it would be much rarer, and harder to detect.

`emtux` lets you randomly, mutably, access arbitrary sets of vector elements fom concurrent threads without the risk of deadlocks.

```rust
let vec: EmtuxVec<usize> = EmtuxVec::from_iter([0, 1]);

    let vec = Box::leak(Box::new(vec));

    let mut view_a = vec.get_view();
    let mut view_b = vec.get_view();

    let swaps_0_then_1 = thread::spawn(move || {
        let [r0, r1] = view_a.get_many([0,1]);
        let mut mg0 = r0.unwrap();
        let mut mg1 = r1.unwrap();

        std::mem::swap(&mut (*mg0), &mut (*mg1));
    });

    let swaps_1_then_0 = thread::spawn(move || {
        let [r1, r0] = view_b.get_many([1,0]);
        let mut mg0 = r0.unwrap();
        let mut mg1 = r1.unwrap();

        std::mem::swap(&mut (*mg0), &mut (*mg1));
        }
    );

    swaps_0_then_1.join().unwrap();
    swaps_1_then_0.join().unwrap();

```

The `get_many` function makes sure that the elements are taken in a consistent order which prevents the possibility of deadlocks. `get_many` also required unique (`mut`) access which prevents you from using it wrong and calling it twice.
It is possible to cause a deadlock by passing multiple views into the same thread but the API is designed to make it clear that you are not supposed to do that.
