use std::collections::HashMap;

pub const PRIMITIVE_TYPES_COUNT: u32 = 3;
pub const I32_TYPE_ID: u32 = 0;
pub const F32_TYPE_ID: u32 = 1;
pub const BOOL_TYPE_ID: u32 = 2;
pub const STRING_TYPE_ID: u32 = 4;
pub const LIST_TYPE_ID: u32 = 5;

pub struct TypeManager {
    slots: HashMap<String, u32>,
    types: Vec<Type>,
}

impl TypeManager {
    pub fn empty() -> Self {
        Self {
            slots: HashMap::new(),
            types: Vec::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, ty: Type) {
        let slot = self.types.len() as u32;
        self.slots.insert(name.into(), slot);
        self.types.push(ty);
    }

    pub fn builtin() -> Self {
        let mut m = Self::empty();

        m.add("Void", Type::new(0));
        m.add("I32", Type::new(0));
        m.add("F32", Type::new(0));
        m.add("Bool", Type::new(0));
        m.add("String", Type::new(0));
        m.add("List", Type::new(1));

        m
    }

    pub fn get_id(&self, name: &str) -> Option<u32> {
        self.slots.get(name).copied()
    }
}

pub struct Type {
    generic_count: u32,
    methods: HashMap<String, u32>,
}

impl Type {
    pub fn new(generic_count: u32) -> Self {
        Self {
            generic_count,
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeSpec {
    pub id: u32,
    pub generics: Vec<TypeSpec>,
}

impl TypeSpec {
    pub fn new(id: u32, generics: Vec<TypeSpec>) -> Self {
        Self { id, generics }
    }

    pub fn void() -> Self {
        Self::new(0, vec![])
    }

    pub fn i32() -> Self {
        Self::new(I32_TYPE_ID, vec![])
    }

    pub fn f32() -> Self {
        Self::new(F32_TYPE_ID, vec![])
    }

    pub fn bool() -> Self {
        Self::new(BOOL_TYPE_ID, vec![])
    }

    pub fn string() -> Self {
        Self::new(STRING_TYPE_ID, vec![])
    }

    pub fn unknown_list() -> Self {
        Self::new(LIST_TYPE_ID, vec![Self::new(u32::MAX, vec![])])
    }

    pub fn list_of(item_type: TypeSpec) -> Self {
        Self::new(LIST_TYPE_ID, vec![item_type])
    }

    pub fn format(&self, tm: &TypeManager) -> String {
        let type_name = tm
            .slots
            .iter()
            .find_map(|(name, &id)| if id == self.id { Some(name) } else { None })
            .unwrap_or(&"{unknown}".to_string())
            .clone();

        if self.generics.is_empty() {
            type_name
        } else {
            let generics_str: Vec<String> = self.generics.iter().map(|g| g.format(tm)).collect();
            format!("{}<{}>", type_name, generics_str.join(", "))
        }
    }

    pub fn is_bool(&self) -> bool {
        self.id == BOOL_TYPE_ID && self.generics.is_empty()
    }

    pub fn get_index_type(&self) -> Option<TypeSpec> {
        match self.id {
            LIST_TYPE_ID => self.generics.get(0).cloned(),
            _ => None,
        }
    }
}
