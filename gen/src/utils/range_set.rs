use num::Integer;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Display},
    ops::Range,
};

#[derive(Debug, Clone)]
pub struct RangeSet<T> {
    ranges: BTreeMap<T, T>,
}

impl<T> Default for RangeSet<T> {
    fn default() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }
}

impl<T> RangeSet<T>
where
    T: Integer + Copy + Debug + Display,
{
    pub fn first(&self) -> Option<Range<T>> {
        self.iter().next()
    }

    pub fn iter(&self) -> impl Iterator<Item = Range<T>> {
        self.ranges.iter().map(|(&from, &to)| from..to)
    }

    pub fn insert(&mut self, val: T) -> bool {
        // eprintln!("INSERT {val}");
        if let Some((&from2, &to2)) = self.ranges.range(val..).next() {
            if val + T::one() == from2 {
                if let Some((_, to1)) = self.ranges.range_mut(..=val).rev().next() {
                    if val == *to1 {
                        *to1 = to2;
                        self.ranges.remove(&from2);
                    } else {
                        self.ranges.remove(&from2);
                        self.ranges.insert(val, to2);
                    }
                } else {
                    self.ranges.remove(&from2);
                    self.ranges.insert(val, to2);
                }

                #[cfg(test)]
                self.check();

                return true;
            }
        }

        if let Some((_, to)) = self.ranges.range_mut(..=val).rev().next() {
            match val.cmp(to) {
                Ordering::Less => return false,
                Ordering::Equal => *to = *to + T::one(),
                Ordering::Greater => {
                    self.ranges.insert(val, val + T::one());
                }
            }
        } else {
            self.ranges.insert(val, val + T::one());
        }

        #[cfg(test)]
        self.check();

        true
    }

    pub fn remove(&mut self, val: T) -> bool {
        // eprintln!("REMOVE {val}");
        let Some((&from, &to)) = self.ranges.range(..=val).rev().next() else {
            #[cfg(test)]
            self.check();
            return false;
        };

        if val >= to {
            #[cfg(test)]
            self.check();
            return false;
        }

        let on_start = val == from;
        let on_end = val + T::one() == to;

        match (on_start, on_end) {
            (true, true) => {
                self.ranges.remove(&from);
            }
            (true, false) => {
                self.ranges.remove(&from);
                self.ranges.insert(from + T::one(), to);
            }
            (false, true) => {
                self.ranges.remove(&from);
                self.ranges.insert(from, to - T::one());
            }
            (false, false) => {
                self.ranges.remove(&from);
                self.ranges.insert(from, val);
                self.ranges.insert(val + T::one(), to);
            }
        }

        #[cfg(test)]
        self.check();

        true
    }

    #[cfg(test)]
    fn check(&self) {
        use itertools::Itertools;

        for ((from1, to1), (from2, to2)) in self.ranges.iter().tuple_windows() {
            debug_assert!(from1 < to1, "{:?}", self.ranges);
            debug_assert!(to1 < from2, "{:?}", self.ranges);
            debug_assert!(from2 < to2, "{:?}", self.ranges);
        }
    }
}
