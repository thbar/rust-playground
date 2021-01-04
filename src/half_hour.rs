// https://fasterthanli.me/articles/a-half-hour-to-learn-rust
#[allow(dead_code)]
mod half_hour {

    fn foobar(z: i32) -> i32 {
        z
    }

    #[test]
    fn assignments() {
        let _x;

        _x = 42;

        let _x = 42;

        let _x: i32;
        _x = 42;

        let _x: i32 = 42;

        // let _x;
        // foobar(_x);
    }

    #[test]
    fn tuples() {
        let tuple = (14, "test");

        // destructuring
        let (i, s) = tuple;

        assert_eq!(14, i);
        assert_eq!("test", s);

        // index access
        assert_eq!(14, tuple.0);
    }

    #[test]
    fn vec() {
        let x = vec![1, 2, 3];

        let mut result = x.iter().map(|x| x * 10);

        assert_eq!(Some(10), result.next());
        assert_eq!(20 + 30, result.fold(0, |x, y| x + y));
    }

    #[test]
    fn blocks() {
        let x = 42;
        let y = { 42 };
        assert_eq!(x, y);

        let z = {
            let x = 30;
            let y = 12;
            x + y
        };

        assert_eq!(y, z);
    }

    #[test]
    fn control() {
        let x = if true { 6 } else { 4 };
        assert_eq!(6, x);

        let cond = false;
        let y = match cond {
            true => 100,
            false => 10,
        };

        assert_eq!(10, y);
    }

    #[test]
    fn calls() {
        assert_eq!(3, std::cmp::min(3, 8));

        // bring in scope
        use std::cmp::min;
        assert_eq!(3, min(3, 8));

        // default stuff is imported by default
        let mut v = Vec::new();
        v.push(1);

        // but this is equivalent to this:
        let mut w = std::vec::Vec::new();
        w.push(1);
    }

    #[test]
    fn structs() {
        struct Vec2 {
            x: f64,
            y: f64,
        };

        let v1 = Vec2 { x: 1.0, y: 5.0 };
        let v2 = Vec2 { x: 3.0, ..v1 };

        assert_eq!(v1.x, 1.0);
        assert_eq!(v2.y, 5.0);
    }
}
