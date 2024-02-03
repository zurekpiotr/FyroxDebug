use std::slice::Iter;

use fyrox::core::algebra::{Vector2, Vector3, Vector4};
use fyrox::core::math::TriangleDefinition;
use fyrox::scene::mesh::vertex::StaticVertex;

const CLOCK_WISE_TRIANGLE: [TriangleDefinition; 2] =
    [TriangleDefinition([2, 1, 0]), TriangleDefinition([3, 2, 0])];

const COUNTER_CLOCK_WISE_TRIANGLE: [TriangleDefinition; 2] =
    [TriangleDefinition([0, 1, 2]), TriangleDefinition([0, 2, 3])];

const FRONT: [StaticVertex; 4] = [
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, 0.5),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, 0.5),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, 0.5),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, -0.5, 0.5),
        normal: Vector3::new(0.0, 0.0, 1.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

const BACK: [StaticVertex; 4] = [
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, -0.5),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, -0.5),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, -0.5),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, -0.5, -0.5),
        normal: Vector3::new(0.0, 0.0, -1.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

const TOP: [StaticVertex; 4] = [
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, 0.5),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, -0.5),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, -0.5),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, 0.5),
        normal: Vector3::new(0.0, 1.0, 0.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

const BOTTOM: [StaticVertex; 4] = [
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, 0.5),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, -0.5),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, -0.5, -0.5),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, -0.5, 0.5),
        normal: Vector3::new(0.0, -1.0, 0.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

const RIGHT: [StaticVertex; 4] = [
    // Right
    StaticVertex {
        position: Vector3::new(0.5, -0.5, -0.5),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, -0.5),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, 0.5, 0.5),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(0.5, -0.5, 0.5),
        normal: Vector3::new(1.0, 0.0, 0.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

const LEFT: [StaticVertex; 4] = [
    // Left
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, -0.5),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coord: Vector2::new(0.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, -0.5),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coord: Vector2::new(0.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, 0.5, 0.5),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coord: Vector2::new(1.0, 1.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
    StaticVertex {
        position: Vector3::new(-0.5, -0.5, 0.5),
        normal: Vector3::new(-1.0, 0.0, 0.0),
        tex_coord: Vector2::new(1.0, 0.0),
        tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
    },
];

#[derive(Debug,PartialEq)]
pub enum Direction {
    Front,
    Back,
    Top,
    Bottom,
    Right,
    Left,
}

impl Direction {

    pub fn vector(&self) -> (i32, i32, i32) {
        match self {
            Direction::Front => (0, 0, 1),
            Direction::Back => (0, 0, -1),
            Direction::Top => (0, 1, 0),
            Direction::Bottom => (0, -1, 0),
            Direction::Right => (1, 0, 0),
            Direction::Left => (-1, 0, 0),
        }
    }

    pub fn triangles(&self) -> [TriangleDefinition; 2] {
        match self {
            Direction::Front | Direction::Top | Direction::Left => CLOCK_WISE_TRIANGLE,
            Direction::Back | Direction::Bottom | Direction::Right => COUNTER_CLOCK_WISE_TRIANGLE,
        }
    }

    pub fn verticles(&self) -> Vec<StaticVertex>
    {
        match self
        {
            Direction::Front => FRONT.to_vec(),
            Direction::Back => BACK.to_vec(),
            Direction::Top => TOP.to_vec(),
            Direction::Bottom => BOTTOM.to_vec(),
            Direction::Right => RIGHT.to_vec(),
            Direction::Left => LEFT.to_vec(),
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [
            Direction::Front,
            Direction::Back,
            Direction::Top,
            Direction::Bottom,
            Direction::Right,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}
