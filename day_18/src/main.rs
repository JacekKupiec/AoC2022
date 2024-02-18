use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x, y, z
        }
    }

    fn is_not_in_cuboid(&self, dim_x: i32, dim_y: i32, dim_z: i32) -> bool {
        self.x < 0
        || self.y < 0
        || self.z < 0
        || self.x >= dim_x
        || self.y >= dim_y
        || self.z >= dim_z
    }
}

impl ops::Add<(i32, i32, i32)> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: (i32, i32, i32)) -> Self::Output {
        return Point3D::new(self.x + rhs.0, self.y + rhs.1, self.z + rhs.2);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DropletType {
    Air, Steam, Rock
}

const DIRECTIONS_TO_MOVE: [(i32, i32, i32); 6] = [(1, 0, 0), (0, 1, 0), (0, 0, 1), (-1, 0, 0), (0, -1, 0), (0, 0, -1)];

#[inline(always)]
fn get_index(dim_y: i32, dim_z: i32, point: &Point3D) -> usize {
    return (point.x*dim_y*dim_z + point.y*dim_z + point.z) as usize;
}

fn dfs_faster(graph: &mut Vec<DropletType>, point: Point3D, dim_x: i32, dim_y: i32, dim_z: i32) {
    let mut stack = vec![point];

    dfs_iterative(graph, &mut stack, dim_x, dim_y, dim_z);
}

fn dfs_iterative(graph: &mut Vec<DropletType>, stack: &mut Vec<Point3D>, dim_x: i32, dim_y: i32, dim_z: i32) {
    while let Some(point) = stack.pop() {
        let index = get_index(dim_y, dim_z, &point);

        if graph[index] != DropletType::Air {
            continue;
        }

        graph[index] = DropletType::Steam;


        for direction in DIRECTIONS_TO_MOVE {
            let new_point = point + direction;

            if new_point.is_not_in_cuboid(dim_x, dim_y, dim_z) {
                continue;
            }

            stack.push(new_point);
        }
    }
}

#[allow(dead_code)]
fn dfs_recursive(graph: &mut Vec<DropletType>, point: Point3D, dim_x: i32, dim_y: i32, dim_z: i32) {
    let index = get_index(dim_y, dim_z, &point);

    if graph[index] != DropletType::Air {
        return;
    }

    graph[index] = DropletType::Steam;

    for direction in DIRECTIONS_TO_MOVE {
        let new_point = point + direction;

        if new_point.is_not_in_cuboid(dim_x, dim_y, dim_z) {
            continue;
        }

        dfs_recursive(graph, new_point, dim_x, dim_y, dim_z);
    }
}

fn main() {
    let file = File::open("input.txt").expect("The file must exist");
    let reader = BufReader::new(file);

    let mut points: Vec<Point3D> = reader.lines().map(|l| {
        let line = l.expect("There must be a valid line here!");
        let parts: Vec<i32> = line.trim_end().split(',').map(|e| e.parse().unwrap()).collect();

        return Point3D::new(parts[0], parts[1], parts[2]);
    })
        .collect();

    let init_min = (points[0].x, points[0].y, points[0].z);
    let (min_x, min_y, min_z) = points.iter()
        .skip(1)
        .fold(init_min, |(x, y, z), point| {
            return (min(x, point.x), min(y, point.y), min(z, point.z));
        });

    let init_max = init_min;
    let (max_x, max_y, max_z) = points.iter()
        .skip(1)
        .fold(init_max, |(x, y, z), point| {
            return (max(x, point.x), max(y, point.y), max(z, point.z));
        });

    // z_dim - number of columns
    // y_dim - number of rows
    // x_dim - number of matrices
    let dim_x = max_x - min_x + 1;
    let dim_y = max_y - min_y + 1;
    let dim_z = max_z - min_z + 1;
    let cube_size = (dim_x * dim_y * dim_z) as usize;
    let mut cuboid = vec![DropletType::Air; cube_size];

    for point in &mut points {
        point.x -= min_x;
        point.y -= min_y;
        point.z -= min_z;

        let index = get_index(dim_y, dim_z, point);
        cuboid[index] = DropletType::Rock;
    }

    /*
    I think I should check all not rock points in the cuboid cage I created because the droplet can have
    2 independent places caves where steam can reach or e.g. what if min point are the coordinates of the rock?
    Fortunately it is not a case but to guarantee that I find all places where steam can get I should call DFS
    on each not rock point of 6 faces on encompassing/wrapping-around cuboid: (0, y, z), (dim_x, y, z),
    (x, 0, z), (x, dim_y, z), (x, y, 0), (x, y, dim_z)
     */
    let x_face_left = (0..dim_y).flat_map(|y| (0..dim_z).map(move |z| Point3D::new(0, y, z)));
    let x_face_right = (0..dim_y).flat_map(|y| (0..dim_z).map(move |z| Point3D::new(dim_x - 1, y, z)));
    let y_face_left = (0..dim_x).flat_map(|x| (0..dim_z).map(move |z| Point3D::new(x, 0, z)));
    let y_face_right = (0..dim_x).flat_map(|x| (0..dim_z).map(move |z| Point3D::new(x, dim_y - 1, z)));
    let z_face_left = (0..dim_x).flat_map(|x| (0..dim_y).map(move |y| Point3D::new(x, y, 0)));
    let z_face_right = (0..dim_x).flat_map(|x| (0..dim_y).map(move |y| Point3D::new(x, y, dim_z - 1)));
    let all_faces = x_face_left
        .chain(x_face_right)
        .chain(y_face_left)
        .chain(y_face_right)
        .chain(z_face_left)
        .chain(z_face_right);

    for start_point in all_faces {
        dfs_faster(&mut cuboid, start_point, dim_x, dim_y, dim_z);
    }

    let face_of_cubes_touch_air_or_rock = points.iter()
        .flat_map(|point| {
            DIRECTIONS_TO_MOVE.map(|direction| *point + direction)
        })
        .filter(|point| {
            if point.is_not_in_cuboid(dim_x, dim_y, dim_z) {
                return false;
            }

            let index = get_index(dim_y, dim_z, point);

            return cuboid[index] != DropletType::Steam;
        })
        .count();

    let all_faces = points.len()*6;
    println!("{}", all_faces - face_of_cubes_touch_air_or_rock);
}