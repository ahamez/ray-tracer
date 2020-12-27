/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{Object, Ray},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    objects: Vec<Object>,
}

/* ---------------------------------------------------------------------------------------------- */

impl Group {
    #[allow(clippy::eq_op)]
    pub fn intersects<F>(&self, _ray: &Ray, mut _push: F)
    where
        F: FnMut(f64),
    {
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        unimplemented!()
    }
}

/* ---------------------------------------------------------------------------------------------- */
