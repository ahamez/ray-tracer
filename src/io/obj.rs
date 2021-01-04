/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{GroupBuilder, Object},
};
use std::{
    collections::HashMap,
    error::Error,
    fmt,
    io::{prelude::*, BufReader},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
struct FaceVertex {
    pub vertex_index: usize,
    pub normal_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
struct Face {
    pub vertices: Vec<FaceVertex>,
    pub group: Option<String>,
}

impl Face {
    fn has_normals(&self) -> bool {
        self.vertices[0].normal_index != None
    }
}

impl Default for Face {
    fn default() -> Self {
        Self {
            vertices: vec![],
            group: None,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Debug)]
struct Data {
    pub ignored: usize,
    pub vertices: Vec<Point>,
    pub normals: Vec<Vector>,
    pub faces: Vec<Face>,
}

impl Data {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            ignored: 0,
            // A dummy point is added as vertices are addressed in a 1-based fashion
            vertices: vec![Point::zero()],
            // A dummy vector is added as normals are addressed in a 1-based fashion
            normals: vec![Vector::zero()],
            faces: vec![],
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

/* ---------------------------------------------------------------------------------------------- */

fn parse_group(
    line_vec: &[&str],
    line: &str,
    line_number: usize,
) -> Result<Option<String>, ParseError> {
    if line_vec.len() != 2 {
        let err_msg = format!("Invalid group `{}` at line {}", line.trim(), line_number);
        return Err(ParseError(err_msg));
    }

    Ok(Some(line_vec[1].into()))
}

/* ---------------------------------------------------------------------------------------------- */

fn parse_vertex(
    line_vec: &[&str],
    line: &str,
    line_number: usize,
    mut data: Data,
) -> Result<Data, ParseError> {
    let err_msg = format!("Invalid vertex `{}` at line {}", line.trim(), line_number);
    let err_fn = |_| ParseError(err_msg.clone());

    if line_vec.len() != 4 {
        return Err(ParseError(err_msg.clone()));
    }

    let x = line_vec[1].parse::<f64>().map_err(err_fn)?;
    let y = line_vec[2].parse::<f64>().map_err(err_fn)?;
    let z = line_vec[3].parse::<f64>().map_err(err_fn)?;

    data.vertices.push(Point::new(x, y, z));

    Ok(data)
}

/* ---------------------------------------------------------------------------------------------- */

fn parse_normal(
    line_vec: &[&str],
    line: &str,
    line_number: usize,
    mut data: Data,
) -> Result<Data, ParseError> {
    let err_msg = format!("Invalid normal `{}` at line {}", line.trim(), line_number);
    let err_fn = |_| ParseError(err_msg.clone());

    if line_vec.len() != 4 {
        return Err(ParseError(err_msg.clone()));
    }

    let x = line_vec[1].parse::<f64>().map_err(err_fn)?;
    let y = line_vec[2].parse::<f64>().map_err(err_fn)?;
    let z = line_vec[3].parse::<f64>().map_err(err_fn)?;

    data.normals.push(Vector::new(x, y, z));

    Ok(data)
}

/* ---------------------------------------------------------------------------------------------- */

fn parse_face(
    line_vec: &[&str],
    line: &str,
    line_number: usize,
    mut data: Data,
    current_group: &Option<String>,
) -> Result<Data, ParseError> {
    let err_msg = format!("Invalid face `{}` at line {}", line.trim(), line_number);
    let err_fn = |_| ParseError(err_msg.clone());

    if line_vec.len() < 4 {
        return Err(ParseError(err_msg.clone()));
    }

    let mut face = Face {
        vertices: vec![],
        group: current_group.clone(),
    };
    for vertex in line_vec.iter().skip(1) {
        let (vertex_index, normal_index) = match vertex.parse::<usize>() {
            Ok(value) => (value, None),
            Err(_) => {
                let extended = vertex.split('/').collect::<Vec<&str>>();
                if extended.len() != 3 {
                    return Err(ParseError(err_msg.clone()));
                }

                let vertex_index = extended[0].parse::<usize>().map_err(err_fn)?;
                let normal_index = extended[2].parse::<usize>().ok();

                (vertex_index, normal_index)
            }
        };

        face.vertices.push(FaceVertex {
            vertex_index,
            normal_index,
        });
    }

    data.faces.push(face);

    Ok(data)
}

/* ---------------------------------------------------------------------------------------------- */

fn parse_data(s: &str) -> Result<Data, ParseError> {
    let buf = BufReader::new(s.as_bytes());
    let mut data = Data::new();
    let mut line_number = 1;
    let mut current_group = None;

    for line in buf.lines() {
        if let Ok(line) = line {
            let vec = line.split_whitespace().collect::<Vec<&str>>();
            if !vec.is_empty() {
                if vec[0] == "g" {
                    current_group = parse_group(&vec[..], &line, line_number)?;
                } else if vec[0] == "v" {
                    data = parse_vertex(&vec[..], &line, line_number, data)?;
                } else if vec[0] == "vn" {
                    data = parse_normal(&vec[..], &line, line_number, data)?;
                } else if vec[0] == "f" {
                    data = parse_face(&vec[..], &line, line_number, data, &current_group)?;
                } else {
                    data.ignored += 1;
                }
            } else {
                data.ignored += 1;
            }
        }
        line_number += 1;
    }

    Ok(data)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_triangles(face: &Face, vertices: &[Point], normals: &[Vector]) -> Vec<Object> {
    let mut triangles = vec![];

    for i in 1..face.vertices.len() - 1 {
        if face.has_normals() {
            triangles.push(Object::new_smooth_triangle(
                vertices[face.vertices[0].vertex_index],
                vertices[face.vertices[i].vertex_index],
                vertices[face.vertices[i + 1].vertex_index],
                normals[face.vertices[0].normal_index.unwrap()],
                normals[face.vertices[i].normal_index.unwrap()],
                normals[face.vertices[i + 1].normal_index.unwrap()],
            ));
        } else {
            triangles.push(Object::new_triangle(
                vertices[face.vertices[0].vertex_index],
                vertices[face.vertices[i].vertex_index],
                vertices[face.vertices[i + 1].vertex_index],
            ));
        }
    }

    triangles
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_group(triangles: Vec<Object>) -> GroupBuilder {
    GroupBuilder::Node(
        Object::new_dummy(),
        triangles
            .iter()
            .map(|o| GroupBuilder::Leaf(o.clone()))
            .collect(),
    )
}

/* ---------------------------------------------------------------------------------------------- */

pub fn parse_str(s: &str) -> Result<Object, ParseError> {
    let data = parse_data(s)?;

    let mut anonymous = vec![];
    let mut named = HashMap::new();

    for face in data.faces {
        let triangles = mk_triangles(&face, &data.vertices, &data.normals);
        let group = mk_group(triangles);

        match face.group {
            None => anonymous.push(group),
            Some(name) => match named.get_mut(&name) {
                None => {
                    named.insert(name, vec![group]);
                }
                Some(entry) => entry.push(group),
            },
        }
    }

    let anonymous_group = GroupBuilder::Node(Object::new_dummy(), anonymous);

    if named.is_empty() {
        Ok(Object::new_group(&anonymous_group))
    } else {
        let mut groups = vec![anonymous_group];
        for (_, triangles) in named {
            groups.push(GroupBuilder::Node(Object::new_dummy(), triangles))
        }

        Ok(Object::new_group(&GroupBuilder::Node(
            Object::new_dummy(),
            groups,
        )))
    }
}

/* ---------------------------------------------------------------------------------------------- */

pub fn parse_file(path: &std::path::Path) -> Result<Object, ParseError> {
    let s = std::fs::read_to_string(path).unwrap();
    parse_str(&s)
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignoring_unrecognized_lines() {
        let txt = r#"
        foo
        bar dqskdqs

        dqsqds
        "#;

        let data = parse_data(&txt).unwrap();
        assert_eq!(data.ignored, 6);
    }

    #[test]
    fn vertex_records() {
        let txt = r#"
        v -1 1 0
        v -1.0000 0.5000 0.0000
        v 1 0 0
        v 1 1 0
        dqsqds
        "#;

        let data = parse_data(&txt).unwrap();
        assert_eq!(data.ignored, 3);
        assert_eq!(data.vertices.len(), 5);
        assert_eq!(data.vertices[1], Point::new(-1.0, 1.0, 0.0));
        assert_eq!(data.vertices[2], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(data.vertices[3], Point::new(1.0, 0.0, 0.0));
        assert_eq!(data.vertices[4], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn vertex_normal_records() {
        let txt = r#"
        vn 0 0 1
        vn 0.707 0 -0.707
        vn 1 2 3
        "#;

        let data = parse_data(&txt).unwrap();
        assert_eq!(data.normals.len(), 4);
        assert_eq!(data.normals[1], Vector::new(0.0, 0.0, 1.0));
        assert_eq!(data.normals[2], Vector::new(0.707, 0.0, -0.707));
        assert_eq!(data.normals[3], Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn vertex_invalid_record() {
        {
            let txt = r#"
                v -1 1 0
                v -1.0000 0.5000 0.0000
                v 3
                v 1 0 0
                v 1 1 0
                "#;

            let data = parse_data(&txt);
            assert!(data.is_err());
            let err = data.unwrap_err();
            assert_eq!(err.0, "Invalid vertex `v 3` at line 4");
        }
        {
            let txt = r#"
                v -1 a 0
                "#;

            let data = parse_data(&txt);
            assert!(data.is_err());
            let err = data.unwrap_err();
            assert_eq!(err.0, "Invalid vertex `v -1 a 0` at line 2");
        }
    }

    #[test]
    fn parse_triangle_faces() {
        {
            let txt = r#"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0

                f 1 2 3
                f 1 3 4
                "#;

            let data = parse_data(&txt).unwrap();

            assert_eq!(data.ignored, 3);
            assert_eq!(data.vertices.len(), 5);
            assert_eq!(data.vertices[1], Point::new(-1.0, 1.0, 0.0));
            assert_eq!(data.vertices[2], Point::new(-1.0, 0.0, 0.0));
            assert_eq!(data.vertices[3], Point::new(1.0, 0.0, 0.0));
            assert_eq!(data.vertices[4], Point::new(1.0, 1.0, 0.0));

            assert_eq!(data.faces.len(), 2);
            assert_eq!(
                data.faces[0],
                Face {
                    group: None,
                    vertices: vec![
                        FaceVertex {
                            vertex_index: 1,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 2,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 3,
                            normal_index: None
                        }
                    ]
                }
            );
            assert_eq!(
                data.faces[1],
                Face {
                    group: None,
                    vertices: vec![
                        FaceVertex {
                            vertex_index: 1,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 3,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 4,
                            normal_index: None
                        }
                    ]
                }
            );
        }
        {
            let txt = r#"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0

                g FirstGroup
                f 1 2 3
                g SecondGroup
                f 1 3 4
                f 2 3 4
                "#;

            let data = parse_data(&txt).unwrap();

            assert_eq!(data.ignored, 3);
            assert_eq!(data.vertices.len(), 5);
            assert_eq!(data.vertices[1], Point::new(-1.0, 1.0, 0.0));
            assert_eq!(data.vertices[2], Point::new(-1.0, 0.0, 0.0));
            assert_eq!(data.vertices[3], Point::new(1.0, 0.0, 0.0));
            assert_eq!(data.vertices[4], Point::new(1.0, 1.0, 0.0));

            assert_eq!(data.faces.len(), 3);
            assert_eq!(
                data.faces[0],
                Face {
                    group: Some("FirstGroup".to_string()),
                    vertices: vec![
                        FaceVertex {
                            vertex_index: 1,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 2,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 3,
                            normal_index: None
                        }
                    ]
                }
            );
            assert_eq!(
                data.faces[1],
                Face {
                    group: Some("SecondGroup".to_string()),
                    vertices: vec![
                        FaceVertex {
                            vertex_index: 1,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 3,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 4,
                            normal_index: None
                        }
                    ]
                }
            );
            assert_eq!(
                data.faces[2],
                Face {
                    group: Some("SecondGroup".to_string()),
                    vertices: vec![
                        FaceVertex {
                            vertex_index: 2,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 3,
                            normal_index: None
                        },
                        FaceVertex {
                            vertex_index: 4,
                            normal_index: None
                        }
                    ]
                }
            );
        }
    }

    #[test]
    fn polygon() {
        {
            let txt = r#"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0
                v 0 2 0

                f 1 2 3 4 5
                "#;

            let data = parse_data(&txt).unwrap();

            let face = &data.faces[0];
            let triangles = mk_triangles(face, &data.vertices, &data.normals);

            assert_eq!(triangles.len(), 3);

            let t0 = triangles[0].shape().as_triangle().unwrap();
            assert_eq!(t0.p1(), data.vertices[1]);
            assert_eq!(t0.p2(), data.vertices[2]);
            assert_eq!(t0.p3(), data.vertices[3]);

            let t1 = triangles[1].shape().as_triangle().unwrap();
            assert_eq!(t1.p1(), data.vertices[1]);
            assert_eq!(t1.p2(), data.vertices[3]);
            assert_eq!(t1.p3(), data.vertices[4]);

            let t2 = triangles[2].shape().as_triangle().unwrap();
            assert_eq!(t2.p1(), data.vertices[1]);
            assert_eq!(t2.p2(), data.vertices[4]);
            assert_eq!(t2.p3(), data.vertices[5]);
        }
    }

    #[test]
    fn faces_with_normal() {
        let txt = r#"
        v 0 1 0
        v -1 0 0
        v 1 0 0

        vn -1 0 0
        vn 1 0 0
        vn 0 1 0

        f 1//3 2//1 3//2
        f 1/0/3 2/102/1 3/14/2
        "#;

        let data = parse_data(&txt).unwrap();

        let face0 = &data.faces[0];
        let face0_triangles = mk_triangles(face0, &data.vertices, &data.normals);

        assert_eq!(face0_triangles.len(), 1);

        let t0 = face0_triangles[0].shape().as_smooth_triangle().unwrap();
        assert_eq!(t0.p1(), data.vertices[1]);
        assert_eq!(t0.p2(), data.vertices[2]);
        assert_eq!(t0.p3(), data.vertices[3]);
        assert_eq!(t0.n1(), data.normals[3]);
        assert_eq!(t0.n2(), data.normals[1]);
        assert_eq!(t0.n3(), data.normals[2]);

        let face1 = &data.faces[0];
        let face1_triangles = mk_triangles(face1, &data.vertices, &data.normals);

        assert_eq!(face1_triangles.len(), 1);

        let t1 = face1_triangles[0].shape().as_smooth_triangle().unwrap();
        assert_eq!(t1.p1(), data.vertices[1]);
        assert_eq!(t1.p2(), data.vertices[2]);
        assert_eq!(t1.p3(), data.vertices[3]);
        assert_eq!(t1.n1(), data.normals[3]);
        assert_eq!(t1.n2(), data.normals[1]);
        assert_eq!(t1.n3(), data.normals[2]);
    }
}

/* ---------------------------------------------------------------------------------------------- */
