use std::any;

fn type_of_helper<T>(_: &T) -> &str {
    any::type_name::<T>()
}

#[macro_export]
macro_rules! type_of {
    ($x:expr) => {
        type_of_helper(&$x)
    };
}

#[macro_export]
macro_rules! var_name {
    ($x:expr) => {
        stringify!($x)
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_get_correct_variable_name() {
        enum Speciality {
            Cryptography,
            System,
            Network,
            Security,
            Blockchain,
        }

        let alice = String::from("alice");
        let age = 42_u32;
        let speciality = Speciality::Cryptography;
        assert_eq!("alice", var_name!(alice));
        assert_eq!("age", var_name!(age));
        assert_eq!("speciality", var_name!(speciality));
    }

    #[test]
    fn should_get_correct_type() {
        enum Speciality {
            Cryptography,
            System,
            Network,
            Security,
            Blockchain,
        }
        let alice = String::from("alice");
        let age = 42_u32;
        let speciality = Speciality::Cryptography;
        // type of value
        assert_eq!("alloc::string::String", type_of!(alice));
        assert_eq!("u32", type_of!(age));
        assert_eq!(
            "mackerel::introspection::tests::should_get_correct_type::Speciality",
            type_of!(speciality)
        );
    }
}
