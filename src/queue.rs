
struct Item<T> {
    value: T,
    next: Box<Item<T>>
}
struct Queue<'a, T> {
    front: &'a Item<T>,
    back: &'a Item<T>,
}

impl<T> Queue<'_, Item<T>> {
    fn remove(mut self) -> &Item<T> {
        let temp:Item<T> = self.front;
        self.front = self.front.next;
        temp
    }
}