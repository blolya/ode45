pub struct Solver<F> 
    where F: Fn(f64, &Vec<f64>) -> Vec<f64>
{
    f: F,
    y: Vec<f64>,
    t: f64,
    tk: f64,
    h: f64,
}

impl<F> Solver<F>
    where F: Fn(f64, &Vec<f64>) -> Vec<f64>
{
    pub fn new(f: F, t0: f64, y0: Vec<f64>, tk: f64) -> Solver<F> {
        Solver {
            f,
            y: y0,
            t: t0,
            tk,
            h: 0.01,
        }
    }
    pub fn solve(mut self) -> Vec<f64> {
        while self.t < self.tk {
            self.step();
        };
        self.y
    }
    fn step(&mut self) {

        let m = |v: &Vec<f64>, b: f64| -> Vec<f64> {
            let mut r: Vec<f64> = vec![];
            
            for e in v {
                r.push( *e * b );
            };
            r
        };
        let a = |v1: &Vec<f64>, v2: &Vec<f64>| -> Vec<f64> {
            let mut r: Vec<f64> = vec![];
            
            for (i, _) in v1.iter().enumerate() {
                r.push( v1[i] + v2[i] );
            };
            r
        };

        let k1 = (self.f)( self.t, &self.y );
        let k2 = (self.f)( self.t + self.h / 2.0, &a( &self.y, &m(&k1, self.h / 2.0) ) );
        let k3 = (self.f)( self.t + self.h / 2.0, &a( &self.y, &m(&k2, self.h / 2.0) ) );
        let k4 = (self.f)( self.t + self.h, &a( &self.y, &m(&k3, self.h) ) );

        let y1 = m( &k1, self.h / 6.0);
        let y2 = m( &k2, self.h / 3.0);
        let y3 = m( &k3, self.h / 3.0);
        let y4 = m( &k4, self.h / 6.0);

        let y = a( &self.y, &a( &y1, &a( &y2, &a(&y3, &y4) ) )  );
        self.y = y;
        self.t += self.h;
    }
}