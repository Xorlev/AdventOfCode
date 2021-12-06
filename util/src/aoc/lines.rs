use aoc::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> LineSegment {
        LineSegment { start, end }
    }

    pub fn len(&self) -> i32 {
        // As these segments are always horizontal or vertical, we can be very lazy about this.
        let x_len = (self.start.x - self.end.x).abs();
        let y_len = (self.start.y - self.start.y).abs();

        std::cmp::max(x_len, y_len)
    }

    pub fn intersection(&self, other_segment: &LineSegment) -> Option<Point> {
        match self.relate(other_segment) {
            LineRelation::DivergentIntersecting(point) => Some(point),
            _ => None,
        }
    }


    /// Borrowed from line_intersection crate, and adapted for this integer-only AoC world.
    pub fn relate(&self, other: &LineSegment) -> LineRelation {
        // see https://stackoverflow.com/a/565282
        let p = self.start;
        let q = other.start;
        let r = self.end - self.start;
        let s = other.end - other.start;

        let r_cross_s = Self::cross(&r, &s);
        let q_minus_p = q - p;
        let q_minus_p_cross_r = Self::cross(&q_minus_p, &r);

        // are the lines are parallel?
        if r_cross_s == 0f32 {
            // are the lines collinear?
            if q_minus_p_cross_r == 0f32 {
                // the lines are collinear
                LineRelation::Collinear
            } else {
                // the lines are parallel but not collinear
                LineRelation::Parallel
            }
        } else {
            // the lines are not parallel
            let t = Self::cross_div(&q_minus_p, &s, r_cross_s);
            let u = Self::cross_div(&q_minus_p, &r, r_cross_s);

            // are the intersection coordinates both in range?
            let t_in_range = 0f32 <= t && t <= 1f32;
            let u_in_range = 0f32 <= u && u <= 1f32;

            if t_in_range && u_in_range {
                // there is an intersection
                LineRelation::DivergentIntersecting(Self::t_coord_to_point(&p, &r, t))
            } else {
                // there is no intersection
                LineRelation::DivergentDisjoint
            }
        }
    }

    fn cross(a: &Point, b: &Point) -> f32 {
        (a.x * b.y - a.y * b.x) as f32
    }

    fn cross_div(a: &Point, b: &Point, r_cross_s: f32) -> f32 {
        let x = b.x as f32 / r_cross_s;
        let y = b.y as f32 / r_cross_s;

        a.x as f32 * y - a.y as f32 * x
    }

    fn t_coord_to_point(p: &Point, r: &Point, t: f32) -> Point {
        let t_x = t * r.x as f32;
        let t_y = t * r.y as f32;
        Point::new(p.x + t_x as i32, p.y + t_y as i32)
    }
}

/// The relationship between two line segments.
#[derive(Debug, PartialEq)]
pub enum LineRelation {
    /// The line intervals are not parallel (or anti-parallel), and "meet" each other at exactly
    /// one point.
    DivergentIntersecting(Point),
    /// The line intervals are not parallel (or anti-parallel), and do not intersect; they "miss"
    /// each other.
    DivergentDisjoint,
    /// The line intervals lie on the same line. They may or may not overlap, and this intersection
    /// is possibly infinite.
    Collinear,
    /// The line intervals are parallel or anti-parallel.
    Parallel,
}
