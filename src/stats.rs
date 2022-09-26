use std::cmp::Ordering::Equal;

pub trait Measurable {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn zero_value() -> Self;
}

macro_rules! measurable_impl {
    ($t:ty, $min:expr, $max:expr, $zero:expr) => {
        impl Measurable for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }

            #[inline]
            fn max_value() -> $t {
                $max
            }

            #[inline]
            fn zero_value() -> $t {
                $zero
            }
        }
    };
}

measurable_impl!(f32, f32::MIN, f32::MAX, 0f32);

pub fn minimum<T: Measurable + std::cmp::PartialOrd>(iterable: Vec<T>) -> T {
    let mut md = T::min_value();
    for d in iterable {
        if d < md {
            md = d;
        }
    }

    return md;
}

pub fn maximum<T: Measurable + std::cmp::PartialOrd>(iterable: Vec<T>) -> T {
    let mut md = T::zero_value();
    for d in iterable {
        if d > md {
            md = d;
        }
    }

    return md;
}

pub fn average<T: Measurable + std::ops::Div<f32, Output = T>>(iterable: Vec<T>) -> T
where
    T: Clone + std::ops::Add<Output = T>,
{
    let mut sum = T::zero_value();
    for d in iterable.clone() {
        sum = sum + d;
    }

    let len = iterable.len() as f32;

    return sum / len;
}

pub fn median<T: Measurable>(iterable: Vec<T>) -> T
where
    T: Copy
        + Clone
        + std::cmp::PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Div<f32, Output = T>,
{
    let l = iterable.len();
    let mut dd = iterable.clone();

    dd.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));

    if l % 2 == 0 {
        return dd[l / 2] + dd[l / 2 - 1] / 2.0f32;
    } else {
        return dd[l / 2];
    }
}

pub fn jitter(iterable: Vec<f32>) -> f32 {
    let mut jj: Vec<f32> = Vec::new();

    for i in 1..iterable.len() {
        if iterable[i - 1] > iterable[i] {
            jj.push(iterable[i - 1] - iterable[i])
        } else {
            jj.push(iterable[i] - iterable[i - 1])
        }
    }

    return average(jj);
}
