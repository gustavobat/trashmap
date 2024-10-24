use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

const LOAD_FACTOR: f64 = 0.75;

struct Bucket<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> Bucket<K, V> {
    fn new() -> Bucket<K, V>
    where
        K: Eq + Hash,
    {
        Bucket { data: Vec::new() }
    }
}

impl<K, V> Bucket<K, V> {
    fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut (K, V)> {
        self.data.iter_mut()
    }

    fn push(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }
}

pub struct Chained<K, V> {
    buckets: Vec<Bucket<K, V>>,
    len: usize,
}

impl<K, V> Chained<K, V> {
    pub fn new() -> Self {
        Chained {
            buckets: Vec::new(),
            len: 0,
        }
    }
}

impl<K, V> Default for Chained<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Chained<K, V>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.is_empty() || self.len as f64 >= self.buckets.len() as f64 * LOAD_FACTOR {
            self.resize();
        }

        let n_buckets = self.buckets.len();
        let bucket_index = Self::bucket_index(&key, n_buckets);
        let bucket = &mut self.buckets[bucket_index];

        let x = bucket.iter_mut().find(|(k, _)| k == &key);
        if let Some((_, v)) = x {
            let old_value = std::mem::replace(v, value);
            Some(old_value)
        } else {
            bucket.push(key, value);
            self.len += 1;
            None
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let n_buckets = self.buckets.len();
        let bucket_index = Self::bucket_index(&key, n_buckets);
        let bucket = &mut self.buckets[bucket_index];

        let i = bucket.iter().position(|(k, _)| k == &key)?;
        let (_, v) = bucket.data.swap_remove(i);

        self.len -= 1;
        Some(v)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let n_buckets = self.buckets.len();
        let bucket_index = Self::bucket_index(&key, n_buckets);
        let bucket = &self.buckets[bucket_index];
        bucket.iter().find(|(k, _)| k == &key).map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn bucket_index(key: &K, n_buckets: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % n_buckets as u64) as usize
    }

    fn resize(&mut self) {
        let target_size = match self.len {
            0 => 1,
            n => n * 2,
        };
        let mut new_buckets = Vec::<Bucket<K, V>>::with_capacity(target_size);
        for _ in 0..target_size {
            new_buckets.push(Bucket::new());
        }
        for bucket in self.buckets.iter_mut() {
            for (key, value) in bucket.data.drain(..) {
                let bucket_index = Self::bucket_index(&key, target_size);
                new_buckets[bucket_index].push(key, value);
            }
        }

        std::mem::swap(&mut self.buckets, &mut new_buckets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let chained: Chained<i32, i32> = Chained::new();
        assert_eq!(chained.buckets.len(), 0);
        assert_eq!(chained.len, 0);
        assert!(chained.is_empty());
    }

    #[test]
    fn operations() {
        let mut chained = Chained::new();

        chained.insert("foo", 10);
        assert_eq!(chained.len(), 1);
        assert!(!chained.is_empty());
        assert_eq!(chained.get("foo"), Some(&10));
        assert_eq!(chained.get("bar"), None);

        chained.insert("foo", 20);
        assert_eq!(chained.len(), 1);
        assert_eq!(chained.get("foo"), Some(&20));

        chained.remove("foo");
        assert_eq!(chained.len(), 0);
        assert!(chained.is_empty());
        assert_eq!(chained.get("foo"), None);
    }
}
