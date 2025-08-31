use crate::{item::Item, utils::can_stack};

fn add_to_slot(slot: &mut Item, item: &mut Item) {
    let space = slot.max_stack - slot.amount;
    let to_add = item.amount.min(space);
    slot.amount += to_add;
    item.amount -= to_add;
}
#[derive(Clone, Copy, Debug)]
pub struct Inventory<const N: usize> {
    slots: [Option<Item>; N],
}

impl<const N: usize> Inventory<N> {
    pub fn new() -> Self {
        Self { slots: [None; N] }
    }
    pub fn add_item(&mut self, mut item: Item) -> usize {
        for slot in &mut self.slots {
            match slot {
                Some(s) if can_stack(s, &item) => {
                    add_to_slot(s, &mut item);
                }
                None if item.amount > 0 => {
                    let to_place = item.amount.min(item.max_stack);
                    let mut new_item = item;
                    new_item.amount = to_place;
                    *slot = Some(new_item);
                    item.amount -= to_place;
                }
                _ => {}
            }

            if item.amount == 0 {
                break;
            }
        }

        item.amount
    }

    pub fn remove_item(&mut self, mut item: Item) -> bool {
        for slot in self.slots.iter_mut().rev() {
            match slot {
                Some(s) if s.name == item.name => {
                    let to_remove = s.amount.min(item.amount);
                    s.amount -= to_remove;
                    item.amount -= to_remove;
                    if s.amount == 0 {
                        *slot = None;
                    }
                }
                _ => {}
            }
            if item.amount == 0 {
                break;
            }
        }
        item.amount == 0
    }

    pub fn get_items(&self) -> [Option<Item>; N] {
        self.slots
    }
}

pub trait HasInventory<const N: usize> {
    fn inventory(&self) -> &Inventory<N>;
    fn inventory_mut(&mut self) -> &mut Inventory<N>;
}
