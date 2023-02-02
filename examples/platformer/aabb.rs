use aftershock::vector2::*;
use aftershock::math::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub position: Vector2,
    pub velocity: Vector2,
    pub extents: Vector2,
}

impl AABB {
    pub fn new(position: Vector2, extents: Vector2) -> AABB {
        AABB {
            position, extents, velocity: Vector2::ZERO
        }
    }

    pub fn closest_point(point: Vector2, aabb: &AABB) -> Vector2 {
        Vector2::new(
            f32::clamp(point.x, aabb.position.x - (aabb.extents.x * 0.5), aabb.position.x + (aabb.extents.x * 0.5)),
            f32::clamp(point.x, aabb.position.x - (aabb.extents.x * 0.5), aabb.position.x + (aabb.extents.x * 0.5))
        )
    }

    pub fn overlap_point(point: Vector2, aabb: &AABB) -> bool {
        point.x > aabb.position.x - (aabb.extents.x * 0.5) &&
        point.x < aabb.position.x + (aabb.extents.x * 0.5) &&
        point.y > aabb.position.y - (aabb.extents.y * 0.5) &&
        point.y < aabb.position.y + (aabb.extents.y * 0.5)
    }

    pub fn overlap_aabb(aabb1: &AABB, aabb2: &AABB) -> bool {
        let aabb_sum: AABB = AABB::new(aabb2.position, aabb1.extents + aabb2.extents);
        AABB::overlap_point(aabb1.position, &aabb_sum)
    }


    /// Cast rays on each side of the box. If a ray point starts inside the box, it will return its closest point.
    pub fn raycast_aabb(aabb: &AABB, ray_start: Vector2, ray_end: Vector2) -> Option<(Vector2, Vector2)> {
        // First check if any or both of the ray points are inside

        let ray_start_overlap: bool = AABB::overlap_point(ray_start, &aabb);
        let ray_end_overlap: bool = AABB::overlap_point(ray_start, &aabb);

        let mut ray_end = ray_end;

        // Entire ray is inside. Fire ray in opposite direction
        // This will partly emulate the box having volume on moving objects.
        if ray_start_overlap && ray_end_overlap {
            ray_end = ray_start + -(Vector2::direction(ray_start, ray_end) * aabb.extents.magnitude());
        }

        let mut closest_intersection: Option<(Vector2, Vector2)> = None;

        let aabb_top_left:      Vector2 = Vector2::new(aabb.position.x - (aabb.extents.x * 0.5), aabb.position.y - (aabb.extents.y * 0.5));
        let aabb_top_right:     Vector2 = Vector2::new(aabb.position.x + (aabb.extents.x * 0.5), aabb.position.y - (aabb.extents.y * 0.5));
        let aabb_bottom_left:   Vector2 = Vector2::new(aabb.position.x - (aabb.extents.x * 0.5), aabb.position.y + (aabb.extents.y * 0.5));
        let aabb_bottom_right:  Vector2 = Vector2::new(aabb.position.x + (aabb.extents.x * 0.5), aabb.position.y + (aabb.extents.y * 0.5));

        let mut intersections: [Option<Vector2>; 4] = [None; 4];

        intersections[0] = Vector2::intersection_segment(ray_start, ray_end, aabb_top_left, aabb_top_right);
        intersections[1] = Vector2::intersection_segment(ray_start, ray_end, aabb_top_right, aabb_bottom_right);
        intersections[2] = Vector2::intersection_segment(ray_start, ray_end, aabb_bottom_right, aabb_bottom_left);
        intersections[3] = Vector2::intersection_segment(ray_start, ray_end, aabb_bottom_left, aabb_top_left);
        
        for i in 0..intersections.len() {
            let normal = match i {
                0 => { Vector2::UP },
                1 => { Vector2::RIGHT },
                2 => { Vector2::DOWN },
                3 => { Vector2::LEFT },
                _ => { Vector2::ZERO }
            };

            if intersections[i].is_some() {
                if closest_intersection.is_none() {
                    closest_intersection = Some((intersections[i].unwrap(), normal));
                } else {
                    let dist1 = Vector2::distance(ray_start, closest_intersection.unwrap().0);
                    let dist2 = Vector2::distance(ray_start, intersections[i].unwrap());
    
                    if dist2 < dist1 {
                        closest_intersection = Some((intersections[i].unwrap(), normal));
                    }
                }
            }
        }

        closest_intersection
    }

    pub fn resolve_aabb(dynamic: &mut AABB, rigid: &AABB, dt: f32) -> Option<(Vector2, Vector2)> {

        let next_frame_dynamic: AABB = AABB::new(dynamic.position + (dynamic.velocity * dt), dynamic.extents);

        // Sum AABB that represents outermost ring around rigid
        let resolve: AABB = AABB::new(rigid.position, rigid.extents + dynamic.extents);

        if AABB::overlap_aabb(&next_frame_dynamic, &resolve) {

            
            // Keep the ray inside the dynamic AABB
            // Otherwise high velocities can collide on far away AABB's
            //let ray_end: Vector2 = AABB::closest_point(next_frame_dynamic.position, &dynamic);


            let cast: Option<(Vector2, Vector2)> = AABB::raycast_aabb(&resolve, dynamic.position, next_frame_dynamic.position);

            cast
        } else { None }
    }
}