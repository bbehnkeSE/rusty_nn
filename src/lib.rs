use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operations {
    Add,
    Sub,
    Mul,
    Non
}


#[derive(Debug, PartialEq)]
struct Val {
    data: f64,
    grad: f64,
    res:  f64,
    prev: Vec<Val>,
    op:   Operations,
}


impl Val {
    fn new(d: f64) -> Val {
        return Val { data: d, grad: 0.0, res: 0.0, prev: Vec::new(), op: Operations::Non };
    }

    fn set_op(&mut self, op: Operations) {
        self.op = op;
    }
}


/*** Operator Overloads ***/

impl ops::Neg for Val {
    type Output = Val;
    fn neg(mut self) -> Val {
        self.data = -self.data;

        return self;
    }
}


impl ops::Add for Val {
    type Output = Val;
    fn add(mut self, rhs: Self) -> Val {
        let mut result: Val = Val::new(self.data + rhs.data);
        self.res = result.grad;
        result.prev.push(self);
        result.prev.push(rhs);
        result.set_op(Operations::Add);

        return result;
    }
}


impl ops::Sub for Val {
    type Output = Val;
    fn sub(self, rhs: Self) -> Val {
        let mut result: Val = Val::new(self.data - rhs.data);
        result.prev.push(self);
        result.prev.push(rhs);
        result.set_op(Operations::Sub);

        return result;
    }
}


impl ops::Mul for Val {
    type Output = Val;
    fn mul(mut self, rhs: Self) -> Val {
        let mut result: Val = Val::new(self.data * rhs.data);
        self.res = result.grad * rhs.data;
        result.prev.push(self);
        result.prev.push(rhs);
        result.set_op(Operations::Mul);

        return result;
    }
}
/*** End Overloads ***/


/*** Displays ***/

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operations::Add => write!(f, "+"),
            Operations::Sub => write!(f, "-"),
            Operations::Mul => write!(f, "*"),
            Operations::Non => write!(f, "Non")
        }
    }
}


impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Data: {}, Grad: {}, Prev: {:?}, Op: {}", self.data, self.grad, self.prev, self.op);
    }
}
/*** End Displays ***/



#[cfg(test)]
mod val_ops {
    use super::*;
    // Helper function for floating point arithmetic
    fn approx_eq(a: f64, b: f64) -> bool {
        return (a - b).abs() < 1e-12;
    }

    #[test]
    fn val() {
        let v: Val = Val::new(3.9);
        assert_eq!(v.data, 3.9);
        assert_eq!(v.grad, 0.0);
        assert_eq!(v.prev.len(), 0);
        assert_eq!(v.op, Operations::Non);
    }

    #[test]
    fn neg() {
        {
            let v1: Val = Val::new(10.0);
            let result: Val = -v1;

            assert_eq!(result.data, -10.0);
        }

        {
            let v1: Val = Val::new(-30.3);
            let result: Val = -v1;

            assert_eq!(result.data, 30.3);
        }

        {
            let v1: Val = Val::new(-30.3);
            let v2: Val = Val::new(20.3);
            let result: Val = -v1 + v2;

            assert_eq!(result.data, 50.6);
        }

        {
            let v1: Val = Val::new(-2.4567);
            let v2: Val = Val::new(-263.413276);
            let result: Val = v2 - -v1;

            assert_eq!(result.data, -265.869976);
        }
    }

