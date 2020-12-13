use crate::{
    shapes::Sphere, Color, Colors, Computations, Intersection, Point, PointLight, Ray,
    Transformation,
};

/// A collection of all objects in a scene.
///
/// Routines for intersecting that world with a ray and computer the colors for
/// intersections.
#[derive(Debug)]
pub struct World {
    // Light source of the world.
    pub light: Option<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    /// Create a world with no objects and no lights.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::World;
    ///
    /// let w = World::new();
    ///
    /// assert!(w.light.is_none());
    /// ```
    pub fn new() -> Self {
        World {
            light: None,
            objects: Vec::new(),
        }
    }

    /// Add an `object` to the world `self`.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Sphere, World};
    ///
    /// let mut w = World::new();
    /// let s = Sphere::new();
    /// let s_id = s.id;
    /// w.add_object(s);
    /// let s = w.get_object(0).unwrap();
    ///
    /// assert_eq!(s.id, s_id);
    /// ```
    pub fn add_object(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    /// Iterate over all of the objects added to the world. Intersecting each
    /// object with a ray and aggregating the intersections into a single
    /// collection. The collection is sorted.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Intersection, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let xs = w.intersect_world(r).expect("No intersections found!");
    ///
    /// assert_eq!(xs.len(), 4);
    /// assert_eq!(xs[0].t, 4.0);
    /// assert_eq!(xs[1].t, 4.5);
    /// assert_eq!(xs[2].t, 5.5);
    /// assert_eq!(xs[3].t, 6.0);
    pub fn intersect_world(&self, r: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();
        for o in &self.objects {
            if let Some(o_xs) = o.intersect(r) {
                for i in o_xs {
                    xs.push(i);
                }
            }
        }

        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }

    /// Call the `lighting` function for the [`crate::Material`] of a `shape` intersected
    /// by a [`Ray`] to get the [`Color`] at that intersection.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Color, Intersection, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    /// let shape = w.get_object(0).expect("Object not found!");
    /// let i = Intersection::new(4.0, &shape);
    /// let comps = i.prepare_computations(r);
    /// let c = w.shade_hit(&comps);
    ///
    /// assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    /// ```
    pub fn shade_hit(&self, comps: &Computations) -> Color {
        let shadowed = self.is_shadow(comps.over_point);

        comps.object.material.lighting(
            self.light.expect("World has no light source"),
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        )
    }

    /// Returns a [`Color`] for an intersection by doing the following
    ///
    /// 1. Find the [`Intersection`]s of a [`Ray`] by calling `intersect_world`.
    /// 2. Find the `hit` from the resulting intersections.
    /// 3. Return black if there are no intersections.
    /// 4. `prepare_computations` on the `hit` to get the [`Computations`] for
    /// the [`Intersection`].
    /// 5. Call `shade_hit` to get the color at the `hit`.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{Color, Point, Ray, Vector, World};
    ///
    /// let w = World::default();
    /// let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 1.0));
    /// let c = w.color_at(r);
    ///
    /// assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    /// ```
    pub fn color_at(&self, r: Ray) -> Color {
        match self.intersect_world(r) {
            Some(xs) => match Intersection::hit(&xs) {
                Some(i) => {
                    let comps = i.prepare_computations(r);
                    self.shade_hit(&comps)
                }
                None => Colors::BLACK,
            },
            None => Colors::BLACK,
        }
    }

    /// Cast a ray, called a *shadow ray*, from the point of an intersection
    /// towards the light source. If an object intersects that *shadow ray* between
    /// the intersection point and the light source, then the point of intersection
    /// is considered to be in shadow, returning `true` otherwise
    /// return `false`.
    pub fn is_shadow(&self, point: Point) -> bool {
        let v = self.light.expect("No light in world!").position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        if let Some(intersections) = self.intersect_world(r) {
            if let Some(hit) = Intersection::hit(&intersections) {
                if hit.t < distance {
                    return true;
                }
            }
        }

        false
    }

    /// Returns a reference to an `object` at the given index or `None`
    /// if index is out of range.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Sphere, World};
    ///
    /// let mut w = World::new();
    /// let s = Sphere::new();
    /// let s_id = s.id;
    /// w.add_object(s);
    /// let s = w.get_object(0).unwrap();
    ///
    /// assert_eq!(s.id, s_id);
    /// ```
    pub fn get_object(&self, index: usize) -> Option<&Sphere> {
        self.objects.get(index)
    }

    /// Returns a mutable reference to an `object` at the given index or `None`
    /// if index is out of range.
    ///
    /// Example
    ///
    /// ```
    /// use rustic_ray::{shapes::Sphere, World};
    ///
    /// let mut w = World::new();
    /// let s = Sphere::new();
    /// let s_id = s.id;
    /// w.add_object(s);
    /// let s = w.get_object_mut(0).unwrap();
    /// s.material.diffuse = 2.0;
    ///
    /// assert_eq!(2.0, s.material.diffuse);
    /// ```
    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut Sphere> {
        self.objects.get_mut(index)
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();

        w.light = Some(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        w.add_object(s1);

        let mut s2 = Sphere::new();
        s2.transform = Transformation::new().scale(0.5, 0.5, 0.5).build();
        w.add_object(s2);

        w
    }
}

#[cfg(test)]
mod tests {
    use crate::{Ray, Vector};

    use super::*;

    // Chapter 7 Making a Scene
    // Page 92
    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    // Chapter 7 Making a Scene
    // Page 92
    #[test]
    fn the_default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = Transformation::new().scale(0.5, 0.5, 0.5).build();

        let w = World::default();

        assert_eq!(w.light.expect("There are not lights!"), light);
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.light.expect("No light source"), light);
        // Each object gets an ID therefore the id of the object created in
        // World::default() will not be the same. The transformation and material
        // should be.
        assert_eq!(w.objects[0].transform, s1.transform);
        assert_eq!(w.objects[0].material, s1.material);
        assert_eq!(w.objects[1].transform, s2.transform);
        assert_eq!(w.objects[1].material, s2.material);
    }

    // Chapter 7 Making a Scene
    // Page 92 & 93
    #[test]
    fn intersecting_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(r).expect("No intersections found!");

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    pub fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.get_object(0).expect("Object not found!");
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    pub fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.get_object(1).expect("Object not found!");
        let i = Intersection::new(0.5, &shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 1.0));
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Chapter 7 Making a Scene
    // Page 96
    #[test]
    pub fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        {
            let outer = w.get_object_mut(0).expect("Object not found!");
            outer.material.ambient = 1.0;
            let inner = w.get_object_mut(1).expect("Object not found!");
            inner.material.ambient = 1.0;
        }
        let inner = w.get_object(1).expect("Object not found!");
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r);

        assert_eq!(c, inner.material.color);
    }

    // Chapter 8 Shadows
    // Page 111
    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 112
    #[test]
    fn there_is_no_shadow_when_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadow(p));
    }

    // Chapter 8 Shadows
    // Page 114
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.light = Some(PointLight::new(
            Point::new(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let s1 = Sphere::new();
        w.add_object(s1);

        let mut s2 = Sphere::new();
        s2.transform = Transformation::new().translate(0.0, 0.0, 10.0).build();
        w.add_object(s2);

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.get_object(1).expect("Object not found!"));
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
