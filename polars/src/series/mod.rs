//! Type agnostic columnar data structure.
pub use crate::prelude::CmpOps;
use crate::prelude::*;
use arrow::{array::ArrayRef, buffer::Buffer};
use std::mem;

pub(crate) mod aggregate;
pub(crate) mod arithmetic;
mod comparison;
pub(crate) mod iterator;
use enum_dispatch::enum_dispatch;

/// # Series
/// The columnar data type for a DataFrame. The [Series enum](enum.Series.html) consists
/// of typed [ChunkedArray](../chunked_array/struct.ChunkedArray.html)'s. To quickly cast
/// a `Series` to a `ChunkedArray` you can call the method with the name of the type:
///
/// ```
/// # use polars::prelude::*;
/// let s: Series = [1, 2, 3].iter().collect();
/// // Quickly obtain the ChunkedArray wrapped by the Series.
/// let chunked_array = s.i32().unwrap();
/// ```
///
/// ## Arithmetic
///
/// You can do standard arithmetic on series.
/// ```
/// # use polars::prelude::*;
/// let s: Series = [1, 2, 3].iter().collect();
/// let out_add = &s + &s;
/// let out_sub = &s - &s;
/// let out_div = &s / &s;
/// let out_mul = &s * &s;
/// ```
///
/// Or with series and numbers.
///
/// ```
/// # use polars::prelude::*;
/// let s: Series = (1..3).collect();
/// let out_add_one = &s + 1;
/// let out_multiply = &s * 10;
///
/// // Could not overload left hand side operator.
/// let out_divide = 1.div(&s);
/// let out_add = 1.add(&s);
/// let out_subtract = 1.sub(&s);
/// let out_multiply = 1.mul(&s);
/// ```
///
/// ## Comparison
/// You can obtain boolean mask by comparing series.
///
/// ```
/// # use polars::prelude::*;
/// use itertools::Itertools;
/// let s = Series::new("dollars", &[1, 2, 3]);
/// let mask = s.eq(1);
/// let valid = [true, false, false].iter();
/// assert!(mask
///     .into_iter()
///     .map(|opt_bool| opt_bool.unwrap()) // option, because series can be null
///     .zip(valid)
///     .all(|(a, b)| a == *b))
/// ```
///
/// See all the comparison operators in the [CmpOps trait](../chunked_array/comparison/trait.CmpOps.html)
///
/// ## Iterators
/// The Series variants contain differently typed [ChunkedArray's](../chunked_array/struct.ChunkedArray.html).
/// These structs can be turned into iterators, making it possible to use any function/ closure you want
/// on a Series.
///
/// These iterators return an `Option<T>` because the values of a series may be null.
///
/// ```
/// use polars::prelude::*;
/// let pi = 3.14;
/// let s = Series::new("angle", [2f32 * pi, pi, 1.5 * pi].as_ref());
/// let s_cos: Series = s.f32()
///                     .expect("series was not an f32 dtype")
///                     .into_iter()
///                     .map(|opt_angle| opt_angle.map(|angle| angle.cos()))
///                     .collect();
/// ```
///
/// ## Creation
/// Series can be create from different data structures. Below we'll show a few ways we can create
/// a Series object.
///
/// ```
/// # use polars::prelude::*;
/// // Series van be created from Vec's, slices and arrays
/// Series::new("boolean series", &vec![true, false, true]);
/// Series::new("int series", &[1, 2, 3]);
/// // And can be nullable
/// Series::new("got nulls", &[Some(1), None, Some(2)]);
///
/// // Series can also be collected from iterators
/// let from_iter: Series = (0..10)
///     .into_iter()
///     .collect();
///
/// ```
#[enum_dispatch]
#[derive(Clone)]
pub enum Series {
    UInt8(ChunkedArray<UInt8Type>),
    UInt16(ChunkedArray<UInt16Type>),
    UInt32(ChunkedArray<UInt32Type>),
    UInt64(ChunkedArray<UInt64Type>),
    Int8(ChunkedArray<Int8Type>),
    Int16(ChunkedArray<Int16Type>),
    Int32(ChunkedArray<Int32Type>),
    Int64(ChunkedArray<Int64Type>),
    Float32(ChunkedArray<Float32Type>),
    Float64(ChunkedArray<Float64Type>),
    Utf8(ChunkedArray<Utf8Type>),
    Bool(ChunkedArray<BooleanType>),
    Date32(ChunkedArray<Date32Type>),
    Date64(ChunkedArray<Date64Type>),
    Time32Millisecond(Time32MillisecondChunked),
    Time32Second(Time32SecondChunked),
    Time64Nanosecond(ChunkedArray<Time64NanosecondType>),
    Time64Microsecond(ChunkedArray<Time64MicrosecondType>),
    DurationNanosecond(ChunkedArray<DurationNanosecondType>),
    DurationMicrosecond(DurationMicrosecondChunked),
    DurationMillisecond(DurationMillisecondChunked),
    DurationSecond(DurationSecondChunked),
    IntervalDayTime(IntervalDayTimeChunked),
    IntervalYearMonth(IntervalYearMonthChunked),
    TimestampNanosecond(TimestampNanosecondChunked),
    TimestampMicrosecond(TimestampMicrosecondChunked),
    TimestampMillisecond(TimestampMillisecondChunked),
    TimestampSecond(TimestampSecondChunked),
}

