pub trait GcdLcm {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}
macro_rules! impl_gcdlcm {
    ($ty:ty, $to:ty) => {
        impl GcdLcm for $ty {
            fn gcd(self, other: Self) -> Self {
                ::gcd::Gcd::gcd(self as $to, other as $to) as $ty
            }

            fn lcm(self, other: Self) -> Self {
                self * other / self.gcd(other)
            }
        }
    };
}
impl_gcdlcm!(u8, u8);
impl_gcdlcm!(u16, u16);
impl_gcdlcm!(u32, u32);
impl_gcdlcm!(u64, u64);
impl_gcdlcm!(u128, u128);
impl_gcdlcm!(usize, u64);
impl_gcdlcm!(i8, u8);
impl_gcdlcm!(i16, u16);
impl_gcdlcm!(i32, u32);
impl_gcdlcm!(i64, u64);
impl_gcdlcm!(isize, u64);
