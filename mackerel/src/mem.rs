/*
TODO try variable is stored on the stack or on the heam
enum MemoryLocation {
    Static,
    Stack,
    Heap,
}
macro_rules! is_copy {
    () => {};
}
*/

// TODO is needed? can probaby use "ty" type fragment?
fn size_of_value_helper<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

#[macro_export]
macro_rules! size_of_value {
    ($x:expr) => {
        size_of_value_helper(&$x)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_correct_size_of_values() {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let v_i32 = 32_i32;
        let v_u8 = 1_u8;
        let v_string = String::from("some data in the heap, but also some in the stack");
        let v_array = [0, 1, 2, 3, 4];
        let p = Point { x: 0, y: 0, z: 0 };
        assert_eq!(4, size_of_value!(v_i32));
        assert_eq!(1, size_of_value!(v_u8));
        assert_eq!(24, size_of_value!(v_string)); // TODO investigate on stack size of a string
        assert_eq!(5 * 4, size_of_value!(v_array));
        assert_eq!(3 * 4, size_of_value!(p));
    }
}
