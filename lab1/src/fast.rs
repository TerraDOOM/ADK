use std::{
    collections::BTreeMap,
    io::{self, Read, Write},
    mem,
};

// smolvec, from https://crates.io/crates/smolvec

// smolvec, from https://crates.io/crates/smolvec
mod smolvec {
    //! A lightweight vector implementation with small-vector optimization.
    //!
    //! `SmallVec<T>` is a vector implementation that optimizes for the case of small vectors
    //! by storing small arrays inline and only heap allocating for larger arrays.

    use std::alloc::{self, Layout};
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::iter::FromIterator;
    use std::mem::MaybeUninit;
    use std::ops::{Deref, DerefMut};
    use std::ptr;

    /// The number of elements that can be stored inline.
    const INLINE_CAPACITY: usize = 16;

    /// A vector implementation with small-vector optimization.
    ///
    /// `SmallVec<T>` stores up to `INLINE_CAPACITY` elements inline,
    /// and only allocates on the heap for larger numbers of elements.
    pub struct SmolVec<T> {
        len: usize,
        data: Data<T>,
    }

    enum Data<T> {
        Inline(MaybeUninit<[T; INLINE_CAPACITY]>),
        Heap { ptr: *mut T, capacity: usize },
    }

    impl<T> SmolVec<T> {
        /// Creates a new, empty `SmallVec<T>`.
        pub fn new() -> Self {
            SmolVec {
                len: 0,
                data: Data::Inline(MaybeUninit::uninit()),
            }
        }

        /// Creates a new `SmallVec<T>` with the specified capacity.
        pub fn with_capacity(capacity: usize) -> Self {
            if capacity <= INLINE_CAPACITY {
                Self::new()
            } else {
                let layout = Layout::array::<T>(capacity).unwrap();
                let ptr = unsafe { alloc::alloc(layout) as *mut T };
                SmolVec {
                    len: 0,
                    data: Data::Heap { ptr, capacity },
                }
            }
        }

        /// Appends an element to the back of the vector.
        pub fn push(&mut self, value: T) {
            if self.len == self.capacity() {
                self.grow();
            }
            unsafe {
                let ptr = self.as_mut_ptr().add(self.len);
                ptr::write(ptr, value);
            }
            self.len += 1;
        }

        /// Removes the last element from the vector and returns it, or `None` if it is empty.
        pub fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            } else {
                self.len -= 1;
                unsafe {
                    let ptr = self.as_mut_ptr().add(self.len);
                    Some(ptr::read(ptr))
                }
            }
        }

        pub fn clear(&mut self) {
            while self.pop().is_some() {}
        }

        /// Returns the number of elements the vector can hold without reallocating.
        pub fn capacity(&self) -> usize {
            match &self.data {
                Data::Inline(_) => INLINE_CAPACITY,
                Data::Heap { capacity, .. } => *capacity,
            }
        }

        /// Returns the number of elements in the vector.
        pub fn len(&self) -> usize {
            self.len
        }

        /// Returns `true` if the vector contains no elements.
        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        fn grow(&mut self) {
            let new_capacity = if self.capacity() == 0 {
                INLINE_CAPACITY
            } else {
                self.capacity() * 2
            };

            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            let new_ptr = unsafe { alloc::alloc(new_layout) as *mut T };

            let old_ptr = self.as_ptr();
            unsafe {
                ptr::copy_nonoverlapping(old_ptr, new_ptr, self.len);
            }

            if let Data::Heap { ptr, capacity } = self.data {
                unsafe {
                    let old_layout = Layout::array::<T>(capacity).unwrap();
                    alloc::dealloc(ptr as *mut u8, old_layout);
                }
            }

            self.data = Data::Heap {
                ptr: new_ptr,
                capacity: new_capacity,
            };
        }

        fn as_ptr(&self) -> *const T {
            match &self.data {
                Data::Inline(ref arr) => arr.as_ptr() as *const T,
                Data::Heap { ptr, .. } => *ptr as *const T,
            }
        }

        fn as_mut_ptr(&mut self) -> *mut T {
            match self.data {
                Data::Inline(ref mut arr) => arr.as_mut_ptr() as *mut T,
                Data::Heap { ptr, .. } => ptr,
            }
        }

        fn into_vec(self) -> Vec<T> {
            let mut vec = Vec::with_capacity(self.len);
            for i in 0..self.len {
                // We need to move the elements out of SmolVec into Vec
                unsafe {
                    vec.push(ptr::read(self.as_ptr().add(i)));
                }
            }

            // If SmolVec was using heap, deallocate the heap memory
            if let Data::Heap { ptr, capacity } = self.data {
                unsafe {
                    let layout = Layout::array::<T>(capacity).unwrap();
                    alloc::dealloc(ptr as *mut u8, layout);
                }
            }

            vec
        }
    }

    impl<T> Deref for SmolVec<T> {
        type Target = [T];

        fn deref(&self) -> &[T] {
            unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len) }
        }
    }

    impl<T> DerefMut for SmolVec<T> {
        fn deref_mut(&mut self) -> &mut [T] {
            unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
        }
    }

    impl<T> Drop for SmolVec<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::drop_in_place(&mut self[..]);
            }
            if let Data::Heap { ptr, capacity } = self.data {
                unsafe {
                    let layout = Layout::array::<T>(capacity).unwrap();
                    alloc::dealloc(ptr as *mut u8, layout);
                }
            }
        }
    }

    impl<T: Clone> Clone for SmolVec<T> {
        fn clone(&self) -> Self {
            let mut new_vec = SmolVec::with_capacity(self.len);
            new_vec.extend(self.iter().cloned());
            new_vec
        }
    }

    impl<T: fmt::Debug> fmt::Debug for SmolVec<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    impl<T: PartialEq> PartialEq for SmolVec<T> {
        fn eq(&self, other: &Self) -> bool {
            self.len() == other.len() && self.iter().eq(other.iter())
        }
    }

    impl<T> IntoIterator for SmolVec<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.into_vec().into_iter()
        }
    }

    impl<'a, T> IntoIterator for &'a SmolVec<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut SmolVec<T> {
        type Item = &'a mut T;
        type IntoIter = std::slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }

    impl<T: Eq> Eq for SmolVec<T> {}

    impl<T: PartialOrd> PartialOrd for SmolVec<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.iter().partial_cmp(other.iter())
        }
    }

    impl<T: Ord> Ord for SmolVec<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.iter().cmp(other.iter())
        }
    }

    impl<T: Hash> Hash for SmolVec<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.len().hash(state);
            for item in self {
                item.hash(state);
            }
        }
    }

    impl<T> Default for SmolVec<T> {
        fn default() -> Self {
            SmolVec::new()
        }
    }

    impl<T> Extend<T> for SmolVec<T> {
        fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
            for item in iter {
                self.push(item);
            }
        }
    }

    impl<T> FromIterator<T> for SmolVec<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut vec = SmolVec::new();
            vec.extend(iter);
            vec
        }
    }
}

