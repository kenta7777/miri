// A callee may not write to the destination of our `&mut` without us noticing.

fn main() {
    let mut x = 15;
    let xraw = &mut x as *mut _;
    // Derived from raw value, so using raw value is still ok ...
    let xref = unsafe { &mut *xraw };
    callee(xraw);
    // ... though any use of raw value will invalidate our ref.
    let _val = *xref;
    //~^ ERROR: does not exist on the borrow stack
}

fn callee(xraw: *mut i32) {
    unsafe { *xraw = 15 };
}
