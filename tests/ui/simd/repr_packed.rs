// run-pass

#![feature(repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]

#[repr(simd, packed)]
struct Simd<T, const N: usize>([T; N]);

fn check_size_align<T, const N: usize>() {
    use std::mem;
    assert_eq!(mem::size_of::<Simd<T, N>>(), mem::size_of::<[T; N]>());
    assert_eq!(mem::size_of::<Simd<T, N>>() % mem::align_of::<Simd<T, N>>(), 0);
}

fn check_ty<T>() {
    check_size_align::<T, 1>();
    check_size_align::<T, 2>();
    check_size_align::<T, 3>();
    check_size_align::<T, 4>();
    check_size_align::<T, 8>();
    check_size_align::<T, 9>();
    check_size_align::<T, 15>();
}

extern "platform-intrinsic" {
    fn simd_add<T>(a: T, b: T) -> T;
}

fn main() {
    check_ty::<u8>();
    check_ty::<i16>();
    check_ty::<u32>();
    check_ty::<i64>();
    check_ty::<usize>();
    check_ty::<f32>();
    check_ty::<f64>();

    unsafe {
        // powers-of-two have no padding and work as usual
        // non-powers-of-two have padding and need to be expanded to full vectors
        let x: Simd<f64, 4> =
            simd_add(Simd::<f64, 4>([0., 1., 2., 3.]), Simd::<f64, 4>([2., 2., 2., 2.]));
        assert_eq!(std::mem::transmute::<_, [f64; 4]>(x), [2., 3., 4., 5.]);
    }
}
