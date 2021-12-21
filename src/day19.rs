use eyre::ContextCompat;
use itertools::{iproduct, Itertools};
use nalgebra::{point, Matrix4, Point3, Transform3, Vector3};
use std::collections::{HashMap, HashSet, VecDeque};

type Point = Point3<f64>;

#[derive(Debug, Default, Clone)]
struct Scanner {
    points: Vec<Point>,
    unique_distances: Vec<HashSet<u32>>,
}

impl Scanner {
    fn find_transform(&self, other: &Scanner) -> Option<Transform3<f64>> {
        let overlaps = self.overlaps(other).take(4).collect::<Vec<_>>();
        if overlaps.len() < 4 {
            return None;
        }

        let self_columns = overlaps
            .iter()
            .map(|&(i, _)| self.points[i].to_homogeneous())
            .collect::<Vec<_>>();
        let mut self_matrix = Matrix4::from_columns(&self_columns);
        if !self_matrix.try_inverse_mut() {
            return None;
        }

        let other_columns = overlaps
            .iter()
            .map(|&(_, j)| other.points[j].to_homogeneous())
            .collect::<Vec<_>>();
        let other_matrix = Matrix4::from_columns(&other_columns);

        let transform_matrix = other_matrix * self_matrix;
        let transform = Transform3::from_matrix_unchecked(transform_matrix);
        Some(transform)
    }

    fn overlaps<'a>(&'a self, other: &'a Scanner) -> impl Iterator<Item = (usize, usize)> + 'a {
        iproduct!(
            self.unique_distances.iter().enumerate(),
            other.unique_distances.iter().enumerate()
        )
        .filter_map(
            |((self_index, self_distances), (other_index, other_distances))| {
                let overlap_distances = self_distances & other_distances;
                if overlap_distances.len() >= 11 {
                    Some((self_index, other_index))
                } else {
                    None
                }
            },
        )
    }
}

impl FromIterator<Point> for Scanner {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let points = iter.into_iter().collect::<Vec<Point>>();
        let distances = points.iter().enumerate().tuple_combinations().fold(
            HashMap::<(usize, u32), HashSet<usize>>::new(),
            |mut acc, ((left_index, left), (right_index, right))| {
                let distance = (left - right).norm_squared() as u32;
                acc.entry((left_index, distance))
                    .or_default()
                    .insert(right_index);
                acc.entry((right_index, distance))
                    .or_default()
                    .insert(left_index);
                acc
            },
        );

        let unique_distances = distances.iter().fold(
            vec![HashSet::new(); points.len()],
            |mut acc, (&(i, d), v)| {
                if v.len() == 1 {
                    acc[i].insert(d);
                }

                acc
            },
        );

        Self {
            points,
            unique_distances,
        }
    }
}

struct Input {
    scanners: Vec<Scanner>,
    transforms: Vec<Transform3<f64>>,
}

#[aoc_generator(day19)]
fn generator(input: &str) -> eyre::Result<Input> {
    let scanners: Vec<Scanner> = input
        .lines()
        .try_fold::<_, _, eyre::Result<Vec<Vec<Point>>>>(vec![], |mut acc, line| {
            if line.is_empty() {
                return Ok(acc);
            }

            if line.starts_with("---") {
                acc.push(vec![]);
                return Ok(acc);
            }

            let point = Point::from(Vector3::from_iterator(
                line.split(',').filter_map(|part| part.parse().ok()),
            ));

            let last = acc.last_mut().context("unable to get last")?;
            last.push(point);

            Ok(acc)
        })
        .map(|scanners| {
            scanners
                .into_iter()
                .map(|points| points.into_iter().collect())
                .collect()
        })?;

    let mut transforms = HashMap::new();
    transforms.insert(0, Transform3::identity());

    let mut ignore = HashSet::new();

    let mut search = (1..scanners.len()).collect::<VecDeque<usize>>();
    while let Some(i) = search.pop_front() {
        let known_scanners = transforms.keys();
        for &j in known_scanners {
            if ignore.contains(&(i, j)) {
                continue;
            }

            if let Some(transform) = scanners[i].find_transform(&scanners[j]) {
                let other_transform = transforms[&j];
                transforms.insert(i, other_transform * transform);
                break;
            } else {
                ignore.insert((i, j));
            }
        }

        if !transforms.contains_key(&i) {
            search.push_back(i);
        }
    }

    let transforms = transforms.into_iter().fold(
        vec![Transform3::identity(); scanners.len()],
        |mut acc, (index, transform)| {
            acc[index] = transform;
            acc
        },
    );

    Ok(Input {
        scanners,
        transforms,
    })
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let beacons = input
        .scanners
        .iter()
        .enumerate()
        .flat_map(|(i, scanner)| {
            let transform = input.transforms[i];
            scanner.points.iter().copied().map(move |local_point| {
                let global_point = transform * local_point;
                Vector3::from_iterator(global_point.coords.iter().map(|n| n.round() as i32)).into()
            })
        })
        .collect::<HashSet<Point3<i32>>>();

    beacons.len()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> Option<i32> {
    (0..input.scanners.len())
        .map(|i| {
            let origin = input.transforms[i] * point![0., 0., 0.];
            Point3::from(Vector3::<i32>::from_iterator(
                origin.coords.iter().map(|x| x.round() as i32),
            ))
        })
        .tuple_combinations()
        .map(|(left, right)| (left - right).abs().sum())
        .max()
}
