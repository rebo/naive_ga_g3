use crate::elements::{Bivector, Rotor, Vector};
use float_cmp::ApproxEq;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub origin: Vector,
    pub direction: Vector,
}

impl Line {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn point_on_line(self, s: f64) -> Vector {
        self.origin + s * self.direction
    }

    pub fn intersection_with_plane_parameter(self, plane: Plane) -> Option<f64> {
        let v = self.direction;
        let b = plane.bivector;
        //
        let v0 = self.origin - plane.origin;

        // so line is v_0 + s * v

        let s_numerator = (-(v0 ^ b)).0;
        let s_denominator = (v ^ b).0;

        if s_denominator.approx_eq(&0.0, 2.0 * std::f64::EPSILON, 2) {
            None
        } else {
            Some(s_numerator / s_denominator)
        }
    }

    pub fn intersection_with_plane_location(self, plane: Plane) -> Option<Vector> {
        match self.intersection_with_plane_parameter(plane) {
            Some(s) => Some(self.point_on_line(s)),
            None => None,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Plane {
    origin: Vector,
    bivector: Bivector,
    north: Vector,
    east: Vector,
}

impl Plane {
    pub fn new(origin: Vector, bivector: Bivector, eastish: Vector) -> Plane {
        // this points 'north' and is in the plane of the bivector
        let north = eastish.dot(bivector);

        let rotor = Rotor::from_exp(-std::f64::consts::PI / 4.0, bivector.normalize());

        let east = north.apply_rotor(rotor);

        Plane {
            origin,
            bivector,
            north: north,
            east: east,
        }
    }

    pub fn point_on_plane(self, s: f64, t: f64) -> Vector {
        self.origin + s * self.north + t * self.east
    }

    pub fn dist_to_point(self, point: Vector) -> f64 {
        // relative to 'origin' of plane
        let other_point = point - self.origin;

        let directed_volume = other_point ^ self.bivector;
        // the above is the volume of the paralelapiped with base the bivector area
        // and height the perpendicular distance to the point

        directed_volume.0 / self.bivector.mag()
    }

    pub fn vector_to_point(self, point: Vector) -> Vector {
        // relative to 'origin' of plane
        let other_point = point - self.origin;

        let directed_volume = other_point ^ self.bivector;
        // the above is the volume of the paralelapiped with base the bivector area
        // and height the perpendicular distance to the point

        // directed_volume = h ^ self.bivector + h_par dot bivector = h * self.bivector
        // so h = directed_volume * self.bivector.inv()
        directed_volume * self.bivector.inv()
    }

    pub fn intersection_with_line_parameter(self, line: Line) -> Option<f64> {
        line.intersection_with_plane_parameter(self)
    }

    pub fn intersection_with_line_location(self, line: Line) -> Option<Vector> {
        line.intersection_with_plane_location(self)
    }
}
