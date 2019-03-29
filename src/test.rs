#[cfg(test)]
mod tests {

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::loader::load_dll;
    use crate::context::*;
    use crate::il::*;

    #[test]
    fn test_run() {
        let dll_path = "./assets/TestDll.dll";
        let dll = load_dll(dll_path);
        let rc_dll = Rc::new(RefCell::new(dll));

        let mut context = Context::new();
        context.reflection.load_dll(&rc_dll);

        let test_class = context.reflection.get_class_info(&"Class1").unwrap();
        let method_add = context.reflection.get_method_info(&"add",&test_class).unwrap();
        let ret = context.exec(&method_add,Some(vec![Data{i32:1574},Data{i32:-433}]));

        assert_eq!(ret.unwrap().to_i32(),1574 - 433);

    }
}