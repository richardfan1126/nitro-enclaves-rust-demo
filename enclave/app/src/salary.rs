use indexmap::IndexMap;
use uuid::Uuid;

pub struct Salary {
    pub salary_map: IndexMap<String, u32>
}

impl Salary {
    /// Constructor
    pub fn new() -> Salary {
        Salary {
            salary_map: IndexMap::new()
        }
    }

    pub fn add(&mut self, salary: u32) -> String {
        let id = Uuid::new_v4().to_string();
        self.salary_map.insert(id.clone(), salary);
        self.sort_salary_map();

        return id;
    }

    fn sort_salary_map(&mut self) {
        self.salary_map.sort_by(|_a_key, a_value, _b_key, b_value| b_value.cmp(a_value));
    }

    pub fn get_position(&self, id: String) -> Option<u32> {
        match self.salary_map.get_index_of(&id) {
            Some(index) => Some((index as u32) + 1),
            None => None
        }
    }
}
