use crate::register::Register;
use std::ops::Add;

pub struct CPU<T: Sized + Add>{
    register: Register<T>,
}

impl<T: Sized + Add> CPU<T>
{
    pub fn new() -> Self{
        CPU{
            register: Register::<T>::new(None, None)
        }
    }

    pub fn add(register_1: &T, register_2: &T) -> T{
        *register_1 + *register_2
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cpu_new(){

    }
}