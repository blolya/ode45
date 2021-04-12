pub struct Solver {
    f: Box<dyn Fn(f64, Vec<f64>)-> Vec<f64>>,
    y: Box<Vec<f64>>,
    t: f64,
    h: f64,
}

impl Solver {
    pub fn new(f: impl Fn(f64, Vec<f64>) -> Vec<f64> + 'static, t0: f64, y0: Vec<f64>, tk: f64) -> Solver {
        let mut s = Solver {
            f: Box::new(f),
            y: Box::new(y0),
            t: t0,
            h: 0.01,
        };

        while s.t < tk {
            s.step();
        };

        s
    }
    fn step(&mut self) {
        let m = |v: &Vec<f64>, b: f64| -> Vec<f64> {
            let mut r: Vec<f64> = vec![];
            
            for e in v {
                r.push( *e * b );
            };
            r
        };
        let a = |v1: &Vec<f64>, v2: &Vec<f64>,| -> Vec<f64> {
            let mut r: Vec<f64> = vec![];
            
            for (i, _) in v1.iter().enumerate() {
                r.push( v1[i] + v2[i] );
            };
            r
        };
        let c = |v: &Vec<f64>| -> Vec<f64> {
            let mut r: Vec<f64> = vec![];
            
            for e in v {
                r.push( *e );
            };
            r
        };

        let k1 = (self.f)( self.t, c(&self.y) );
        let k2 = (self.f)( self.t + self.h / 2.0, a(&self.y, &m(&k1, self.h / 2.0) ));
        let k3 = (self.f)( self.t + self.h / 2.0, a(&self.y, &m(&k2, self.h / 2.0) ));
        let k4 = (self.f)(self.t + self.h, a(&self.y, &m(&k3, self.h) ));

        let y = a( &self.y, &m(  &a(&k1, &a(&m(&k2, 2.0), &a(&m(&k3, 2.0), &k4)) ), self.h / 6.0 ) );
        self.y = Box::new(y);
        self.t += self.h;
    }
}