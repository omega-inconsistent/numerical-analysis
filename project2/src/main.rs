use function::{G, dG};

pub mod function;

fn steepest_descent(N : usize, TOL: f64, mut x: f64, mut y: f64, mut z: f64) -> (f64, f64, f64) {

    for k in 1..N {
        let g1 = G(x, y, z);
        let (mut z1, mut z2, mut z3) = dG(x, y, z);
        let norm_z = (z1*z1 + z2*z2 + z3*z3).sqrt();

        if norm_z == 0.0 {
            println!("Zero gradient");
            return (x, y, z); 
        }

        z1 /= norm_z;
        z2 /= norm_z;
        z3 /= norm_z;
        
        let a1 = 0.0;
        let mut a3 = 1.0;
        let g = |a: f64| {G(x - a * z1, y - a * z2, z - a * z3)};
        let mut g3 = g(a3);

        while g3 >= g1 {
            a3 /= 2.0;
            g3 = g(a3);

            if a3 < TOL / 2.0 {
                println!("No likely improvement");
                return (x, y, z);
            }
        }

        let a2 = a3 / 2.0;
        let g2 = g(a2);

        let h1 = (g2 - g1) / a2;
        let h2 = (g3 - g2) / (a3 - a2);
        let h3 = (h2 -h1)/ a3;

        let a0 = (a2 - h1) / h3 / 2.0;  // the critcal point the interpolation formula
        let g0 = g(a0);

        let (a, g) = if g0 >= g3 { (a3, g3) }  else { (a0, g0) };

        x -= a * z1;
        y -= a * z2;
        z -= a * z3;

        println!("k:{}, x:{x}, y:{y}, z:{z}, err:{}", k+1, (g - g1).abs());

        if (g - g1).abs() < TOL {
            println!("The procedure was successful");
            return (x, y, z);
        }
    }

    println!("Maximum iterations exceeded");
    (x, y, z)
}

fn main() {
    let (x, y, z) =  steepest_descent(10000, 10e-5, 0.0, 0.0, 0.0);
    println!("x: {x}, y: {y}, z: {z}, G: {}", G(x, y, z));
}
