#[cfg(test)]
mod tests {
    #[test]
    // see https://doc.rust-lang.org/std/fmt/
    fn formatted_print() {
        // Regular arguments
        let s = format!("Hello {}.", "John");
        assert_eq!(s, "Hello John.");
        
        // Positional arguments
        let s = format!("Hello {0}. You are {1} years old.", "John", 39);
        assert_eq!(s, "Hello John. You are 39 years old.");
        
        // Named arguments
        let s = format!("Hello {first_name}.", first_name = "John");
        assert_eq!(s, "Hello John.");
        
        // Formatting in hex
        assert_eq!(
            "10 in hex is 0xA",
            format!("10 in hex is 0x{:X}", 10)
        );
        
        // Formatting float
        assert_eq!(
            "The number is 12.889",
            format!("The number is {:.*}", 3, 12.8888)
        );
        
        // Formatting zero-padding
        assert_eq!(
            "We have 0001",
            format!("We have {number:>0width$}", number=1, width=4)
        )
    }
    
    #[test]
    fn formatter_print_for_struct() {
        #[derive(Debug)]
        struct MyStruct {
            x: i32,
            y: i32
        }
        
        let test = MyStruct { x: 10, y: -50 };
        
        assert_eq!(
            "MyStruct { x: 10, y: -50 }",
            format!("{:?}", test)
        );
    }
    
    #[test]
    fn primitives() {
        let _choice: bool = true;
        let _integer = 100i32;
        let _float = 1.0f64;
        // we're shadowing the previous variable declaration here
        let _float : f64 = 1.0;
        
        // mutable so can be re-assigned (but with the same type only)
        let mut _float = 1.0; // f64
        _float = 67.0;
        
        let _char = 'é';
        let _array = [1, 2, 3, 4];
        let _tuple = (1, 2, 3, 4);
        let _string = "John";
    }
}
