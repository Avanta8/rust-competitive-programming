#[derive(Default, Debug)]
struct Map<T> {
    vec: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Map<T> {
    fn add(&mut self, item: T) -> (usize, &T) {
        let idx = self.free.pop().unwrap_or_else(|| {
            let idx = self.vec.len();
            self.vec.push(None);
            idx
        });
        assert!(idx < self.vec.len());
        assert!(self.vec[idx].is_none());
        self.vec[idx] = Some(item);
        (idx, self.vec[idx].as_ref().unwrap())
    }

    fn get(&self, idx: usize) -> Option<&T> {
        self.vec.get(idx).map(|x| x.as_ref()).flatten()
    }

    fn remove(&mut self, idx: usize) -> Option<T> {
        if let Some(item) = self.vec.get_mut(idx) {
            self.free.push(idx);
            std::mem::take(item)
        } else {
            None
        }
    }
}

fn main() {}
