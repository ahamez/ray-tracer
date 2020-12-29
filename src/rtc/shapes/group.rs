/* ---------------------------------------------------------------------------------------------- */

use std::sync::Arc;

use crate::rtc::{Object, Ray};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    objects: Vec<Arc<Object>>,
}

/* ---------------------------------------------------------------------------------------------- */

impl Group {
    pub fn new(objects: Vec<Arc<Object>>) -> Self {
        Self { objects }
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        unimplemented!()
    pub fn intersects(&self, ray: &Ray, push: &mut impl FnMut(f64))
    {
        for o in &self.objects {
            o.intersects(&ray, push);
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
