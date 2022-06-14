use super::*;

extern crate test;
use crate::boxed::Box;
use test::Bencher;

#[test]
fn allocate_zeroed() {
    unsafe {
        let layout = Layout::from_size_align(1024, 1).unwrap();
        let ptr =
            Global.allocate_zeroed(layout.clone()).unwrap_or_else(|_| handle_alloc_error(layout));

        let mut i = ptr.as_non_null_ptr().as_ptr();
        let end = i.add(layout.size());
        while i < end {
            assert_eq!(*i, 0);
            i = i.offset(1);
        }
        Global.deallocate(ptr.as_non_null_ptr(), layout);
    }
}

#[test]
fn reallocate_zeroed() {
    unsafe {
        // first allocate two pages and fill them with ones
        let old_layout = Layout::from_size_align(8192, 1).unwrap();
        let ptr = Global
            .allocate(old_layout.clone())
            .unwrap_or_else(|_| handle_alloc_error(old_layout))
            .as_non_null_ptr();
        ptr.as_ptr().write_bytes(1, old_layout.size());

        // shrink to one page and grow again
        let new_layout = Layout::from_size_align(4096, 1).unwrap();
        let ptr = Global
            .shrink(ptr, old_layout, new_layout)
            .unwrap_or_else(|_| handle_alloc_error(old_layout))
            .as_non_null_ptr();

        let old_layout = new_layout;
        let new_layout = Layout::from_size_align(5000, 1).unwrap();
        let ptr = Global
            .grow_zeroed(ptr, old_layout, new_layout)
            .unwrap_or_else(|_| handle_alloc_error(new_layout))
            .as_non_null_ptr();

        let mut i = ptr.as_ptr();
        let old_end = i.add(old_layout.size());
        let new_end = i.add(new_layout.size());
        while i < old_end {
            assert_eq!(*i, 1);
            i = i.offset(1);
        }
        while i < new_end {
            assert_eq!(*i, 0);
            i = i.offset(1);
        }
    }
}

#[bench]
#[cfg_attr(miri, ignore)] // isolated Miri does not support benchmarks
fn alloc_owned_small(b: &mut Bencher) {
    b.iter(|| {
        let _: Box<_> = Box::new(10);
    })
}
