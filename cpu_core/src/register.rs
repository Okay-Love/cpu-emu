pub struct Register<T: Sized> {
    A: Option<T>,
    B: Option<T>,
}

impl<T: Sized> Register<T>{
    pub fn register_a(&self) -> &Option<T>{
        &self.A
    }

    pub fn register_b(&self) -> &Option<T>{
        &self.B
    }

    pub fn set_register_a(&mut self, value: Option<T>){
        self.A = value;
    }

    pub fn set_register_b(&mut self, value: Option<T>){
        self.B = value;
    }
}