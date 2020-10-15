use crate::utils::HasId;

pub trait AddUnique<T>
where
    T: HasId,
{
    fn add_unique(&mut self, item: T);
    fn set_unique(&mut self, item: T);
}

impl<T> AddUnique<T> for Vec<T>
where
    T: HasId,
{
    fn add_unique(&mut self, item: T) {
        for it in self.iter() {
            if it.id() == item.id() {
                return;
            }
        }

        self.push(item);
    }

    fn set_unique(&mut self, item: T) {
        for it in self.iter_mut() {
            if it.id() == item.id() {
                *it = item;
                return;
            }
        }

        self.push(item);
    }
}
