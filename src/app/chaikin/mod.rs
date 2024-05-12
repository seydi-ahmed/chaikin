pub use iced::Point;

pub fn chaikin(points: &Vec<Point>, iterations: usize) -> Vec<Point> {
    if iterations == 0 {
        return points.clone();
    }

    let mut new_points = Vec::new();

    // Keep the first point
    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];

        let q = Point::new(0.75 * p0.x + 0.25 * p1.x, 0.75 * p0.y + 0.25 * p1.y);

        let r = Point::new(0.25 * p0.x + 0.75 * p1.x, 0.25 * p0.y + 0.75 * p1.y);

        new_points.push(q);
        new_points.push(r);
    }

    // Keep the last point
    new_points.push(*points.last().unwrap());

    chaikin(&new_points, iterations - 1)
}