    #[test]
    fn add() {
        {
            let v1: Val = Val::new(2.0);
            let v2: Val = Val::new(4.5);
            let result: Val = v1 + v2;

            assert_eq!(result.data, 6.5);
            assert_eq!(result.prev[0].data, 2.0);
            assert_eq!(result.prev[1].data, 4.5);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(2.0);
            let v2: Val = Val::new(4.5);
            let result: Val = v2 + v1;

            assert_eq!(result.data, 6.5);
            assert_eq!(result.prev[0].data, 4.5);
            assert_eq!(result.prev[1].data, 2.0);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(-5.1);
            let v2: Val = Val::new(2.3);
            let result: Val = v1 + v2;

            assert_eq!(result.data, -2.8);
            assert_eq!(result.prev[0].data, -5.1);
            assert_eq!(result.prev[1].data, 2.3);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(-5.1);
            let v2: Val = Val::new(2.3);
            let result: Val = v2 + v1;

            assert_eq!(result.data, -2.8);
            assert_eq!(result.prev[0].data, 2.3);
            assert_eq!(result.prev[1].data, -5.1);
            assert_eq!(result.op, Operations::Add);
        }
        
        {
            let v1: Val = Val::new(0.0);
            let v2: Val = Val::new(2.3);
            let result: Val = v2 + v1;

            assert_eq!(result.data, 2.3);
            assert_eq!(result.prev[0].data, 2.3);
            assert_eq!(result.prev[1].data, 0.0);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(-5.1);
            let v2: Val = Val::new(0.0);
            let result: Val = v2 + v1;

            assert_eq!(result.data, -5.1);
            assert_eq!(result.prev[0].data, 0.0);
            assert_eq!(result.prev[1].data, -5.1);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(-5.1000000006);
            let v2: Val = Val::new(82.999999993);
            let result: Val = v2 + v1;

            assert_eq!(result.data, 77.8999999924);
            assert_eq!(result.prev[0].data, 82.999999993);
            assert_eq!(result.prev[1].data, -5.1000000006);
            assert_eq!(result.op, Operations::Add);
        }
    }

    #[test]
    fn sub() {
        {
            let v1: Val = Val::new(100.1);
            let v2: Val = Val::new(100.1);
            let result: Val = v1 - v2;

            assert_eq!(result.data, 0.0);
            assert_eq!(result.prev[0].data, 100.1);
            assert_eq!(result.prev[1].data, 100.1);
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(8.9);
            let v2: Val = Val::new(2.3);
            let result: Val = v1 - v2;

            assert!(approx_eq(result.data, 6.6));
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(8.9);
            let v2: Val = Val::new(2.3);
            let result: Val = v2 - v1;

            assert!(approx_eq(result.data, -6.6));
            assert_eq!(result.prev[0].data, 2.3);
            assert_eq!(result.prev[1].data, 8.9);
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(289.37);
            let v2: Val = Val::new(-367.11);
            let result: Val = v1 - v2;

            assert!(approx_eq(result.data, 656.48));
            assert_eq!(result.prev[0].data, 289.37);
            assert_eq!(result.prev[1].data, -367.11);
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(289.37);
            let v2: Val = Val::new(0.0);
            let result: Val = v1 - v2;

            assert!(approx_eq(result.data, 289.37));
            assert_eq!(result.prev[0].data, 289.37);
            assert_eq!(result.prev[1].data, 0.0);
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(0.0);
            let v2: Val = Val::new(-367.11);
            let result: Val = v1 - v2;

            assert!(approx_eq(result.data, 367.11));
            assert_eq!(result.prev[0].data, 0.0);
            assert_eq!(result.prev[1].data, -367.11);
            assert_eq!(result.op, Operations::Sub);
        }

        {
            let v1: Val = Val::new(472.123456789);
            let v2: Val = Val::new(0.0987654321);
            let result: Val = v1 - v2;

            assert!(approx_eq(result.data, 472.0246913569));
            assert_eq!(result.prev[0].data, 472.123456789);
            assert_eq!(result.prev[1].data, 0.0987654321);
            assert_eq!(result.op, Operations::Sub);
        }
    }