#[macro_export]
macro_rules! apply_method_all_series {
    ($self:ident, $method:ident, $($args:expr),*) => {
        match $self {
            Series::Utf8(a) => a.$method($($args),*),
            Series::Bool(a) => a.$method($($args),*),
            Series::UInt8(a) => a.$method($($args),*),
            Series::UInt16(a) => a.$method($($args),*),
            Series::UInt32(a) => a.$method($($args),*),
            Series::UInt64(a) => a.$method($($args),*),
            Series::Int8(a) => a.$method($($args),*),
            Series::Int16(a) => a.$method($($args),*),
            Series::Int32(a) => a.$method($($args),*),
            Series::Int64(a) => a.$method($($args),*),
            Series::Float32(a) => a.$method($($args),*),
            Series::Float64(a) => a.$method($($args),*),
            Series::Date32(a) => a.$method($($args),*),
            Series::Date64(a) => a.$method($($args),*),
            Series::Time32Millisecond(a) => a.$method($($args),*),
            Series::Time32Second(a) => a.$method($($args),*),
            Series::Time64Nanosecond(a) => a.$method($($args),*),
            Series::Time64Microsecond(a) => a.$method($($args),*),
            Series::DurationNanosecond(a) => a.$method($($args),*),
            Series::DurationMicrosecond(a) => a.$method($($args),*),
            Series::DurationMillisecond(a) => a.$method($($args),*),
            Series::DurationSecond(a) => a.$method($($args),*),
            Series::TimestampNanosecond(a) => a.$method($($args),*),
            Series::TimestampMicrosecond(a) => a.$method($($args),*),
            Series::TimestampMillisecond(a) => a.$method($($args),*),
            Series::TimestampSecond(a) => a.$method($($args),*),
            Series::IntervalDayTime(a) => a.$method($($args),*),
            Series::IntervalYearMonth(a) => a.$method($($args),*),
        }
    }
}

// doesn't include Bool and Utf8
#[macro_export]
macro_rules! apply_method_numeric_series {
    ($self:ident, $method:ident, $($args:expr),*) => {
        match $self {
            Series::UInt8(a) => a.$method($($args),*),
            Series::UInt16(a) => a.$method($($args),*),
            Series::UInt32(a) => a.$method($($args),*),
            Series::UInt64(a) => a.$method($($args),*),
            Series::Int8(a) => a.$method($($args),*),
            Series::Int16(a) => a.$method($($args),*),
            Series::Int32(a) => a.$method($($args),*),
            Series::Int64(a) => a.$method($($args),*),
            Series::Float32(a) => a.$method($($args),*),
            Series::Float64(a) => a.$method($($args),*),
            Series::Date32(a) => a.$method($($args),*),
            Series::Date64(a) => a.$method($($args),*),
            Series::Time32Millisecond(a) => a.$method($($args),*),
            Series::Time32Second(a) => a.$method($($args),*),
            Series::Time64Nanosecond(a) => a.$method($($args),*),
            Series::Time64Microsecond(a) => a.$method($($args),*),
            Series::DurationNanosecond(a) => a.$method($($args),*),
            Series::DurationMicrosecond(a) => a.$method($($args),*),
            Series::DurationMillisecond(a) => a.$method($($args),*),
            Series::DurationSecond(a) => a.$method($($args),*),
            Series::TimestampNanosecond(a) => a.$method($($args),*),
            Series::TimestampMicrosecond(a) => a.$method($($args),*),
            Series::TimestampMillisecond(a) => a.$method($($args),*),
            Series::TimestampSecond(a) => a.$method($($args),*),
            Series::IntervalDayTime(a) => a.$method($($args),*),
            Series::IntervalYearMonth(a) => a.$method($($args),*),
            _ => unimplemented!(),
        }
    }
}

