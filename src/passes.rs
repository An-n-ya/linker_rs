use std::collections::LinkedList;

use crate::context::Context;

impl Context {
    pub fn resolve_symbol(&mut self) {
        for obj in self.object_iter() {
            let mut obj = obj.lock().unwrap();
            obj.resolve_symbol();
        }
        println!("before num obj: {}", self.obj_size());
        self.mark_live_objects();
        self.reclaim_objects();
        println!("after num obj: {}", self.obj_size());
    }

    pub fn mark_live_objects(&self) {
        let mut list = LinkedList::new();
        for obj in self.object_iter() {
            let obj_guard = obj.lock().unwrap();
            if obj_guard.is_alive {
                list.push_back(obj.clone());
            }
        }

        assert!(list.len() > 0);

        while list.len() > 0 {
            let obj = list.pop_front().unwrap();
            let obj = obj.lock().unwrap();
            obj.mark_live_objects(self, |elf| list.push_back(elf));
        }
    }
}