    #[test]
    fn mul() {
        {
            let v1: Val = Val::new(16.2);
            let v2: Val = Val::new(2.0);
            let result: Val = v1 * v2;

            assert_eq!(result.data, 32.4);
            assert_eq!(result.prev[0].data, 16.2);
            assert_eq!(result.prev[1].data, 2.0);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(16.2);
            let v2: Val = Val::new(2.0);
            let result: Val = v2 * v1;

            assert_eq!(result.data, 32.4);
            assert_eq!(result.prev[0].data, 2.0);
            assert_eq!(result.prev[1].data, 16.2);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(16.2);
            let v2: Val = Val::new(0.0);
            let result: Val = v2 * v1;

            assert_eq!(result.data, 0.0);
            assert_eq!(result.prev[0].data, 0.0);
            assert_eq!(result.prev[1].data, 16.2);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(16.2);
            let v2: Val = Val::new(0.0);
            let result: Val = v1 * v2;

            assert_eq!(result.data, 0.0);
            assert_eq!(result.prev[0].data, 16.2);
            assert_eq!(result.prev[1].data, 0.0);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(739.123456789);
            let v2: Val = Val::new(99.0987654321);
            let result: Val = v1 * v2;

            assert!(approx_eq(result.data, 73_246.222069696));
            assert_eq!(result.prev[0].data, 739.123456789);
            assert_eq!(result.prev[1].data, 99.0987654321);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(739.123456789);
            let v2: Val = Val::new(99.0987654321);
            let result: Val = v2 * v1;

            assert!(approx_eq(result.data, 73_246.222069696));
            assert_eq!(result.prev[0].data, 99.0987654321);
            assert_eq!(result.prev[1].data, 739.123456789);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(-739.123456789);
            let v2: Val = Val::new(99.0987654321);
            let result: Val = v1 * v2;

            assert!(approx_eq(result.data, -73_246.222069696));
            assert_eq!(result.prev[0].data, -739.123456789);
            assert_eq!(result.prev[1].data, 99.0987654321);
            assert_eq!(result.op, Operations::Mul);
        }

        {
            let v1: Val = Val::new(739.123456789);
            let v2: Val = Val::new(-99.0987654321);
            let result: Val = v1 * v2;

            assert!(approx_eq(result.data, -73_246.222069696));
            assert_eq!(result.prev[0].data, 739.123456789);
            assert_eq!(result.prev[1].data, -99.0987654321);
            assert_eq!(result.op, Operations::Mul);
        }
    }

    #[test]
    fn com() {
        {
            let v1: Val = Val::new(40.0034);
            let v2: Val = Val::new(11.9253);
            let v3: Val = Val::new(-526.9637);
            let result: Val = v1 * v2 + v3;

            assert!(approx_eq(result.data, -49.91115398));
            assert_eq!(result.prev[0].op, Operations::Mul);
            assert_eq!(result.prev[0].data, 40.0034 * 11.9253);
            assert_eq!(result.prev[1].data, -526.9637);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(40.0034);
            let v2: Val = Val::new(11.9253);
            let v3: Val = Val::new(-526.9637);
            let result: Val = v2 * v1 + v3;

            assert!(approx_eq(result.data, -49.91115398));
            assert_eq!(result.prev[0].op, Operations::Mul);
            assert_eq!(result.prev[0].data, 40.0034 * 11.9253);
            assert_eq!(result.prev[1].data, -526.9637);
            assert_eq!(result.op, Operations::Add);
        }

        {
            let v1: Val = Val::new(40.0034);
            let v2: Val = Val::new(11.9253);
            let v3: Val = Val::new(-526.9637);
            let result: Val = v3 * v2 + v1;

            assert!(approx_eq(result.data, -6244.19681161));
            assert_eq!(result.prev[0].op, Operations::Mul);
            assert_eq!(result.prev[0].data, -526.9637 * 11.9253);
            assert_eq!(result.prev[1].data, 40.0034);
            assert_eq!(result.op, Operations::Add);
        }
    }
}

#[cfg(test)]
mod prt {
    use super::*;
    #[test]
    #[ignore]
    fn prt() {
        let v1: Val = Val::new(8.0);
        println!("{}", v1);
        let v2: Val = Val::new(2.2123);
        let v3: Val = Val::new(-2.2);
        let mut result: Val = v1 * v2 + v3;
        result.grad = 1.0;

        println!("Result: {}", result);
    }
}