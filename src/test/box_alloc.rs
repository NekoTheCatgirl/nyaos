use alloc::{boxed::Box, vec::Vec};

use super::Test;

pub struct BoxAlloc;

impl Test for BoxAlloc {
    fn test(self) {
        println!("[TEST] [BoxAlloc] Executing Box allocation test 1/3");
        let x = Box::new(41);
        println!("[TEST] [BoxAlloc] Created a Box with contents '41' as i32");
        drop(x);
        println!("[TEST] [BoxAlloc] Deallocating the newly created Box!");
        println!("[TEST] [BoxAlloc] Executing Box allocation test 2/3");
        let mut boxes = Vec::new();
        for x in 0..500 {
            boxes.push(Box::new(x));
        }
        println!("[TEST] [BoxAlloc] Allocated 500 boxes in a Vec");
        drop(boxes);
        println!("[TEST] [BoxAlloc] Deallocated the Vec containing the boxes!");
        println!("[TEST] [BoxAlloc] Executing Box allocation test 3/3");
        let long_lived = Box::new(1);
        println!("[TEST] [BoxAlloc] Allocated a new Box containing a 1");
        for i in 0..500 {
            let x = Box::new(i);
            assert_eq!(*x, i);
        }
        assert_eq!(*long_lived, 1);
        println!("[TEST] [BoxAlloc] All Box allocation tests passed!");
    }
}