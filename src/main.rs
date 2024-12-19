use std::{
    io::{stdout, Write}, mem::{self, ManuallyDrop}, ptr, thread::sleep_ms
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

    pub fn insert_before(&mut self, val: &T) {
        let current = mem::replace(self, Node {
            value: *val,
            next: None,
        });
        self.next = Some(Box::new(current));
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
    /*let mut first = ManuallyDrop::new(Node {
        value: 1usize,
        next: None,
    });
    */
    let mut first = ManuallyDrop::new(Node {
        value: [2usize; 10_000_000],
        next: None,
    });

    println!("Got");

    /*
     first.insert_at_the_end(&2);
    first.insert_at_the_end(&3);
    first.insert_at_the_end(&4); */

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
    let copyoffirst = ManuallyDrop::new(unsafe { ptr::read(&mut first) });
    
    // This should not go to copyoffirst
    /* first.insert_in_place(222222);
    first.insert_before(&0); */

    // This proves that the 'inser_before' is not copying the data. Because both the copyoffirst
    // and the first get updated
/*     first.get_mut(3).unwrap().value = 333333; */

    /*
     * This proves that the insert_in_place is not copying the data in order to move it, when i
     * update the first[3] value, both the copyoffirst and the first values get updated lol. unsafe
     * rust is amazing.
     * Tho i am getting a seg fault for some reason.
     * [ADDED LATER] Just Fixed seg fault lmao, read all of the comments to understand
     */
/*     first.get_mut(4).unwrap().value = 44444; */

    for i in 5..100_000 {
        first.get_mut(i).unwrap().insert_in_place([i; 10_000_000]);
    }
    println!("Done creating");

    /*
     * with this commented code it is possible to create an infinite linked list because the fourth
     * element points to the data that is in the first element (which eventually points to the
     * fourth element). This happens because even tho we are copying the first data, the actual
     * nodes are stored in the heap and then when we tell rust to write to the fourth element to
     * point to the first it is actually pointing at the previous data
     * Even if you don't print it or do anything with it eventually it may cause a stack overflow
     * because eventually rust is going to try to drop and it seems like there is some type of
     * recursion inside the drop implementation(i could not find the actualy code becase it seems
     * like it is done under the hood by the rust compiler. Even the function ptr::drop_in_place
     * gets replaced by the compiler code.)
     * To fix this issue, you must use ManuallyDrop in the first, This way, you will never end up
     * calling the Drop trait...
     *
     * Now apparently it is impossible to ask rust to free the actual heap linked list.
     * Even if you call ManuallyDrop::drop(), it will run ptr::drop_in_place with the value of the
     * list which has that problem of having the infinite linked list.
     *
     * But, you can just memory leak and pretend like that memory never existed(but it will still
     * be there until you close the program). To do that you must call:
     * ptr::drop_in_place(&mut first). Notice the difference.
     * It is not the same as: ptr::drop_in_place(&mut *first); because the second one tries to drop
     * the value of first. and the first one just drops the ManuallyDrop struct wrapper.
     * [ADDED_LATER] It is funny because for some reason, calling manuallyDrop::drop or ptr::drop_in_place(&mut *first) doubles the memory used by the program even when there is no infinite linked list
     *
     * And by the way, if there is any nodes after the node index in get_mut, it will end up
     * getting memory leaked because those nodes will be lost for ever but the memory won't get
     * freed
     */
    /* unsafe {
        ptr::write(&mut first.get_mut(49_000).unwrap().next, Some(Box::new(ptr::read(&mut *first))))
    } */

    // let g = vec![first.get(0), copyoffirst.get(0)];

    // println!("{:?}", g);
    // println!("{:?}", first);
    println!("Created nasted loop and");

    // It is funny because for some reason, calling manuallyDrop::drop or ptr::drop_in_place(&mut *first) doubles the memory used by the program even when there is no infinite linked list
    /* unsafe {
        ManuallyDrop::drop(&mut first);
    } */

    println!("Finished creating memory leak");

    let mut i: usize = 0;
    while i < 300{
        print!("\rhey there {:10}   \r", &i);
        i += 1;
        stdout().flush().unwrap();
        sleep_ms(50);
    }

    first.get_mut(100).unwrap().value = [3; 10_000_000];
    println!("{:?}", first.get_mut(100).unwrap().value);

    // this will generate a seg fault because the data owned by copyoffirst is also owned by 'first'
    // then, it will get dropped and given back to the operational system.
    // The problem is that, when rust tries to clean the first value(at the end of the program),
    // the operational system will notify that it is not owned by rust anymore and therefore it
    // will result in a seg fault.
    // To fix the seg fault just comment the next line
    // mem::drop(ManuallyDrop::into_inner(copyoffirst));
    // Or you can use the following:
    // mem::drop(&mut *copyoffirst);
}
