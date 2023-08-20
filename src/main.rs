use num::Zero;
use std::ops;

#[derive(Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Array<T> {
    shape: (i32, i32),
    value: Vec<Vec<T>>,
}

impl<T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy + Default> Array<T> {
    fn zeros(shape: (i32, i32)) -> Self
    where
        T: Zero,
    {
        Array {
            shape,
            // value: vec![vec![0; shape.1 as usize]; shape.0 as usize],
            value: vec![vec![T::zero(); shape.1 as usize]; shape.0 as usize],
        }
    }
    fn dot(&self, other: &Array<T>) -> T {
        unimplemented!()
    }
}

impl<T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy + Clone> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
    fn dot(&self, other: &Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: ops::Add<Output = T> + Copy + Clone> ops::Add<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy + Clone> ops::Sub<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy + Clone> ops::Mul<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T: ops::Div<Output = T> + Copy + Clone> ops::Div<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// Update in place
impl<T: ops::Add<Output = T> + Copy + Clone> ops::Add<T> for &mut Vec2<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self
    }
}
impl<T: ops::Sub<Output = T> + Copy + Clone> ops::Sub<T> for &mut Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self
    }
}

impl<T: ops::Mul<Output = T> + Copy + Clone> ops::Mul<T> for &mut Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self
    }
}

impl<T: ops::Div<Output = T> + Copy + Clone> ops::Div<T> for &mut Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self
    }
}

// Create new vector
impl<T: ops::Add<Output = T> + Copy + Clone> ops::Add<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy + Clone> ops::Sub<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy + Clone> ops::Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T: ops::Div<Output = T> + Copy + Clone> ops::Div<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

macro_rules! scalar_ops (
    ($($T: ty),+) => {
        $(
        // Update in place
        impl<'a> ops::Add<&'a mut Vec2<$T>> for $T {
            type Output = &'a mut Vec2<$T>;

            fn add(self, rhs: &'a mut Vec2<$T>) -> Self::Output {
                rhs + self
            }
        }

        impl<'a> ops::Sub<&'a mut Vec2<$T>> for $T {
            type Output = &'a mut Vec2<$T>;

            fn sub(self, rhs: &'a mut Vec2<$T>) -> Self::Output {
                rhs - self
            }
        }
        impl<'a> ops::Mul<&'a mut Vec2<$T>> for $T {

            type Output = &'a mut Vec2<$T>;

            fn mul(self, rhs: &'a mut Vec2<$T>) -> Self::Output {
                rhs * self
            }
        }
        impl<'a> ops::Div<&'a mut Vec2<$T>> for $T
        {
            type Output = &'a mut Vec2<$T>;

            fn div(self, rhs: &'a mut Vec2<$T>) -> Self::Output {
                rhs / self
            }
        }

        // Create new vector
        impl ops::Add<&Vec2<$T>> for $T {
            type Output = Vec2<$T>;

            fn add(self, rhs: &Vec2<$T>) -> Self::Output {
                    rhs + self

            }
        }
        impl ops::Sub<&Vec2<$T>> for $T {
            type Output = Vec2<$T>;

            fn sub(self, rhs: &Vec2<$T>) -> Self::Output {
                    rhs - self

            }
        }
        impl ops::Mul<&Vec2<$T>> for $T {
            type Output = Vec2<$T>;

            fn mul(self, rhs: &Vec2<$T>) -> Self::Output {
                    rhs * self

            }
        }
        impl ops::Div<&Vec2<$T>> for $T {
            type Output = Vec2<$T>;

            fn div(self, rhs: &Vec2<$T>) -> Self::Output {
                    rhs / self

            }
        }
        )+
    }
);

scalar_ops!(i32, i64, f32, f64);

fn main() {
    let mut a = Vec2::new(1., 10.);
    let b = Vec2::new(5., 2.);
    // let c = &mut a * 10.;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("----------------------------------");

    println!("a + b = {:?}", &a + &b);
    println!("a * b = {:?}", &a * &b);
    println!("a dot b = {:?}", a.dot(&b));
    println!("a * 10.0 = {:?} (in-place update)", &mut a * 10.);

    let c = 2. * &a;
    println!("c = 2.0 * a = {:?} (new vector)", c);
    println!("a = {:?}", a);

    let arr: Array<f64> = Array::zeros((5, 5));
    println!("{:?}", arr);
}
