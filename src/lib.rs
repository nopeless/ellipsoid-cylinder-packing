use std::{f32::consts::PI, fs::OpenOptions};
use stl_io::{Normal, Vertex};

/// STL functions

pub fn make_cylinder(ox: f32, oy: f32, oz: f32, r: f32, d: f32) -> Vec<stl_io::Triangle> {
    let mut triangles = vec![];

    let count = 16;
    let radius = r;
    let ds = d.signum();
    let angle_step = 2.0 * std::f32::consts::PI / count as f32;
    for z in [oz, oz + d] {
        // This line calculates normals in a non-efficient way
        let zn = (z - oz - d / 2.0).signum();
        let normal = Normal::new([0.0, 0.0, zn]);
        for i in 0..count {
            let angle = angle_step * i as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();

            let mut vertices = [
                Vertex::new([ox + x, oy + y, z]),
                Vertex::new([
                    ox + radius * (angle + angle_step).cos(),
                    oy + radius * (angle + angle_step).sin(),
                    z,
                ]),
                Vertex::new([ox, oy, z]),
            ];

            if zn < 0.0 {
                vertices.swap(1, 2);
            }

            triangles.push(stl_io::Triangle { normal, vertices });
        }
    }

    // Make the walls
    for i in 0..count {
        let angle = angle_step * i as f32;
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let normal = Normal::new([angle.sin() * ds, angle.cos() * ds, 0.0]);

        let mut vertices = [
            Vertex::new([ox + x, oy + y, oz]),
            Vertex::new([ox + x, oy + y, oz + d]),
            Vertex::new([
                ox + radius * (angle + angle_step).cos(),
                oy + radius * (angle + angle_step).sin(),
                oz,
            ]),
        ];

        if d > 0.0 {
            vertices.swap(1, 2);
        }

        triangles.push(stl_io::Triangle {
            normal: normal,
            vertices,
        });

        let mut vertices = [
            Vertex::new([ox + x, oy + y, oz + d]),
            Vertex::new([
                ox + radius * (angle + angle_step).cos(),
                oy + radius * (angle + angle_step).sin(),
                oz + d,
            ]),
            Vertex::new([
                ox + radius * (angle + angle_step).cos(),
                oy + radius * (angle + angle_step).sin(),
                oz,
            ]),
        ];

        if d > 0.0 {
            vertices.swap(1, 2);
        }

        triangles.push(stl_io::Triangle { normal, vertices });
    }

    triangles
}

/// Main program

#[derive(Debug, Clone, Copy)]
pub struct Coin {
    pub radius: f32,
    pub height: f32,
}

/// 2D context
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: f32,
    pub x: f32,
    pub y: f32,
}

pub fn point_is_in_polygon(x: f32, y: f32, polygon: &Vec<(f32, f32)>) -> bool {
    let mut is_inside = false;
    let head = polygon[0];
    let mut min_x = head.0;
    let mut max_x = head.0;
    let mut min_y = head.1;
    let mut max_y = head.1;

    for point in polygon.iter() {
        min_x = min_x.min(point.0);
        max_x = max_x.max(point.0);
        min_y = min_y.min(point.1);
        max_y = max_y.max(point.1);
    }

    if x < min_x || y < min_y || x > max_x || y > max_y {
        return false;
    }

    let mut i = 0;
    let mut j = polygon.len() - 1;
    while i < polygon.len() {
        if (polygon[i].1 > y) != (polygon[j].1 > y)
            && x < (polygon[j].0 - polygon[i].0) * (y - polygon[i].1)
                / (polygon[j].1 - polygon[i].1)
                + polygon[i].0
        {
            is_inside = !is_inside;
        }
        j = i;
        i += 1;
    }

    is_inside
}

pub fn polygon_to_lines(polygon: &Vec<(f32, f32)>) -> Vec<((f32, f32), (f32, f32))> {
    let mut result = Vec::new();
    let li = polygon.len() - 1;
    let mut x = polygon[li].0;
    let mut y = polygon[li].1;

    for point in polygon.iter() {
        let lx = x;
        let ly = y;
        x = point.0;
        y = point.1;
        result.push(((lx, ly), (x, y)));
    }
    result
}

impl Circle {
    pub fn new(radius: f32, x: f32, y: f32) -> Self {
        Circle { radius, x, y }
    }

