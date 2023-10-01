use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize, Ordering},
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

// Sending an Arc across threads results in a T object being shared. This requires T to be Sync.
// Sending an Arc across threads could result in another thread dropping T, transferring it to that thread.
// This requires T to be Send.
unsafe impl<T> Send for Arc<T> where T: Send + Sync {}
unsafe impl<T> Sync for Arc<T> where T: Send + Sync {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        // Safety: We will ensure that the ptr points to valid data as long as
        // the Arc object exist.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // We can use Relaxed ordering, because we do not need to ensure that
        // some operation on other variables that need to strictly happen-before
        // of happen-after this atomic operation.
        if self.data().ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            // handle potential overflow
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // We cannot use Relaxed ordering, because we need to ensure that
        // no other thread can access the data after we drop it.
        // The final fetch_sub must establish a happens-before relationship with
        // every previous fetch-sub operation.
        // Solution 1: Use AcqRel for every fetch_sub operation.
        /*
                if self.data().ref_count.fetch_sub(1, Ordering::AcqRel) == 1 {
                    // This is the last thread having access to ArcData
                    unsafe {
                        drop(Box::from_raw(self.ptr.as_ptr()));
                    }
        }*/

        // Solution 2: Use Release for the fetch_sub operation,
        // and a fence for the last drop.
        // This is because only the last decrement to zero needs Acquire ordering,
        // and others only need the Release ordering.
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Create two Arcs sharing an object containing a string
    // and a DetectDrop
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();


    // Send x to another thread, and use it there
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });

    // In parallel, y should still be usable
    assert_eq!(y.0, "hello");

    t.join().unwrap();
    // x has been dropped by now. However, y hasn't been dropped.
    // Thus x::drop() has been called, but that doesn't call
    // the internal drop.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);


    // Drop the remaining Arc
    drop(y);

    // The internal drop() should have been called after both x and y are dropped.
    // Check the number of times the arc has been dropped.
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
}
