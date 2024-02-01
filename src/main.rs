struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let p = 23; // Prime number for the field
    let a = 1;
    let b = 1;
    let base_point = Point { x: 3, y: 10 }; // Base point on the curve
    let n = 7; // A small number for scalar multiplication (for demonstration)

    let public_key = scalar_multiplication(&base_point, n, a, b, p);
    println!("Public Key: ({}, {})", public_key.x, public_key.y);
}

fn add_points(p1: &Point, p2: &Point, a: i64, modulus: i64) -> Point {
    if p1.x == p2.x && p1.y == p2.y {
        return double_point(p1, a, modulus);
    }
    let m = (p2.y - p1.y) * mod_inverse(p2.x - p1.x, modulus) % modulus;
    let x3 = (m.pow(2) - p1.x - p2.x) % modulus;
    let y3 = (m * (p1.x - x3) - p1.y) % modulus;
    Point { x: (x3 + modulus) % modulus, y: (y3 + modulus) % modulus }
}

fn double_point(point: &Point, a: i64, modulus: i64) -> Point {
    let m = (3 * point.x.pow(2) + a) * mod_inverse(2 * point.y, modulus) % modulus;
    let x3 = (m.pow(2) - 2 * point.x) % modulus;
    let y3 = (m * (point.x - x3) - point.y) % modulus;
    Point { x: (x3 + modulus) % modulus, y: (y3 + modulus) % modulus }
}

fn scalar_multiplication(point: &Point, scalar: i64, a: i64, b: i64, modulus: i64) -> Point {
    let mut result = Point { x: 0, y: 0 }; // Identity element in additive group
    let temp_point = point.clone();
    for _ in 0..scalar {
        result = add_points(&result, &temp_point, a, modulus);
    }
    result
}

fn mod_inverse(value: i64, modulus: i64) -> i64 {
    // Extended Euclidean Algorithm to find modular inverse
    let mut m0 = modulus;
    let mut y = 0;
    let mut x = 1;

    if modulus == 1 {
        return 0;
    }

    let mut a = value % modulus;
    while a > 1 {
        let q = a / m0;
        let mut t = m0;

        // m0 is remainder now, process same as Euclid's algo
        m0 = a % m0;
        a = t;
        t = y;

        // Update y and x
        y = x - q * y;
        x = t;
    }

    // Make x positive
    if x < 0 {
        x += modulus;
    }

    x
}