    pub fn is_point_inside(&self, x: f32, y: f32) -> bool {
        (x - self.x) * (x - self.x) + (y - self.y) * (y - self.y) <= self.radius * self.radius
    }

    pub fn inside_polygon(&self, polygon: &Vec<(f32, f32)>) -> bool {
        if !point_is_in_polygon(self.x, self.y, polygon) {
            return false;
        }

        for line in polygon_to_lines(polygon) {
            let ((x1, y1), (x2, y2)) = line;
            if self.is_point_inside(x1, y1) || self.is_point_inside(x2, y2) {
                return false;
            }

            let dx = x2 - x1;
            let dy = y2 - y1;

            let a = dx * dx + dy * dy;
            let b = 2.0 * (dx * (x1 - self.x) + dy * (y1 - self.y));
            let c = (x1 - self.x) * (x1 - self.x) + (y1 - self.y) * (y1 - self.y)
                - self.radius * self.radius;

            let det = b * b - 4.0 * a * c;

            // this tolerance should be enough
            if det.abs() > 0.0001 {
                let t1 = (-b + det.sqrt()) / (2.0 * a);
                let t2 = (-b - det.sqrt()) / (2.0 * a);

                if 0.0 < t1 && t1 < 1.0 {
                    return false;
                }
                if 0.0 < t2 && t2 < 1.0 {
                    return false;
                }
            }
        }
        true
    }
}

/// a, b, and c must all be positive
#[derive(Debug)]
pub struct ProblemContext {
    pub coin: Coin,
    /// $$ x^2/a^2 $$
    pub a: f32,
    /// y^2/b^2
    pub b: f32,
    /// z^2/c^2
    pub c: f32,
}

impl ProblemContext {
    pub fn generate_ellipse_contexts(&self) -> Vec<EllipseContext> {
        let height = self.coin.height;
        // Generate two different ellipses based on top and bottom
        // take the smaller one
        // stop when both are invalid

        let mut z = -self.c;

        let mut contexts = vec![];

        loop {
            let e1 = ellipse_at_z(self.a, self.b, self.c, z);
            let e2 = ellipse_at_z(self.a, self.b, self.c, z + height);

            // Both should be valid
            if e1.0.is_nan() || e2.0.is_nan() {
                break;
            }

            let d: f32 = height;

            let ellipse = if e1.0 >= e2.0 { e2 } else { e1 };

            contexts.push(EllipseContext::new(self.coin, ellipse.0, ellipse.1, z, d));

            z += height;
        }

        return contexts;
    }
    pub fn get_volume(&self) -> f32 {
        4.0 / 3.0 * PI * self.a * self.b * self.c
    }
}

#[derive(Debug)]
pub struct EllipseContext {
    pub coin: Coin,
    pub a: f32,
    pub b: f32,
    pub d: f32,
    pub z: f32,
    pub polygon: Vec<(f32, f32)>,
}

