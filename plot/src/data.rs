use std::mem;

use crate::traits::Data;

// Strictly lossless conversions
macro_rules! impl_data_lossless {
    ($($ty:ty),+) => {
        $(
            impl Data for $ty {
                #[inline]
                fn f64(self) -> f64 {
                    f64::from(self)
                }
            }

            impl<'a> Data for &'a $ty {
                #[inline]
                fn f64(self) -> f64 {
                    f64::from(*self)
                }
            }
        )+
    };
}

// Conversions with possible precision loss (>53 bits) using `as`
macro_rules! impl_data_lossy {
    ($($ty:ty),+) => {
        $(
            impl Data for $ty {
                #[inline]
                fn f64(self) -> f64 {
                    self as f64
                }
            }

            impl<'a> Data for &'a $ty {
                #[inline]
                fn f64(self) -> f64 {
                    (*self) as f64
                }
            }
        )+
    };
}

impl_data_lossless!(f32, i8, i16, i32, u8, u16, u32);
impl_data_lossy!(i64, u64, isize, usize);

impl Data for f64 {
    #[inline]
    fn f64(self) -> f64 {
        self
    }
}

impl Data for &f64 {
    #[inline]
    fn f64(self) -> f64 {
        *self
    }
}

#[derive(Clone)]
pub struct Matrix {
    bytes: Vec<u8>,
    ncols: usize,
    nrows: usize,
}

impl Matrix {
    pub fn new<I>(rows: I, scale: <I::Item as Row>::Scale) -> Matrix
    where
        I: Iterator,
        I::Item: Row,
    {
        let ncols = I::Item::ncols();
        let bytes_per_row = ncols * mem::size_of::<f64>();
        let mut bytes = Vec::with_capacity(rows.size_hint().0 * bytes_per_row);

        let mut nrows = 0;
        for row in rows {
            nrows += 1;
            row.append_to(&mut bytes, scale);
        }

        Matrix {
            bytes,
            ncols,
            nrows,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn nrows(&self) -> usize {
        self.nrows
    }
}

/// Data that can serve as a row of the data matrix
pub trait Row {
    /// Private
    type Scale: Copy;

    /// Append this row to a buffer
    fn append_to(self, buffer: &mut Vec<u8>, scale: Self::Scale);
    /// Number of columns of the row
    fn ncols() -> usize;
}

fn write_f64(w: &mut impl std::io::Write, f: f64) -> std::io::Result<()> {
    w.write_all(&f.to_bits().to_le_bytes())
}

impl<A, B> Row for (A, B)
where
    A: Data,
    B: Data,
{
    type Scale = (f64, f64);

    fn append_to(self, buffer: &mut Vec<u8>, scale: (f64, f64)) {
        let (a, b) = self;

        write_f64(buffer, a.f64() * scale.0).unwrap();
        write_f64(buffer, b.f64() * scale.1).unwrap();
    }

    fn ncols() -> usize {
        2
    }
}

impl<A, B, C> Row for (A, B, C)
where
    A: Data,
    B: Data,
    C: Data,
{
    type Scale = (f64, f64, f64);

    fn append_to(self, buffer: &mut Vec<u8>, scale: (f64, f64, f64)) {
        let (a, b, c) = self;

        write_f64(buffer, a.f64() * scale.0).unwrap();
        write_f64(buffer, b.f64() * scale.1).unwrap();
        write_f64(buffer, c.f64() * scale.2).unwrap();
    }

    fn ncols() -> usize {
        3
    }
}

impl<A, B, C, D> Row for (A, B, C, D)
where
    A: Data,
    B: Data,
    C: Data,
    D: Data,
{
    type Scale = (f64, f64, f64, f64);

    fn append_to(self, buffer: &mut Vec<u8>, scale: (f64, f64, f64, f64)) {
        let (a, b, c, d) = self;

        write_f64(buffer, a.f64() * scale.0).unwrap();
        write_f64(buffer, b.f64() * scale.1).unwrap();
        write_f64(buffer, c.f64() * scale.2).unwrap();
        write_f64(buffer, d.f64() * scale.3).unwrap();
    }

    fn ncols() -> usize {
        4
    }
}

impl<A, B, C, D, E> Row for (A, B, C, D, E)
where
    A: Data,
    B: Data,
    C: Data,
    D: Data,
    E: Data,
{
    type Scale = (f64, f64, f64, f64, f64);

    #[allow(clippy::many_single_char_names)]
    fn append_to(self, buffer: &mut Vec<u8>, scale: (f64, f64, f64, f64, f64)) {
        let (a, b, c, d, e) = self;

        write_f64(buffer, a.f64() * scale.0).unwrap();
        write_f64(buffer, b.f64() * scale.1).unwrap();
        write_f64(buffer, c.f64() * scale.2).unwrap();
        write_f64(buffer, d.f64() * scale.3).unwrap();
        write_f64(buffer, e.f64() * scale.4).unwrap();
    }

    fn ncols() -> usize {
        5
    }
}
