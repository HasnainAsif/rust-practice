// adding pub to expose struct to other modules, by default its private
pub struct RandomInfo {
    pub some_bool: bool, // adding pub to expose variable to other modules, by default its private
    pub some_int: i64,   // adding pub to expose variable to other modules, by default its private
}

// instead of adding method above, we will use following code
impl RandomInfo {
    pub fn new(param_a: bool) -> Self {
        // Self is equal to RandomInfo because we are inside RandomInfo
        Self {
            some_bool: !param_a,
            some_int: 8,
        }
    }

    pub fn is_smaller(&self, compare_to: i64) -> bool {
        // lowercase self means internal data
        self.some_int < compare_to // will return true or false i.e; boolean
    }
}
