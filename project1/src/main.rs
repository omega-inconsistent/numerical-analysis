mod matrix;
mod functions;

use functions::*;

fn newton(mut p: Point, tol: f64, m: usize) -> Option<Point> {
    
    for _k in 0..m {
        println!("k: {_k}, x: {}, y: {} ", p.x, p.y); 
        // list the point p_k
        
        let q = p - J(p).inverse().unwrap() * F(p); 
        // definition of psi, where J is the Jacobian matrix of the system
        //                          F is the system itself

        if (p.x - q.x).abs().max((p.y - q.y).abs()) <= tol { 
            // whether | p - q |_infty < tol

            println!("k: {}, x: {}, y: {} ", _k+1, q.x, q.y);
             // list the point p_k+1

            return Some(q);  
            // if the condition holds, then return the final point
        }

        p = q;
    }

    None // Maximum number of iteration exceeded, 
         // return None to indicate the failure
}

fn main() {
    let p1 = Point { x: 0.25, y: 0.25 };
    let q1 = newton(p1, 10e-6, 1000).unwrap();
    println!("({}, {})", q1.x, q1.y);

    let p2 = Point { x: -0.1, y: 0.15 };
    let q2 = newton(p2, 10e-6, 1000).unwrap();
    println!("({}, {})", q2.x, q2.y);
}