/// finds the first true value
pub fn linear_binary_search(mut val: f32, func: impl Fn(f32) -> bool, tolerance: f32) -> f32 {
    let mut d = 1.0;

    while !func(val + d) {
        val += d;
        d *= 2.0;

        // This value will likely be handled by the function that called this
        if d > 2e9 {
            return 2e9;
        }
    }

    let mut left = val;
    let mut right = val + d;

    while right - left > tolerance {
        let mid = (left + right) / 2.0;
        if func(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    return (left + right) / 2.0 + 0.0001;
}

impl EllipseContext {
    pub fn new(coin: Coin, a: f32, b: f32, z: f32, d: f32) -> Self {
        let mut polygon = vec![];

        // Use 64 points to approximate the ellipse
        for i in 0..64 {
            let angle = 2.0 * std::f32::consts::PI * i as f32 / 64.0;
            let x = a * angle.cos();
            let y = b * angle.sin();
            polygon.push((x, y));
        }

        return EllipseContext {
            coin,
            a,
            b,
            polygon,
            z,
            d,
        };
    }

    pub fn is_circle_inside(&self, circle: Circle) -> bool {
        return circle.inside_polygon(&self.polygon);
    }

    pub fn fit_circles(&self) -> Vec<Circle> {
        let mut result = vec![];

        // get starting position of circle
        let start_y = linear_binary_search(
            -self.b - self.coin.radius,
            |v| self.is_circle_inside(Circle::new(self.coin.radius, 0.0, v)),
            // If program is too slow, change this number
            0.0001,
        );

        // start from the bottom, work way up
        // this method is HIGHLY inefficient
        // HERE if the code is too slow, fix this first
        for layer in 0..(1.0 + (2.0 * self.b) / (self.coin.radius * 3.0_f32.sqrt())).floor() as i32
        {
            let y = start_y + layer as f32 * self.coin.radius * 3.0_f32.sqrt();
            // start from straight bottom, go right
            // start from slightly right

            let mut circles = vec![];

            if layer % 2 == 0 {
                // starting from middle
                let mut circle = Circle::new(self.coin.radius, 0.0, y);
                if self.is_circle_inside(circle) {
                    circles.push(circle.clone());
                    // push twice from now on
                    loop {
                        circle.x += self.coin.radius * 2.0;
                        if !self.is_circle_inside(circle) {
                            break;
                        }
                        circles.push(circle.clone());
                        // Push another mirrored
                        circles.push(Circle {
                            x: -circle.x,
                            ..circle.clone()
                        });
                    }
                }
            } else {
                // starting slightly right
                let mut circle = Circle::new(self.coin.radius, self.coin.radius, y);
                while self.is_circle_inside(circle) {
                    circles.push(circle.clone());
                    // Push another mirrored
                    circles.push(Circle {
                        x: -circle.x,
                        ..circle.clone()
                    });
                    circle.x += self.coin.radius * 2.0;
                }
            }

            result.extend(circles);
        }

        return result;
    }
}

pub fn ellipse_at_z(a: f32, b: f32, c: f32, z: f32) -> (f32, f32) {
    let zc = z * z / (c * c);
    let na = (a * a * (1.0 - zc)).sqrt();
    let nb = (b * b * (1.0 - zc)).sqrt();
    (na, nb)
}

pub struct Config {
    coin_radius: f32,
    coin_height: f32,
    a: f32,
    b: f32,
    c: f32,
}

impl Config {
    pub fn from_args(args: Vec<f32>) -> Config {
        // Get item at index or return default values
        let coin_radius = args.get(0).unwrap_or(&(19.05 / 2.0)).to_owned();
        let coin_height = args.get(1).unwrap_or(&1.52).to_owned();
        let a = args.get(2).unwrap_or(&150.0).to_owned();
        let b = args.get(3).unwrap_or(&300.0).to_owned();
        let c = args.get(4).unwrap_or(&150.0).to_owned();

        Config {
            coin_radius,
            coin_height,
            a,
            b,
            c,
        }
    }
}

pub fn run(config: Config) {
    // [x,y,z]=[300,600,300]
    // x parameter ranging from [-150,150]
    // y parameter ranging from [-300,300]
    // z parameter ranging from [-150,150]

    // Diameter 19.05 mm, Thickness/Height 1.52 mm: an American penny pretty much
    let problem = ProblemContext {
        coin: Coin {
            radius: config.coin_radius,
            height: config.coin_height,
        },
        a: config.a,
        b: config.b,
        c: config.c,
    };

    println!("current configuration: \n{:#?}", problem);

    let mut circles = vec![];

    let mut meshvec = Vec::new();

    for ellipse in problem.generate_ellipse_contexts() {
        let cs = ellipse.fit_circles();

        for circle in cs {
            meshvec.extend(make_cylinder(
                circle.x,
                circle.y,
                ellipse.z,
                problem.coin.radius,
                ellipse.d,
            ));
            circles.push(circle);
        }
    }

    println!("{} circles found", circles.len());

    let vol = problem.get_volume();
    println!(
        "The circles fill {}% of the total volume {}",
        circles.len() as f32 * PI * problem.coin.radius.powi(2) * problem.coin.height / vol * 100.0,
        vol
    );

    // Write it to stl format file
    let fx = std::fs::remove_file("mesh.stl");
    match fx {
        Ok(_) => println!("Replacing old mesh file"),
        Err(__export) => println!("Generating new file"),
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("mesh.stl")
        .unwrap();
    stl_io::write_stl(&mut file, meshvec.iter()).unwrap();
}
