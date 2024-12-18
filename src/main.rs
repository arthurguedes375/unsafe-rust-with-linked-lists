use std::{
    mem::{self, ManuallyDrop},
    ptr,
};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
    pub fn get(&self, i: usize) -> Option<&Node<T>> {
        if i == 0 {
            return Some(self);
        }

        if let Some(next) = &self.next {
            return (*next).get(i - 1);
        }

        None
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Node<T>> {
        if i == 0 {
            return Some(self);
        }

        if let Some(next) = &mut self.next {
            return (*next).get_mut(i - 1);
        }

        None
    }

    pub fn insert_at_the_end(&mut self, val: &T) {
        if let Some(next) = &mut self.next {
            (**next).insert_at_the_end(val);
        } else {
            let node = Box::new(Node {
                value: *val,
                next: None,
            });
            self.next = Some(node);
        }
    }

    pub fn insert_in_place(&mut self, val: T) {
        /*
         * All of this code is made as a learning resource.
         * Instead of all of this unsafe code you could just do: 'self.next.take()' (which 
         * internally uses the mem::replace which uses the same code provided here.)
         */
        let next_val = unsafe {
            /*
             * Performs a bit by bit copy(bitwise copy) of the self.next.
             * Since the self.next is just an Option with a box pointer it is not expansive.
             */
            let next_val = ptr::read(&self.next);

            /*
             * In this case we can't just do self.next = None.
             * Because when we do self.next = None we are placing None in the self.next and then we
             * are dropping the old value(the Box value). Instead we just want to place the None in
             * the self.next without triggering any type of Drop trait.
             */
            ptr::write(&mut self.next, None);

            // Returns the Option with the box pointer...
            next_val
        };

        let new_node = Box::new(Node {
            value: val,
            // next: self.next.take(), // Instead of using all of the code above we can just call self.next.take() and it will work just fine, but since this is a learning project there you go... Don't forget tho, if you use self.next.take() COMMENT the code above
            next: next_val,
        });
        self.next = Some(new_node);
    }
}

fn main() {
    let mut first = Node {
        value: 1,
        next: None,
    };

    first.insert_at_the_end(&2);
    first.insert_at_the_end(&3);
    first.insert_at_the_end(&4);

        /*
     * If we don't use the ManuallyDrop,
     * At the end of the program both the first and the copyoffirst will get freed by rust.
     * The problem is that both of them have a reference to the same values.
     * So one of them gets freed successfully but when rust tries to free the second one it will be
     * freeing a memory that has already been "given back" to the operational system. Therefore
     * resulting in a seg fault.
     *
     * In this case, we don't need to drop the copyoffirst because it's data is also owned by first
     * which will get freed up automagically by rust :). Resulting in no seg faults
     */
    let mut copyoffirst = ManuallyDrop::new(unsafe { ptr::read(&mut first) });

    first.insert_in_place(76600);

    copyoffirst.insert_in_place(99999);

    /*
     * As a prove that the insert_in_place is not copying the data in order to move it, when i
     * update the first[3] value, both the copyoffirst and the first values get updated lol. unsafe
     * rust is amazing.
     * Tho i am getting a seg fault for some reason.
     * [ADDED LATER] Just Fixed seg fault lmao, read all of the comments to understand
     */
    first.get_mut(3).unwrap().value = 888888;

    let g = vec![first.get(0), copyoffirst.get(0)];

    println!("{:#?}", g);

    // this will generate a seg fault because the data owned by copyoffirst is also owned by 'first'
    // then, it will get dropped and given back to the operational system.
    // The problem is that, when rust tries to clean the first value(at the end of the program),
    // the operational system will notify that it is not owned by rust anymore and therefore it
    // will result in a seg fault.
    // To fix the seg fault just comment the next line
    mem::drop(ManuallyDrop::into_inner(copyoffirst));
    // Or you can use the following:
    // mem::drop(&mut *copyoffirst);
}