#[macro_export]
macro_rules! apply_method_numeric_series_and_return {
    ($self:ident, $method:ident, [$($args:expr),*], $($opt_question_mark:tt)*) => {
        match $self {
            Series::UInt8(a) => Series::UInt8(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt16(a) => Series::UInt16(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt32(a) => Series::UInt32(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt64(a) => Series::UInt64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int8(a) => Series::Int8(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int16(a) => Series::Int16(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int32(a) => Series::Int32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int64(a) => Series::Int64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Float32(a) => Series::Float32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Float64(a) => Series::Float64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Date32(a) => Series::Date32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Date64(a) => Series::Date64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time32Millisecond(a) => Series::Time32Millisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time32Second(a) => Series::Time32Second(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time64Nanosecond(a) => Series::Time64Nanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time64Microsecond(a) => Series::Time64Microsecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationNanosecond(a) => Series::DurationNanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationMicrosecond(a) => Series::DurationMicrosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationMillisecond(a) => Series::DurationMillisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationSecond(a) => Series::DurationSecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampNanosecond(a) => Series::TimestampNanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampMicrosecond(a) => Series::TimestampMicrosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampMillisecond(a) => Series::TimestampMillisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampSecond(a) => Series::TimestampSecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::IntervalDayTime(a) => Series::IntervalDayTime(a.$method($($args),*)$($opt_question_mark)*),
            Series::IntervalYearMonth(a) => Series::IntervalYearMonth(a.$method($($args),*)$($opt_question_mark)*),
            _ => unimplemented!()
        }
    }
}

macro_rules! apply_method_all_series_and_return {
    ($self:ident, $method:ident, [$($args:expr),*], $($opt_question_mark:tt)*) => {
        match $self {
            Series::UInt8(a) => Series::UInt8(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt16(a) => Series::UInt16(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt32(a) => Series::UInt32(a.$method($($args),*)$($opt_question_mark)*),
            Series::UInt64(a) => Series::UInt64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int8(a) => Series::Int8(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int16(a) => Series::Int16(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int32(a) => Series::Int32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Int64(a) => Series::Int64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Float32(a) => Series::Float32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Float64(a) => Series::Float64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Utf8(a) => Series::Utf8(a.$method($($args),*)$($opt_question_mark)*),
            Series::Bool(a) => Series::Bool(a.$method($($args),*)$($opt_question_mark)*),
            Series::Date32(a) => Series::Date32(a.$method($($args),*)$($opt_question_mark)*),
            Series::Date64(a) => Series::Date64(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time32Millisecond(a) => Series::Time32Millisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time32Second(a) => Series::Time32Second(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time64Nanosecond(a) => Series::Time64Nanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::Time64Microsecond(a) => Series::Time64Microsecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationNanosecond(a) => Series::DurationNanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationMicrosecond(a) => Series::DurationMicrosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationMillisecond(a) => Series::DurationMillisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::DurationSecond(a) => Series::DurationSecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampNanosecond(a) => Series::TimestampNanosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampMicrosecond(a) => Series::TimestampMicrosecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampMillisecond(a) => Series::TimestampMillisecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::TimestampSecond(a) => Series::TimestampSecond(a.$method($($args),*)$($opt_question_mark)*),
            Series::IntervalDayTime(a) => Series::IntervalDayTime(a.$method($($args),*)$($opt_question_mark)*),
            Series::IntervalYearMonth(a) => Series::IntervalYearMonth(a.$method($($args),*)$($opt_question_mark)*),
        }
    }
}

macro_rules! unpack_series {
    ($self:ident, $variant:ident) => {
        if let Series::$variant(ca) = $self {
            Ok(ca)
        } else {
            Err(PolarsError::DataTypeMisMatch)
        }
    };
}

impl Series {
    pub fn from_chunked_array<T: PolarsDataType>(ca: ChunkedArray<T>) -> Self {
        pack_ca_to_series(ca)
    }

    /// Get the lengths of the underlying chunks
    pub fn chunk_lengths(&self) -> &Vec<usize> {
        apply_method_all_series!(self, chunk_id,)
    }
    /// Name of series.
    pub fn name(&self) -> &str {
        apply_method_all_series!(self, name,)
    }

    /// Rename series.
    pub fn rename(&mut self, name: &str) {
        apply_method_all_series!(self, rename, name)
    }

    /// Get field (used in schema)
    pub fn field(&self) -> &Field {
        apply_method_all_series!(self, ref_field,)
    }

    /// Get datatype of series.
    pub fn dtype(&self) -> &ArrowDataType {
        self.field().data_type()
    }

    /// Underlying chunks.
    pub fn chunks(&self) -> &Vec<ArrayRef> {
        apply_method_all_series!(self, chunks,)
    }

    /// No. of chunks
    pub fn n_chunks(&self) -> usize {
        self.chunks().len()
    }

    pub fn i8(&self) -> Result<&Int8Chunked> {
        unpack_series!(self, Int8)
    }

    pub fn i16(&self) -> Result<&Int16Chunked> {
        unpack_series!(self, Int16)
    }

    /// Unpack to ChunkedArray
    /// ```
    /// # use polars::prelude::*;
    /// let s: Series = [1, 2, 3].iter().collect();
    /// let s_squared: Series = s.i32()
    ///     .unwrap()
    ///     .into_iter()
    ///     .map(|opt_v| {
    ///         match opt_v {
    ///             Some(v) => Some(v * v),
    ///             None => None, // null value
    ///         }
    /// }).collect();
    /// ```
    pub fn i32(&self) -> Result<&Int32Chunked> {
        unpack_series!(self, Int32)
    }

    /// Unpack to ChunkedArray
    pub fn i64(&self) -> Result<&Int64Chunked> {
        unpack_series!(self, Int64)
    }

    /// Unpack to ChunkedArray
    pub fn f32(&self) -> Result<&Float32Chunked> {
        unpack_series!(self, Float32)
    }

    /// Unpack to ChunkedArray
    pub fn f64(&self) -> Result<&Float64Chunked> {
        unpack_series!(self, Float64)
    }

    /// Unpack to ChunkedArray
    pub fn u8(&self) -> Result<&UInt8Chunked> {
        unpack_series!(self, UInt8)
    }

    /// Unpack to ChunkedArray
    pub fn u16(&self) -> Result<&UInt16Chunked> {
        unpack_series!(self, UInt16)
    }

    /// Unpack to ChunkedArray
    pub fn u32(&self) -> Result<&UInt32Chunked> {
        unpack_series!(self, UInt32)
    }

    /// Unpack to ChunkedArray
    pub fn u64(&self) -> Result<&UInt64Chunked> {
        unpack_series!(self, UInt64)
    }

    /// Unpack to ChunkedArray
    pub fn bool(&self) -> Result<&BooleanChunked> {
        unpack_series!(self, Bool)
    }

    /// Unpack to ChunkedArray
    pub fn utf8(&self) -> Result<&Utf8Chunked> {
        unpack_series!(self, Utf8)
    }

    /// Unpack to ChunkedArray
    pub fn date32(&self) -> Result<&Date32Chunked> {
        unpack_series!(self, Date32)
    }

    /// Unpack to ChunkedArray
    pub fn date64(&self) -> Result<&Date64Chunked> {
        unpack_series!(self, Date64)
    }

    /// Unpack to ChunkedArray
    pub fn time32_millisecond(&self) -> Result<&Time32MillisecondChunked> {
        unpack_series!(self, Time32Millisecond)
    }

    /// Unpack to ChunkedArray
    pub fn time32_second(&self) -> Result<&Time32SecondChunked> {
        unpack_series!(self, Time32Second)
    }

    /// Unpack to ChunkedArray
    pub fn time64_nanosecond(&self) -> Result<&Time64NanosecondChunked> {
        unpack_series!(self, Time64Nanosecond)
    }

    /// Unpack to ChunkedArray
    pub fn time64_microsecond(&self) -> Result<&Time64MicrosecondChunked> {
        unpack_series!(self, Time64Microsecond)
    }

    /// Unpack to ChunkedArray
    pub fn duration_nanosecond(&self) -> Result<&DurationNanosecondChunked> {
        unpack_series!(self, DurationNanosecond)
    }

    /// Unpack to ChunkedArray
    pub fn duration_microsecond(&self) -> Result<&DurationMicrosecondChunked> {
        unpack_series!(self, DurationMicrosecond)
    }

    /// Unpack to ChunkedArray
    pub fn duration_millisecond(&self) -> Result<&DurationMillisecondChunked> {
        unpack_series!(self, DurationMillisecond)
    }

    /// Unpack to ChunkedArray
    pub fn duration_second(&self) -> Result<&DurationSecondChunked> {
        unpack_series!(self, DurationSecond)
    }

    /// Unpack to ChunkedArray
    pub fn timestamp_nanosecond(&self) -> Result<&TimestampNanosecondChunked> {
        unpack_series!(self, TimestampNanosecond)
    }

    /// Unpack to ChunkedArray
    pub fn timestamp_microsecond(&self) -> Result<&TimestampMicrosecondChunked> {
        unpack_series!(self, TimestampMicrosecond)
    }

    /// Unpack to ChunkedArray
    pub fn timestamp_millisecond(&self) -> Result<&TimestampMillisecondChunked> {
        unpack_series!(self, TimestampMillisecond)
    }

    /// Unpack to ChunkedArray
    pub fn timestamp_second(&self) -> Result<&TimestampSecondChunked> {
        unpack_series!(self, TimestampSecond)
    }

    /// Unpack to ChunkedArray
    pub fn interval_daytime(&self) -> Result<&IntervalDayTimeChunked> {
        unpack_series!(self, IntervalDayTime)
    }

    /// Unpack to ChunkedArray
    pub fn interval_year_month(&self) -> Result<&IntervalYearMonthChunked> {
        unpack_series!(self, IntervalYearMonth)
    }

    pub fn append_array(&mut self, other: ArrayRef) -> Result<()> {
        apply_method_all_series!(self, append_array, other)
    }

    /// Take `num_elements` from the top as a zero copy view.
    pub fn limit(&self, num_elements: usize) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, limit, [num_elements], ?))
    }

    /// Get a zero copy view of the data.
    pub fn slice(&self, offset: usize, length: usize) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, slice, [offset, length], ?))
    }

    /// Append a Series of the same type in place.
    pub fn append(&mut self, other: &Self) -> Result<()> {
        match self {
            Series::Utf8(arr) => arr.append(other.utf8()?),
            Series::Bool(arr) => arr.append(other.bool()?),
            Series::UInt8(arr) => arr.append(other.u8()?),
            Series::UInt16(arr) => arr.append(other.u16()?),
            Series::UInt32(arr) => arr.append(other.u32()?),
            Series::UInt64(arr) => arr.append(other.u64()?),
            Series::Int8(arr) => arr.append(other.i8()?),
            Series::Int16(arr) => arr.append(other.i16()?),
            Series::Int32(arr) => arr.append(other.i32()?),
            Series::Int64(arr) => arr.append(other.i64()?),
            Series::Float32(arr) => arr.append(other.f32()?),
            Series::Float64(arr) => arr.append(other.f64()?),
            Series::Date32(arr) => arr.append(other.date32()?),
            Series::Date64(arr) => arr.append(other.date64()?),
            Series::Time32Millisecond(arr) => arr.append(other.time32_millisecond()?),
            Series::Time32Second(arr) => arr.append(other.time32_second()?),
            Series::Time64Nanosecond(arr) => arr.append(other.time64_nanosecond()?),
            Series::Time64Microsecond(arr) => arr.append(other.time64_microsecond()?),
            Series::DurationNanosecond(arr) => arr.append(other.duration_nanosecond()?),
            Series::DurationMillisecond(arr) => arr.append(other.duration_millisecond()?),
            Series::DurationMicrosecond(arr) => arr.append(other.duration_microsecond()?),
            Series::DurationSecond(arr) => arr.append(other.duration_second()?),
            Series::TimestampNanosecond(arr) => arr.append(other.timestamp_nanosecond()?),
            Series::TimestampMicrosecond(arr) => arr.append(other.timestamp_microsecond()?),
            Series::TimestampMillisecond(arr) => arr.append(other.timestamp_millisecond()?),
            Series::TimestampSecond(arr) => arr.append(other.timestamp_second()?),
            Series::IntervalDayTime(arr) => arr.append(other.interval_daytime()?),
            Series::IntervalYearMonth(arr) => arr.append(other.interval_year_month()?),
        };
        Ok(())
    }

    /// Filter by boolean mask. This operation clones data.
    pub fn filter<T: AsRef<BooleanChunked>>(&self, filter: T) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, filter, [filter.as_ref()], ?))
    }

    /// Take by index from an iterator. This operation clones the data.
    pub fn take_iter(
        &self,
        iter: impl Iterator<Item = usize>,
        capacity: Option<usize>,
    ) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, take, [iter,  capacity], ?))
    }

    /// Take by index from an iterator. This operation clones the data.
    pub fn take_opt_iter(
        &self,
        iter: impl Iterator<Item = Option<usize>>,
        capacity: Option<usize>,
    ) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, take_opt, [iter,  capacity], ?))
    }

    /// Take by index. This operation is clone.
    pub fn take<T: TakeIndex>(&self, indices: &T) -> Result<Self> {
        let mut iter = indices.as_take_iter();
        let capacity = indices.take_index_len();
        self.take_iter(&mut iter, Some(capacity))
    }

    /// Get length of series.
    pub fn len(&self) -> usize {
        apply_method_all_series!(self, len,)
    }

    /// Aggregate all chunks to a contiguous array of memory.
    pub fn rechunk(&self, chunk_lengths: Option<&[usize]>) -> Result<Self> {
        Ok(apply_method_all_series_and_return!(self, rechunk, [chunk_lengths], ?))
    }

    /// Get the head of the Series.
    pub fn head(&self, length: Option<usize>) -> Self {
        apply_method_all_series_and_return!(self, head, [length],)
    }

    /// Get the tail of the Series.
    pub fn tail(&self, length: Option<usize>) -> Self {
        apply_method_all_series_and_return!(self, tail, [length],)
    }

    /// Cast to some primitive type.
    pub fn cast<N>(&self) -> Result<Self>
    where
        N: PolarsDataType,
    {
        let s = match self {
            Series::Bool(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Utf8(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::UInt8(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::UInt16(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::UInt32(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::UInt64(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Int8(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Int16(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Int32(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Int64(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Float32(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Float64(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Date32(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Date64(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Time32Millisecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Time32Second(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Time64Nanosecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::Time64Microsecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::DurationNanosecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::DurationMicrosecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::DurationMillisecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::DurationSecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::TimestampNanosecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::TimestampMicrosecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::TimestampMillisecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::TimestampSecond(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::IntervalDayTime(arr) => pack_ca_to_series(arr.cast::<N>()?),
            Series::IntervalYearMonth(arr) => pack_ca_to_series(arr.cast::<N>()?),
        };
        Ok(s)
    }

    /// Get a single value by index. Don't use this operation for loops as a runtime cast is
    /// needed for every iteration.
    pub fn get(&self, index: usize) -> AnyType {
        apply_method_all_series!(self, get, index)
    }

    /// Sort in place.
    pub fn sort_in_place(&mut self, reverse: bool) {
        apply_method_all_series!(self, sort_in_place, reverse);
    }

    pub fn sort(&self, reverse: bool) -> Self {
        apply_method_all_series_and_return!(self, sort, [reverse],)
    }

    /// Retrieve the indexes needed for a sort.
    pub fn argsort(&self, reverse: bool) -> Vec<usize> {
        apply_method_all_series!(self, argsort, reverse)
    }

    /// Count the null values.
    pub fn null_count(&self) -> usize {
        apply_method_all_series!(self, null_count,)
    }

    /// Get unique values in the Series.
    pub fn unique(&self) -> Self {
        apply_method_all_series_and_return!(self, unique, [],)
    }

    /// Get first indexes of unique values.
    pub fn arg_unique(&self) -> Vec<usize> {
        apply_method_all_series!(self, arg_unique,)
    }

    /// Get a mask of the null values.
    pub fn is_null(&self) -> BooleanChunked {
        apply_method_all_series!(self, is_null,)
    }

    /// Get the bits that represent the null values of the underlying ChunkedArray
    pub fn null_bits(&self) -> Vec<(usize, Option<Buffer>)> {
        apply_method_all_series!(self, null_bits,)
    }

    /// return a Series in reversed order
    pub fn reverse(&self) -> Self {
        apply_method_all_series_and_return!(self, reverse, [],)
    }

    /// Rechunk and return a pointer to the start of the Series.
    /// Only implemented for numeric types
    pub fn as_single_ptr(&mut self) -> usize {
        apply_method_numeric_series!(self, as_single_ptr,)
    }
}

fn pack_ca_to_series<N: PolarsDataType>(ca: ChunkedArray<N>) -> Series {
    unsafe {
        match N::get_data_type() {
            ArrowDataType::Boolean => Series::Bool(mem::transmute(ca)),
            ArrowDataType::Utf8 => Series::Utf8(mem::transmute(ca)),
            ArrowDataType::UInt8 => Series::UInt8(mem::transmute(ca)),
            ArrowDataType::UInt16 => Series::UInt16(mem::transmute(ca)),
            ArrowDataType::UInt32 => Series::UInt32(mem::transmute(ca)),
            ArrowDataType::UInt64 => Series::UInt64(mem::transmute(ca)),
            ArrowDataType::Int8 => Series::Int8(mem::transmute(ca)),
            ArrowDataType::Int16 => Series::Int16(mem::transmute(ca)),
            ArrowDataType::Int32 => Series::Int32(mem::transmute(ca)),
            ArrowDataType::Int64 => Series::Int64(mem::transmute(ca)),
            ArrowDataType::Float32 => Series::Float32(mem::transmute(ca)),
            ArrowDataType::Float64 => Series::Float64(mem::transmute(ca)),
            ArrowDataType::Date32(DateUnit::Day) => Series::Date32(mem::transmute(ca)),
            ArrowDataType::Date64(DateUnit::Millisecond) => Series::Date64(mem::transmute(ca)),
            ArrowDataType::Time64(datatypes::TimeUnit::Microsecond) => {
                Series::Time64Microsecond(mem::transmute(ca))
            }
            ArrowDataType::Time64(datatypes::TimeUnit::Nanosecond) => {
                Series::Time64Nanosecond(mem::transmute(ca))
            }
            ArrowDataType::Time32(datatypes::TimeUnit::Millisecond) => {
                Series::Time32Millisecond(mem::transmute(ca))
            }
            ArrowDataType::Time32(datatypes::TimeUnit::Second) => {
                Series::Time32Second(mem::transmute(ca))
            }
            ArrowDataType::Duration(datatypes::TimeUnit::Nanosecond) => {
                Series::DurationNanosecond(mem::transmute(ca))
            }
            ArrowDataType::Duration(datatypes::TimeUnit::Microsecond) => {
                Series::DurationMicrosecond(mem::transmute(ca))
            }
            ArrowDataType::Duration(datatypes::TimeUnit::Millisecond) => {
                Series::DurationMillisecond(mem::transmute(ca))
            }
            ArrowDataType::Duration(datatypes::TimeUnit::Second) => {
                Series::DurationSecond(mem::transmute(ca))
            }
            ArrowDataType::Timestamp(TimeUnit::Nanosecond, _) => {
                Series::TimestampNanosecond(mem::transmute(ca))
            }
            ArrowDataType::Timestamp(TimeUnit::Microsecond, _) => {
                Series::TimestampMicrosecond(mem::transmute(ca))
            }
            ArrowDataType::Timestamp(TimeUnit::Millisecond, _) => {
                Series::TimestampMillisecond(mem::transmute(ca))
            }
            ArrowDataType::Timestamp(TimeUnit::Second, _) => {
                Series::TimestampSecond(mem::transmute(ca))
            }
            ArrowDataType::Interval(IntervalUnit::YearMonth) => {
                Series::IntervalYearMonth(mem::transmute(ca))
            }
            ArrowDataType::Interval(IntervalUnit::DayTime) => {
                Series::IntervalDayTime(mem::transmute(ca))
            }
            _ => panic!("Not implemented: {:?}", N::get_data_type()),
        }
    }
}

pub trait NamedFrom<T, Phantom: ?Sized> {
    /// Initialize by name and values.
    fn new(name: &str, _: T) -> Self;
}

macro_rules! impl_named_from {
    ($type:ty, $series_var:ident, $method:ident) => {
        impl<T: AsRef<$type>> NamedFrom<T, $type> for Series {
            fn new(name: &str, v: T) -> Self {
                Series::$series_var(ChunkedArray::$method(name, v.as_ref()))
            }
        }
    };
}

impl<'a, T: AsRef<[&'a str]>> NamedFrom<T, [&'a str]> for Series {
    fn new(name: &str, v: T) -> Self {
        Series::Utf8(ChunkedArray::new_utf8_from_slice(name, v.as_ref()))
    }
}
impl<'a, T: AsRef<[Option<&'a str>]>> NamedFrom<T, [Option<&'a str>]> for Series {
    fn new(name: &str, v: T) -> Self {
        Series::Utf8(ChunkedArray::new_utf8_from_opt_slice(name, v.as_ref()))
    }
}

impl_named_from!([String], Utf8, new_utf8_from_slice);
impl_named_from!([bool], Bool, new_from_slice);
impl_named_from!([u8], UInt8, new_from_slice);
impl_named_from!([u16], UInt16, new_from_slice);
impl_named_from!([u32], UInt32, new_from_slice);
impl_named_from!([u64], UInt64, new_from_slice);
impl_named_from!([i8], Int8, new_from_slice);
impl_named_from!([i16], Int16, new_from_slice);
impl_named_from!([i32], Int32, new_from_slice);
impl_named_from!([i64], Int64, new_from_slice);
impl_named_from!([f32], Float32, new_from_slice);
impl_named_from!([f64], Float64, new_from_slice);
impl_named_from!([Option<String>], Utf8, new_utf8_from_opt_slice);
impl_named_from!([Option<bool>], Bool, new_from_opt_slice);
impl_named_from!([Option<u8>], UInt8, new_from_opt_slice);
impl_named_from!([Option<u16>], UInt16, new_from_opt_slice);
impl_named_from!([Option<u32>], UInt32, new_from_opt_slice);
impl_named_from!([Option<u64>], UInt64, new_from_opt_slice);
impl_named_from!([Option<i8>], Int8, new_from_opt_slice);
impl_named_from!([Option<i16>], Int16, new_from_opt_slice);
impl_named_from!([Option<i32>], Int32, new_from_opt_slice);
impl_named_from!([Option<i64>], Int64, new_from_opt_slice);
impl_named_from!([Option<f32>], Float32, new_from_opt_slice);
impl_named_from!([Option<f64>], Float64, new_from_opt_slice);

macro_rules! impl_as_ref_ca {
    ($type:ident, $series_var:ident) => {
        impl AsRef<ChunkedArray<datatypes::$type>> for Series {
            fn as_ref(&self) -> &ChunkedArray<datatypes::$type> {
                match self {
                    Series::$series_var(a) => a,
                    _ => unimplemented!(),
                }
            }
        }
    };
}

impl_as_ref_ca!(UInt8Type, UInt8);
impl_as_ref_ca!(UInt16Type, UInt16);
impl_as_ref_ca!(UInt32Type, UInt32);
impl_as_ref_ca!(UInt64Type, UInt64);
impl_as_ref_ca!(Int8Type, Int8);
impl_as_ref_ca!(Int16Type, Int16);
impl_as_ref_ca!(Int32Type, Int32);
impl_as_ref_ca!(Int64Type, Int64);
impl_as_ref_ca!(Float32Type, Float32);
impl_as_ref_ca!(Float64Type, Float64);
impl_as_ref_ca!(BooleanType, Bool);
impl_as_ref_ca!(Utf8Type, Utf8);

macro_rules! impl_as_mut_ca {
    ($type:ident, $series_var:ident) => {
        impl AsMut<ChunkedArray<datatypes::$type>> for Series {
            fn as_mut(&mut self) -> &mut ChunkedArray<datatypes::$type> {
                match self {
                    Series::$series_var(a) => a,
                    _ => unimplemented!(),
                }
            }
        }
    };
}

impl_as_mut_ca!(UInt8Type, UInt8);
impl_as_mut_ca!(UInt16Type, UInt16);
impl_as_mut_ca!(UInt32Type, UInt32);
impl_as_mut_ca!(UInt64Type, UInt64);
impl_as_mut_ca!(Int8Type, Int8);
impl_as_mut_ca!(Int16Type, Int16);
impl_as_mut_ca!(Int32Type, Int32);
impl_as_mut_ca!(Int64Type, Int64);
impl_as_mut_ca!(Float32Type, Float32);
impl_as_mut_ca!(Float64Type, Float64);
impl_as_mut_ca!(BooleanType, Bool);
impl_as_mut_ca!(Utf8Type, Utf8);

macro_rules! from_series_to_ca {
    ($variant:ident, $ca:ident) => {
        impl<'a> From<&'a Series> for &'a $ca {
            fn from(s: &'a Series) -> Self {
                match s {
                    Series::$variant(ca) => ca,
                    _ => unimplemented!(),
                }
            }
        }
    };
}
from_series_to_ca!(UInt8, UInt8Chunked);
from_series_to_ca!(UInt16, UInt16Chunked);
from_series_to_ca!(UInt32, UInt32Chunked);
from_series_to_ca!(UInt64, UInt64Chunked);
from_series_to_ca!(Int8, Int8Chunked);
from_series_to_ca!(Int16, Int16Chunked);
from_series_to_ca!(Int32, Int32Chunked);
from_series_to_ca!(Int64, Int64Chunked);
from_series_to_ca!(Float32, Float32Chunked);
from_series_to_ca!(Float64, Float64Chunked);
from_series_to_ca!(Bool, BooleanChunked);
from_series_to_ca!(Utf8, Utf8Chunked);
from_series_to_ca!(Date32, Date32Chunked);
from_series_to_ca!(Date64, Date64Chunked);
from_series_to_ca!(Time32Millisecond, Time32MillisecondChunked);
from_series_to_ca!(Time32Second, Time32SecondChunked);
from_series_to_ca!(Time64Microsecond, Time64MicrosecondChunked);
from_series_to_ca!(Time64Nanosecond, Time64NanosecondChunked);
from_series_to_ca!(DurationMillisecond, DurationMillisecondChunked);
from_series_to_ca!(DurationSecond, DurationSecondChunked);
from_series_to_ca!(DurationMicrosecond, DurationMicrosecondChunked);
from_series_to_ca!(DurationNanosecond, DurationNanosecondChunked);
from_series_to_ca!(TimestampMillisecond, TimestampMillisecondChunked);
from_series_to_ca!(TimestampSecond, TimestampSecondChunked);
from_series_to_ca!(TimestampMicrosecond, TimestampMicrosecondChunked);
from_series_to_ca!(TimestampNanosecond, TimestampNanosecondChunked);
from_series_to_ca!(IntervalDayTime, IntervalDayTimeChunked);
from_series_to_ca!(IntervalYearMonth, IntervalYearMonthChunked);

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn cast() {
        let ar = ChunkedArray::<Int32Type>::new_from_slice("a", &[1, 2]);
        let s = Series::Int32(ar);
        let s2 = s.cast::<Int64Type>().unwrap();
        match s2 {
            Series::Int64(_) => assert!(true),
            _ => assert!(false),
        }
        let s2 = s.cast::<Float32Type>().unwrap();
        match s2 {
            Series::Float32(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn new_series() {
        Series::new("boolean series", &vec![true, false, true]);
        Series::new("int series", &[1, 2, 3]);
    }
}
