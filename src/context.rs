use std::{
    collections::{hash_map::Values, HashMap},
    rc::Rc,
    sync::Mutex,
};

use crate::{
    symbol::{ShareSymbol, Symbol},
    utils::input_elf::InputElf,
};

pub struct Context {
    objects: HashMap<usize, Rc<Mutex<InputElf>>>,
    symbol_map: HashMap<String, ShareSymbol>,
    obj_id: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            objects: HashMap::default(),
            symbol_map: HashMap::default(),
            obj_id: 1,
        }
    }
    pub fn push(&mut self, mut object: InputElf) {
        object.id = self.obj_id;
        object.initialize_symbol(self);
        self.objects
            .insert(self.obj_id, Rc::new(Mutex::new(object)));
        self.obj_id += 1;
    }
    pub fn obj_size(&self) -> usize {
        self.objects.len()
    }
    pub fn object_iter(&self) -> Values<'_, usize, Rc<Mutex<InputElf>>> {
        self.objects.values()
    }
    pub fn get_object(&self, id: usize) -> Option<Rc<Mutex<InputElf>>> {
        self.objects.get(&id).map(|n| n.clone())
    }
    pub fn reclaim_objects(&mut self) {
        // clear objects
        let arr = self
            .objects
            .iter()
            .filter(|(_, obj)| {
                let obj = obj.lock().unwrap();
                let filtered = !obj.is_alive;
                if filtered {
                    obj.clear_symbol();
                }
                filtered
            })
            .map(|(i, _)| *i)
            .collect::<Vec<_>>();
        for ind in arr {
            self.objects.remove(&ind);
        }
        // clear symbol
        let arr = self
            .symbol_map
            .iter()
            .filter(|(_, symbol)| {
                let sym = symbol.lock().unwrap();
                sym.is_alive
            })
            .map(|(s, _)| s.clone())
            .collect::<Vec<_>>();
        for s in arr {
            self.symbol_map.remove(&s);
        }
    }
    pub fn find_symbol_by_name(&mut self, name: String) -> ShareSymbol {
        if self.symbol_map.contains_key(&name) {
            self.symbol_map[&name].clone()
        } else {
            let sym = Rc::new(Mutex::new(Symbol::new(name.clone(), 0xffff, 0)));
            self.symbol_map.insert(name, sym.clone());
            sym
        }
    }
}
