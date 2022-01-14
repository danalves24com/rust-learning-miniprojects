enum RiemannSumTypes {
    Right,
    Left,
    Midpoint,
    Trapezoidal
}

fn f(x : f32) -> f32 {
    5.0 * x * x
}

fn fPrime(x : f32) -> f32 {
    10.0 * x
}


struct RiemannCalculator {
    b : f32,
    a : f32,
    n : u16
}

impl RiemannCalculator {
    fn calculate_right_sum(&self) -> f32 {
        let mut sum : f32 = 0.0;
        let dx = (&self.b - &self.a) / &self.b;
        for i in 0..(*&self.n as i16) {
            let comp = fPrime(&self.a + &dx*((1+i) as f32)) * &dx;
            sum = sum + comp;
        }
        sum
    }
}

fn calculuate_riemann_sum(
    sumtype : &RiemannSumTypes,
    b       : &f32,
    a       : &f32,
    n       : &u16
) -> f32
{
    let mut calculator : RiemannCalculator = RiemannCalculator {
        b:*b,
        a:*a,
        n:*n
    };
    calculator.calculate_right_sum()
}

fn main() {

    let a : f32 = -2.0;
    let b : f32 = 2.0;
    let n : u16 = 5;
    let sum_type = RiemannSumTypes::Left;
    let area = calculuate_riemann_sum(&sum_type, &b, &a, &n);
    println!("Area under curve is: {}", &area);

}
