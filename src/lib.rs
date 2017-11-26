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
    
    #[test]
    fn tuple() {
        let tuple = (1,2,"Hello", "John");
        // Destructuring
        let (a,_,c,_) = tuple;
        assert_eq!(1, a);
        assert_eq!("Hello", c);
        
        let _tuple = ((10, 20),("N","W"));
    }
    
    // https://rustbyexample.com/primitives/tuples.html
    #[derive(Debug)]
    struct Matrix(f32,f32,f32,f32);
    
    use std::fmt;
    
    impl fmt::Display for Matrix {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    
    #[test]
    fn printing_struct() {
        let matrix = Matrix(1.1,1.2,2.1,2.2);
        // {} kicks the fmt::Display formatting declared above
        assert_eq!(
            "( 1.1 1.2 )\n( 2.1 2.2 )",
            format!("{}", matrix)
        ); 
        // {:?} kicks the fmt::Debug default formatting
        assert_eq!(
            "Matrix(1.1, 1.2, 2.1, 2.2)",
            format!("{:?}", matrix)
        ); 
    }
    
    fn transpose(matrix: Matrix) -> Matrix {
        return Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    }
    
    #[test]
    fn matrix_transpose() {
        let matrix = Matrix(1.1,1.2,2.1,2.2);
        let transposed = transpose(matrix);
        assert_eq!("( 1.1 2.1 )\n( 1.2 2.2 )", format!("{}", transposed));
    }
    
    // https://rustbyexample.com/primitives/array.html
    #[test]
    fn array() {
        // Fixed-size array (type signature is superfluous)
        let _array: [i32; 5] = [1, 2, 3, 4, 5];
        
        // Initialization of value
        let ys: [i32; 500] = [17; 500];
        assert_eq!(ys[2], 17);
        assert_eq!(ys.len(), 500);
    }
    
    // TODO: https://rustbyexample.com/custom_types/structs.html
}