use smolvec::SmolVec;

macro_rules! smolvec {
    [$($e:expr),*] => {
        {
            let mut v = SmolVec::new();
            $(v.push($e);)*
                v
        }
    }
}

#[derive(Debug, Clone)]
enum MapValue {
    Multi(SmolVec<i32>),
    Single(i32),
}

impl MapValue {
    fn get(&self) -> Option<i32> {
        match self {
            MapValue::Multi(v) => v.last().copied(),
            MapValue::Single(n) => Some(*n),
        }
    }
}

impl PartialEq for MapValue {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl Eq for MapValue {}

impl PartialOrd for MapValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get()
            .partial_cmp(&other.get())
            .or(Some(std::cmp::Ordering::Greater))
    }
}

impl Ord for MapValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get()
            .partial_cmp(&other.get())
            .unwrap_or(std::cmp::Ordering::Greater)
    }
}

#[derive(Default)]
struct ArrayButLmao {
    sets: Vec<u32>,
    map: BTreeMap<u32, MapValue>,
}

impl ArrayButLmao {
    fn new() -> Self {
        ArrayButLmao {
            sets: Vec::with_capacity(10_000),
            map: BTreeMap::new(),
        }
    }
    fn set(&mut self, i: u32, e: i32) {
        self.sets.push(i);
        self.map
            .entry(i)
            .and_modify(|x| match x {
                MapValue::Multi(v) => v.push(e),
                v @ &mut MapValue::Single(n) => *v = MapValue::Multi(smolvec![n, e]),
            })
            .or_insert(MapValue::Single(e));
    }
    fn get(&self, i: u32) -> Option<i32> {
        match self.map.get(&i)? {
            MapValue::Multi(v) => v.last().copied(),
            MapValue::Single(n) => Some(*n),
        }
    }
    fn unset(&mut self) {
        if let Some(i) = self.sets.pop() {
            match self.map.get_mut(&i) {
                Some(MapValue::Multi(v)) => {
                    v.pop();
                }
                Some(MapValue::Single(_)) => {
                    self.map.remove(&i);
                }
                None => {}
            }
        }
    }
    fn maxininterval(&mut self, lo: u32, hi: u32) -> Option<i32> {
        if lo > hi {
            return None;
        }

        self.map
            .range(lo..=hi)
            .map(|(_, e)| e)
            .max()
            .and_then(|x| x.get())
    }
}

macro_rules! next {
    ($e:expr) => {{
        let Some(x) = $e.next() else { continue };
        x
    }};
}

fn main() {
    let mut stdin = io::stdin().lock();
    let stdout = io::stdout().lock();
    let mut writer = io::BufWriter::with_capacity(100_000, stdout);
    let mut s = String::with_capacity(100_000);
    let _ = stdin.read_to_string(&mut s);
    let mut lines = s.lines();
    let mut getline = || lines.next();

    let uint = |s: &str| s.parse::<u32>().ok();
    let mut array = ArrayButLmao::new();

    while let Some(line) = getline() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split_whitespace();
        match next!(split) {
            "get" => {
                let Some(i) = uint(next!(split)) else {
                    continue;
                };
                let _ = writeln!(&mut writer, "{}", array.get(i).unwrap_or(0));
            }
            "set" => {
                let Some(i) = uint(next!(split)) else {
                    continue;
                };
                let Some(e) = uint(next!(split)) else {
                    continue;
                };
                array.set(i, e as i32);
            }
            "unset" => array.unset(),
            "maxininterval" => {
                let Some(lo) = uint(next!(split)) else {
                    continue;
                };
                let Some(hi) = uint(next!(split)) else {
                    continue;
                };

                let _ = writeln!(&mut writer, "{}", array.maxininterval(lo, hi).unwrap_or(0));
            }
            _ => break,
        }
    }
    let _ = writer.flush();
    mem::forget(writer);
    mem::forget(s);
    mem::forget(array);
}
