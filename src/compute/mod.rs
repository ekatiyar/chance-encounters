pub mod spacetime;
pub mod rtree;

use spacetime::{SpaceTimePoint, SpaceTimeRecord};
use rstar::RTree;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

struct NearestNeighborPoint<I> where I: Iterator {
    point: SpaceTimePoint,
    iter: I,
    distance_2: f64
}

impl<I: Iterator> Eq for NearestNeighborPoint<I> {}

impl<I: Iterator> PartialEq for NearestNeighborPoint<I> {
    fn eq(&self, other: &Self) -> bool {
        self.point.eq(&other.point) && self.distance_2.eq(&other.distance_2)
    }
}

impl<I: Iterator> Ord for NearestNeighborPoint<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance_2.total_cmp(&self.distance_2)
    }
}

impl<I: Iterator> PartialOrd for NearestNeighborPoint<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance_2.partial_cmp(&self.distance_2)
    }
}

pub struct ChanceEncounter { // roll credits
    pub point1: SpaceTimePoint,
    pub point2: SpaceTimePoint,
    pub distance_km: f64,
    pub distance_s: f64
}

pub fn get_nearest_points(points1: SpaceTimeRecord, points2: SpaceTimeRecord) -> Vec<ChanceEncounter> {
    let point1_rtree = RTree::bulk_load(points1.points);

    let mut point2_btree = BinaryHeap::with_capacity(points2.points.len());
    for point in points2.points {
        let mut iter = point1_rtree.nearest_neighbor_iter_with_distance_2(&[point.latitude, point.longitude, point.start_time.timestamp() as f64, point.end_time.timestamp() as f64]).peekable();
        let current_nearest = iter.peek();
        match current_nearest {
            Some(&(_, distance_2)) => {
                let neighbor_point = NearestNeighborPoint {point, iter, distance_2};
                point2_btree.push(neighbor_point);
            },
            None => continue
        }
    }

    let mut seen_points1 = HashSet::new();
    let mut chance_encounters = Vec::with_capacity(10);
    while chance_encounters.len() < 10 && !point2_btree.is_empty() {
        let mut closest_pair = point2_btree.pop().expect("Unable to pop from heap");
        let point1 = closest_pair.iter.peek();
        match point1 {
            Some(&(point, _)) => {
                if seen_points1.insert(point.clone()) {
                    chance_encounters.push(ChanceEncounter {point1: point.clone(), point2: closest_pair.point, distance_km: 0, distance_s: 0})
                }
            },
            None => continue
        }
    }

    return chance_encounters
}