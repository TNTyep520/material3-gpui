// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/Cubic.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/RoundedPolygon.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/Morph.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/PolygonMeasure.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/FeatureMapping.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/FloatMapping.kt
// 参考 https://github.com/androidx/androidx/blob/androidx-main/graphics/graphics-shapes/src/commonMain/kotlin/androidx/graphics/shapes/Utils.kt

use std::f32::consts::PI;

const DISTANCE_EPSILON: f32 = 1.0e-4;
const ANGLE_EPSILON: f32 = 1.0e-6;

fn positive_modulo(num: f32, modulo: f32) -> f32 {
    (num % modulo + modulo) % modulo
}

fn progress_distance(p1: f32, p2: f32) -> f32 {
    (p1 - p2).abs().min(1.0 - (p1 - p2).abs())
}

fn progress_in_range(progress: f32, progress_from: f32, progress_to: f32) -> bool {
    if progress_to >= progress_from {
        progress >= progress_from && progress <= progress_to
    } else {
        progress >= progress_from || progress <= progress_to
    }
}

fn convex(previous: (f32, f32), current: (f32, f32), next: (f32, f32)) -> bool {
    let ax = current.0 - previous.0;
    let ay = current.1 - previous.1;
    let bx = next.0 - current.0;
    let by = next.1 - current.1;
    ax * by - ay * bx > 0.0
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cubic {
    pub anchor0_x: f32,
    pub anchor0_y: f32,
    pub control0_x: f32,
    pub control0_y: f32,
    pub control1_x: f32,
    pub control1_y: f32,
    pub anchor1_x: f32,
    pub anchor1_y: f32,
}

impl Cubic {
    pub const fn new(points: [f32; 8]) -> Self {
        Self {
            anchor0_x: points[0],
            anchor0_y: points[1],
            control0_x: points[2],
            control0_y: points[3],
            control1_x: points[4],
            control1_y: points[5],
            anchor1_x: points[6],
            anchor1_y: points[7],
        }
    }

    pub fn straight_line(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self::new([
            x0,
            y0,
            x0 + (x1 - x0) / 3.0,
            y0 + (y1 - y0) / 3.0,
            x0 + (x1 - x0) * 2.0 / 3.0,
            y0 + (y1 - y0) * 2.0 / 3.0,
            x1,
            y1,
        ])
    }

    pub fn circular_arc(center: (f32, f32), p0: (f32, f32), p1: (f32, f32)) -> Self {
        let p0d = (p0.0 - center.0, p0.1 - center.1);
        let p1d = (p1.0 - center.0, p1.1 - center.1);
        let p0d_len = (p0d.0 * p0d.0 + p0d.1 * p0d.1).sqrt();
        let p1d_len = (p1d.0 * p1d.0 + p1d.1 * p1d.1).sqrt();
        let (p0dx, p0dy) = (p0d.0 / p0d_len, p0d.1 / p0d_len);
        let (p1dx, p1dy) = (p1d.0 / p1d_len, p1d.1 / p1d_len);
        let rotated_p0 = (-p0dy, p0dx);
        let rotated_p1 = (-p1dy, p1dx);
        let clockwise = rotated_p0.0 * (p1.0 - center.0) + rotated_p0.1 * (p1.1 - center.1) >= 0.0;
        let cosa = p0dx * p1dx + p0dy * p1dy;
        if cosa > 0.999 {
            return Self::straight_line(p0.0, p0.1, p1.0, p1.1);
        }
        let r = p0d_len;
        let k = r * 4.0 / 3.0
            * (((2.0 * (1.0 - cosa)).sqrt() - (1.0 - cosa * cosa).sqrt()) / (1.0 - cosa))
            * if clockwise { 1.0 } else { -1.0 };
        Self::new([
            p0.0,
            p0.1,
            p0.0 + rotated_p0.0 * k,
            p0.1 + rotated_p0.1 * k,
            p1.0 - rotated_p1.0 * k,
            p1.1 - rotated_p1.1 * k,
            p1.0,
            p1.1,
        ])
    }

    fn zero_length(&self) -> bool {
        self.anchor0_x == self.anchor1_x && self.anchor0_y == self.anchor1_y
    }

    pub fn reverse(&self) -> Self {
        Self::new([
            self.anchor1_x,
            self.anchor1_y,
            self.control1_x,
            self.control1_y,
            self.control0_x,
            self.control0_y,
            self.anchor0_x,
            self.anchor0_y,
        ])
    }

    pub fn point_on_curve(&self, t: f32) -> (f32, f32) {
        let u = 1.0 - t;
        let a = u * u * u;
        let b = 3.0 * u * u * t;
        let c = 3.0 * u * t * t;
        let d = t * t * t;
        (
            self.anchor0_x * a + self.control0_x * b + self.control1_x * c + self.anchor1_x * d,
            self.anchor0_y * a + self.control0_y * b + self.control1_y * c + self.anchor1_y * d,
        )
    }

    pub fn split(&self, t: f32) -> (Self, Self) {
        let u = 1.0 - t;
        let point_on_curve = self.point_on_curve(t);
        let first = Self::new([
            self.anchor0_x,
            self.anchor0_y,
            self.anchor0_x * u + self.control0_x * t,
            self.anchor0_y * u + self.control0_y * t,
            self.anchor0_x * (u * u) + self.control0_x * (2.0 * u * t) + self.control1_x * (t * t),
            self.anchor0_y * (u * u) + self.control0_y * (2.0 * u * t) + self.control1_y * (t * t),
            point_on_curve.0,
            point_on_curve.1,
        ]);
        let second = Self::new([
            point_on_curve.0,
            point_on_curve.1,
            self.control0_x * (u * u) + self.control1_x * (2.0 * u * t) + self.anchor1_x * (t * t),
            self.control0_y * (u * u) + self.control1_y * (2.0 * u * t) + self.anchor1_y * (t * t),
            self.control1_x * u + self.anchor1_x * t,
            self.control1_y * u + self.anchor1_y * t,
            self.anchor1_x,
            self.anchor1_y,
        ]);
        (first, second)
    }

    pub fn transform(&self, f: impl Fn(f32, f32) -> (f32, f32)) -> Self {
        let (a0x, a0y) = f(self.anchor0_x, self.anchor0_y);
        let (c0x, c0y) = f(self.control0_x, self.control0_y);
        let (c1x, c1y) = f(self.control1_x, self.control1_y);
        let (a1x, a1y) = f(self.anchor1_x, self.anchor1_y);
        Self::new([a0x, a0y, c0x, c0y, c1x, c1y, a1x, a1y])
    }

    pub fn calculate_bounds(&self, approximate: bool) -> (f32, f32, f32, f32) {
        if self.zero_length() {
            return (
                self.anchor0_x,
                self.anchor0_y,
                self.anchor0_x,
                self.anchor0_y,
            );
        }
        let mut min_x = self.anchor0_x.min(self.anchor1_x);
        let mut min_y = self.anchor0_y.min(self.anchor1_y);
        let mut max_x = self.anchor0_x.max(self.anchor1_x);
        let mut max_y = self.anchor0_y.max(self.anchor1_y);
        if approximate {
            min_x = min_x.min(self.control0_x).min(self.control1_x);
            min_y = min_y.min(self.control0_y).min(self.control1_y);
            max_x = max_x.max(self.control0_x).max(self.control1_x);
            max_y = max_y.max(self.control0_y).max(self.control1_y);
            return (min_x, min_y, max_x, max_y);
        }

        let xa = -self.anchor0_x + 3.0 * self.control0_x - 3.0 * self.control1_x + self.anchor1_x;
        let xb = 2.0 * self.anchor0_x - 4.0 * self.control0_x + 2.0 * self.control1_x;
        let xc = -self.anchor0_x + self.control0_x;
        let mut update_x = |x: f32| {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
        };
        if xa.abs() < 1.0e-5 {
            if xb != 0.0 {
                let t = 2.0 * xc / (-2.0 * xb);
                if (0.0..=1.0).contains(&t) {
                    update_x(self.point_on_curve(t).0);
                }
            }
        } else {
            let xs = xb * xb - 4.0 * xa * xc;
            if xs >= 0.0 {
                let t1 = (-xb + xs.sqrt()) / (2.0 * xa);
                if (0.0..=1.0).contains(&t1) {
                    update_x(self.point_on_curve(t1).0);
                }
                let t2 = (-xb - xs.sqrt()) / (2.0 * xa);
                if (0.0..=1.0).contains(&t2) {
                    update_x(self.point_on_curve(t2).0);
                }
            }
        }

        let ya = -self.anchor0_y + 3.0 * self.control0_y - 3.0 * self.control1_y + self.anchor1_y;
        let yb = 2.0 * self.anchor0_y - 4.0 * self.control0_y + 2.0 * self.control1_y;
        let yc = -self.anchor0_y + self.control0_y;
        let mut update_y = |y: f32| {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        };
        if ya.abs() < 1.0e-5 {
            if yb != 0.0 {
                let t = 2.0 * yc / (-2.0 * yb);
                if (0.0..=1.0).contains(&t) {
                    update_y(self.point_on_curve(t).1);
                }
            }
        } else {
            let ys = yb * yb - 4.0 * ya * yc;
            if ys >= 0.0 {
                let t1 = (-yb + ys.sqrt()) / (2.0 * ya);
                if (0.0..=1.0).contains(&t1) {
                    update_y(self.point_on_curve(t1).1);
                }
                let t2 = (-yb - ys.sqrt()) / (2.0 * ya);
                if (0.0..=1.0).contains(&t2) {
                    update_y(self.point_on_curve(t2).1);
                }
            }
        }

        (min_x, min_y, max_x, max_y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornerRounding {
    pub radius: f32,
    pub smoothing: f32,
}

impl CornerRounding {
    pub const UNROUNDED: Self = Self {
        radius: 0.0,
        smoothing: 0.0,
    };

    pub const fn new(radius: f32) -> Self {
        Self {
            radius,
            smoothing: 0.0,
        }
    }

    pub const fn with_smoothing(radius: f32, smoothing: f32) -> Self {
        Self { radius, smoothing }
    }
}

#[derive(Clone, Debug)]
pub enum Feature {
    Corner { cubics: Vec<Cubic>, convex: bool },
    Edge { cubics: Vec<Cubic> },
}

impl Feature {
    pub fn cubics(&self) -> &[Cubic] {
        match self {
            Feature::Corner { cubics, .. } | Feature::Edge { cubics } => cubics,
        }
    }

    fn is_corner(&self) -> bool {
        matches!(self, Feature::Corner { .. })
    }

    fn convex(&self) -> bool {
        match self {
            Feature::Corner { convex, .. } => *convex,
            _ => false,
        }
    }

    fn transformed(&self, f: impl Fn(f32, f32) -> (f32, f32)) -> Feature {
        let cubics = self.cubics().iter().map(|c| c.transform(&f)).collect();
        match self {
            Feature::Corner { convex, .. } => Feature::Corner {
                cubics,
                convex: *convex,
            },
            Feature::Edge { .. } => Feature::Edge { cubics },
        }
    }
}

fn calculate_center(vertices: &[(f32, f32)]) -> (f32, f32) {
    let n = vertices.len() as f32;
    let mut x = 0.0;
    let mut y = 0.0;
    for vertex in vertices {
        x += vertex.0;
        y += vertex.1;
    }
    (x / n, y / n)
}

#[derive(Clone, Debug)]
pub struct RoundedPolygon {
    features: Vec<Feature>,
    center: (f32, f32),
}

impl RoundedPolygon {
    pub fn from_vertices(
        vertices: &[(f32, f32)],
        rounding: CornerRounding,
        per_vertex_rounding: Option<&[CornerRounding]>,
        center: Option<(f32, f32)>,
    ) -> Self {
        assert!(
            vertices.len() >= 3,
            "Polygons must have at least 3 vertices"
        );
        let n = vertices.len();
        let mut rounded_corners = Vec::with_capacity(n);
        for i in 0..n {
            let rounding = per_vertex_rounding.map(|list| list[i]).unwrap_or(rounding);
            rounded_corners.push(RoundedCorner::new(
                vertices[(i + n - 1) % n],
                vertices[i],
                vertices[(i + 1) % n],
                Some(rounding),
            ));
        }

        let cut_adjusts: Vec<(f32, f32)> = (0..n)
            .map(|ix| {
                let expected_round_cut = rounded_corners[ix].expected_round_cut
                    + rounded_corners[(ix + 1) % n].expected_round_cut;
                let expected_cut = rounded_corners[ix].expected_cut()
                    + rounded_corners[(ix + 1) % n].expected_cut();
                let side_size = distance(
                    vertices[ix].0 - vertices[(ix + 1) % n].0,
                    vertices[ix].1 - vertices[(ix + 1) % n].1,
                );
                if expected_round_cut > side_size {
                    (side_size / expected_round_cut, 0.0)
                } else if expected_cut > side_size {
                    (
                        1.0,
                        (side_size - expected_round_cut) / (expected_cut - expected_round_cut),
                    )
                } else {
                    (1.0, 1.0)
                }
            })
            .collect();

        let mut corner_cubics: Vec<Vec<Cubic>> = Vec::with_capacity(n);
        for i in 0..n {
            let mut allowed_cuts = [0.0_f32; 2];
            for delta in 0..2 {
                let (round_cut_ratio, cut_ratio) = cut_adjusts[(i + n - 1 + delta) % n];
                allowed_cuts[delta] = rounded_corners[i].expected_round_cut * round_cut_ratio
                    + (rounded_corners[i].expected_cut() - rounded_corners[i].expected_round_cut)
                        * cut_ratio;
            }
            corner_cubics.push(rounded_corners[i].get_cubics(allowed_cuts[0], allowed_cuts[1]));
        }

        let mut features = Vec::with_capacity(n * 2);
        for i in 0..n {
            let prev_vertex = vertices[(i + n - 1) % n];
            let curr_vertex = vertices[i];
            let next_vertex = vertices[(i + 1) % n];
            features.push(Feature::Corner {
                cubics: corner_cubics[i].clone(),
                convex: convex(prev_vertex, curr_vertex, next_vertex),
            });
            let last = corner_cubics[i].last().expect("corner has cubics");
            let next_first = corner_cubics[(i + 1) % n]
                .first()
                .expect("corner has cubics");
            features.push(Feature::Edge {
                cubics: vec![Cubic::straight_line(
                    last.anchor1_x,
                    last.anchor1_y,
                    next_first.anchor0_x,
                    next_first.anchor0_y,
                )],
            });
        }

        let center = center.unwrap_or_else(|| calculate_center(vertices));
        Self { features, center }
    }

    pub fn from_features(features: Vec<Feature>, center: Option<(f32, f32)>) -> Self {
        assert!(
            features.len() >= 2,
            "Polygons must have at least 2 features"
        );
        let mut vertices = Vec::with_capacity(features.len() * 2);
        for feature in &features {
            for cubic in feature.cubics() {
                vertices.push((cubic.anchor0_x, cubic.anchor0_y));
            }
        }
        let center = center.unwrap_or_else(|| calculate_center(&vertices));
        Self { features, center }
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    pub fn center(&self) -> (f32, f32) {
        self.center
    }

    pub fn transformed(&self, f: impl Fn(f32, f32) -> (f32, f32)) -> Self {
        Self {
            features: self
                .features
                .iter()
                .map(|feature| feature.transformed(&f))
                .collect(),
            center: f(self.center.0, self.center.1),
        }
    }

    pub fn normalized(&self) -> Self {
        let (min_x, min_y, max_x, max_y) = self.calculate_bounds(false);
        let width = max_x - min_x;
        let height = max_y - min_y;
        let side = width.max(height);
        let offset_x = (side - width) / 2.0 - min_x;
        let offset_y = (side - height) / 2.0 - min_y;
        self.transformed(move |x, y| ((x + offset_x) / side, (y + offset_y) / side))
    }

    pub fn calculate_bounds(&self, approximate: bool) -> (f32, f32, f32, f32) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        for feature in &self.features {
            for cubic in feature.cubics() {
                let (bx0, by0, bx1, by1) = cubic.calculate_bounds(approximate);
                min_x = min_x.min(bx0);
                min_y = min_y.min(by0);
                max_x = max_x.max(bx1);
                max_y = max_y.max(by1);
            }
        }
        (min_x, min_y, max_x, max_y)
    }

    pub fn calculate_max_bounds(&self) -> (f32, f32, f32, f32) {
        let mut max_dist_squared = 0.0_f32;
        for feature in &self.features {
            for cubic in feature.cubics() {
                let anchor_dist = distance_squared(
                    cubic.anchor0_x - self.center.0,
                    cubic.anchor0_y - self.center.1,
                );
                let middle = cubic.point_on_curve(0.5);
                let middle_dist =
                    distance_squared(middle.0 - self.center.0, middle.1 - self.center.1);
                max_dist_squared = max_dist_squared.max(anchor_dist.max(middle_dist));
            }
        }
        let distance = max_dist_squared.sqrt();
        (
            self.center.0 - distance,
            self.center.1 - distance,
            self.center.0 + distance,
            self.center.1 + distance,
        )
    }
}

fn distance(x: f32, y: f32) -> f32 {
    (x * x + y * y).sqrt()
}

fn radial_to_cartesian(radius: f32, angle: f32) -> (f32, f32) {
    (radius * angle.cos(), radius * angle.sin())
}

fn distance_squared(x: f32, y: f32) -> f32 {
    x * x + y * y
}

struct RoundedCorner {
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    d1: (f32, f32),
    d2: (f32, f32),
    corner_radius: f32,
    smoothing: f32,
    expected_round_cut: f32,
}

impl RoundedCorner {
    fn new(
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        rounding: Option<CornerRounding>,
    ) -> Self {
        let v01 = (p0.0 - p1.0, p0.1 - p1.1);
        let v21 = (p2.0 - p1.0, p2.1 - p1.1);
        let d01 = distance(v01.0, v01.1);
        let d21 = distance(v21.0, v21.1);
        if d01 <= 0.0 || d21 <= 0.0 {
            return Self {
                p0,
                p1,
                p2,
                d1: (0.0, 0.0),
                d2: (0.0, 0.0),
                corner_radius: 0.0,
                smoothing: 0.0,
                expected_round_cut: 0.0,
            };
        }
        let d1 = (v01.0 / d01, v01.1 / d01);
        let d2 = (v21.0 / d21, v21.1 / d21);
        let corner_radius = rounding.map(|r| r.radius).unwrap_or(0.0);
        let smoothing = rounding.map(|r| r.smoothing).unwrap_or(0.0);
        let cos_angle = d1.0 * d2.0 + d1.1 * d2.1;
        let sin_angle = (1.0 - cos_angle * cos_angle).sqrt();
        let expected_round_cut = if sin_angle > 1.0e-3 {
            corner_radius * (cos_angle + 1.0) / sin_angle
        } else {
            0.0
        };
        Self {
            p0,
            p1,
            p2,
            d1,
            d2,
            corner_radius,
            smoothing,
            expected_round_cut,
        }
    }

    fn expected_cut(&self) -> f32 {
        (1.0 + self.smoothing) * self.expected_round_cut
    }

    fn get_cubics(&mut self, allowed_cut0: f32, allowed_cut1: f32) -> Vec<Cubic> {
        let allowed_cut = allowed_cut0.min(allowed_cut1);
        if self.expected_round_cut < DISTANCE_EPSILON
            || allowed_cut < DISTANCE_EPSILON
            || self.corner_radius < DISTANCE_EPSILON
        {
            return vec![Cubic::straight_line(
                self.p1.0, self.p1.1, self.p1.0, self.p1.1,
            )];
        }
        let actual_round_cut = allowed_cut.min(self.expected_round_cut);
        let actual_smoothing0 = self.actual_smoothing_value(allowed_cut0);
        let actual_smoothing1 = self.actual_smoothing_value(allowed_cut1);
        let actual_r = self.corner_radius * actual_round_cut / self.expected_round_cut;
        let center_distance = (actual_r * actual_r + actual_round_cut * actual_round_cut).sqrt();
        let bis = (self.d1.0 + self.d2.0, self.d1.1 + self.d2.1);
        let bis_len = distance(bis.0, bis.1);
        let center = if bis_len > 0.0 {
            (
                self.p1.0 + bis.0 / bis_len * center_distance,
                self.p1.1 + bis.1 / bis_len * center_distance,
            )
        } else {
            self.p1
        };
        let circle_intersection0 = (
            self.p1.0 + self.d1.0 * actual_round_cut,
            self.p1.1 + self.d1.1 * actual_round_cut,
        );
        let circle_intersection2 = (
            self.p1.0 + self.d2.0 * actual_round_cut,
            self.p1.1 + self.d2.1 * actual_round_cut,
        );
        let flanking0 = Self::compute_flanking_curve(
            actual_round_cut,
            actual_smoothing0,
            self.p1,
            self.p0,
            circle_intersection0,
            circle_intersection2,
            center,
            actual_r,
        );
        let flanking2 = Self::compute_flanking_curve(
            actual_round_cut,
            actual_smoothing1,
            self.p1,
            self.p2,
            circle_intersection2,
            circle_intersection0,
            center,
            actual_r,
        )
        .reverse();
        vec![
            flanking0,
            Cubic::circular_arc(center, flanking0.anchor1(), flanking2.anchor0()),
            flanking2,
        ]
    }

    fn actual_smoothing_value(&self, allowed_cut: f32) -> f32 {
        if allowed_cut > self.expected_cut() {
            self.smoothing
        } else if allowed_cut > self.expected_round_cut {
            self.smoothing * (allowed_cut - self.expected_round_cut)
                / (self.expected_cut() - self.expected_round_cut)
        } else {
            0.0
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_flanking_curve(
        actual_round_cut: f32,
        actual_smoothing: f32,
        corner: (f32, f32),
        side_start: (f32, f32),
        circle_segment_intersection: (f32, f32),
        other_circle_segment_intersection: (f32, f32),
        circle_center: (f32, f32),
        actual_r: f32,
    ) -> Cubic {
        let side_direction = (side_start.0 - corner.0, side_start.1 - corner.1);
        let side_len = distance(side_direction.0, side_direction.1);
        let side_direction = if side_len > 0.0 {
            (side_direction.0 / side_len, side_direction.1 / side_len)
        } else {
            side_direction
        };
        let curve_start = (
            corner.0 + side_direction.0 * actual_round_cut * (1.0 + actual_smoothing),
            corner.1 + side_direction.1 * actual_round_cut * (1.0 + actual_smoothing),
        );
        let p = (
            circle_segment_intersection.0
                + (other_circle_segment_intersection.0 - circle_segment_intersection.0)
                    * actual_smoothing,
            circle_segment_intersection.1
                + (other_circle_segment_intersection.1 - circle_segment_intersection.1)
                    * actual_smoothing,
        );
        let p_dir = (p.0 - circle_center.0, p.1 - circle_center.1);
        let p_dir_len = distance(p_dir.0, p_dir.1);
        let curve_end = if p_dir_len > 0.0 {
            (
                circle_center.0 + p_dir.0 / p_dir_len * actual_r,
                circle_center.1 + p_dir.1 / p_dir_len * actual_r,
            )
        } else {
            p
        };
        let circle_tangent = (
            curve_end.1 - circle_center.1,
            -(curve_end.0 - circle_center.0),
        );
        let anchor_end = line_intersection(side_start, side_direction, curve_end, circle_tangent)
            .unwrap_or(circle_segment_intersection);
        let anchor_start = (
            (curve_start.0 + anchor_end.0 * 2.0) / 3.0,
            (curve_start.1 + anchor_end.1 * 2.0) / 3.0,
        );
        Cubic::new([
            curve_start.0,
            curve_start.1,
            anchor_start.0,
            anchor_start.1,
            anchor_end.0,
            anchor_end.1,
            curve_end.0,
            curve_end.1,
        ])
    }
}

fn line_intersection(
    p0: (f32, f32),
    d0: (f32, f32),
    p1: (f32, f32),
    d1: (f32, f32),
) -> Option<(f32, f32)> {
    let rotated_d1 = (-d1.1, d1.0);
    let den = d0.0 * rotated_d1.0 + d0.1 * rotated_d1.1;
    if den.abs() < DISTANCE_EPSILON {
        return None;
    }
    let num = (p1.0 - p0.0) * rotated_d1.0 + (p1.1 - p0.1) * rotated_d1.1;
    if den.abs() < DISTANCE_EPSILON * num.abs() {
        return None;
    }
    let k = num / den;
    Some((p0.0 + d0.0 * k, p0.1 + d0.1 * k))
}

impl Cubic {
    fn anchor1(&self) -> (f32, f32) {
        (self.anchor1_x, self.anchor1_y)
    }

    fn anchor0(&self) -> (f32, f32) {
        (self.anchor0_x, self.anchor0_y)
    }
}

#[derive(Clone, Copy, Debug)]
struct MeasuredCubic {
    cubic: Cubic,
    start_outline_progress: f32,
    end_outline_progress: f32,
}

struct MeasuredPolygon {
    measured_cubics: Vec<MeasuredCubic>,
    features: Vec<MeasuredFeature>,
}

#[derive(Clone)]
struct MeasuredFeature {
    progress: f32,
    representative: (f32, f32),
    convex: bool,
}

impl MeasuredPolygon {
    fn measure_polygon(polygon: &RoundedPolygon) -> Self {
        let mut cubics: Vec<Cubic> = Vec::new();
        let mut feature_to_cubic: Vec<(usize, bool)> = Vec::new();

        for feature in polygon.features() {
            let cubics_count = feature.cubics().len();
            for (cubic_index, cubic) in feature.cubics().iter().enumerate() {
                if feature.is_corner() && cubic_index == cubics_count / 2 {
                    feature_to_cubic.push((cubics.len(), feature.convex()));
                }
                cubics.push(*cubic);
            }
        }

        let mut measures = vec![0.0_f32; cubics.len() + 1];
        for (i, cubic) in cubics.iter().enumerate() {
            measures[i + 1] = measures[i] + LengthMeasurer::measure_cubic(cubic);
        }
        let total_measure = measures[cubics.len()];

        let mut outline_progress = vec![0.0_f32; cubics.len() + 1];
        for (i, measure) in measures.iter().enumerate() {
            outline_progress[i] = measure / total_measure;
        }

        let mut features = Vec::with_capacity(feature_to_cubic.len());
        for &(cubic_ix, convex) in &feature_to_cubic {
            let progress = positive_modulo(
                (outline_progress[cubic_ix] + outline_progress[cubic_ix + 1]) / 2.0,
                1.0,
            );
            let start_cubic = cubics[cubic_ix];
            let end_cubic = cubics[cubic_ix + 1];
            features.push(MeasuredFeature {
                progress,
                representative: (
                    (start_cubic.anchor0_x + end_cubic.anchor0_x) / 2.0,
                    (start_cubic.anchor0_y + end_cubic.anchor0_y) / 2.0,
                ),
                convex,
            });
        }

        let mut measured_cubics = Vec::with_capacity(cubics.len());
        let mut start = 0.0_f32;
        for i in 0..cubics.len() {
            if outline_progress[i + 1] - outline_progress[i] > DISTANCE_EPSILON {
                measured_cubics.push(MeasuredCubic {
                    cubic: cubics[i],
                    start_outline_progress: start,
                    end_outline_progress: outline_progress[i + 1],
                });
                start = outline_progress[i + 1];
            }
        }
        if let Some(last) = measured_cubics.last_mut() {
            last.end_outline_progress = 1.0;
        }

        Self {
            measured_cubics,
            features,
        }
    }

    fn cut_at_progress(
        measured: &MeasuredCubic,
        cut_outline_progress: f32,
    ) -> (MeasuredCubic, MeasuredCubic) {
        let bounded = cut_outline_progress.clamp(
            measured.start_outline_progress,
            measured.end_outline_progress,
        );
        let outline_progress_size = measured.end_outline_progress - measured.start_outline_progress;
        let progress_from_start = bounded - measured.start_outline_progress;
        let relative_progress = progress_from_start / outline_progress_size;
        let t = LengthMeasurer::find_cubic_cut_point(
            &measured.cubic,
            relative_progress * LengthMeasurer::measure_cubic(&measured.cubic),
        );
        let (c1, c2) = measured.cubic.split(t);
        (
            MeasuredCubic {
                cubic: c1,
                start_outline_progress: measured.start_outline_progress,
                end_outline_progress: bounded,
            },
            MeasuredCubic {
                cubic: c2,
                start_outline_progress: bounded,
                end_outline_progress: measured.end_outline_progress,
            },
        )
    }

    fn cut_and_shift(&self, cutting_point: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&cutting_point),
            "Cutting point is expected to be between 0 and 1"
        );
        if cutting_point < DISTANCE_EPSILON {
            return Self {
                measured_cubics: self.measured_cubics.clone(),
                features: self.features.clone(),
            };
        }

        let target_index = self
            .measured_cubics
            .iter()
            .position(|mc| {
                cutting_point >= mc.start_outline_progress
                    && cutting_point <= mc.end_outline_progress
            })
            .expect("cutting point inside outline");
        let target = &self.measured_cubics[target_index];
        let (b1, b2) = Self::cut_at_progress(target, cutting_point);

        let mut ret_cubics = vec![b2.cubic];
        for i in 1..self.measured_cubics.len() {
            ret_cubics
                .push(self.measured_cubics[(i + target_index) % self.measured_cubics.len()].cubic);
        }
        ret_cubics.push(b1.cubic);

        let count = self.measured_cubics.len();
        let mut ret_outline_progress = Vec::with_capacity(count + 2);
        for index in 0..count + 2 {
            ret_outline_progress.push(match index {
                0 => 0.0,
                i if i == count + 1 => 1.0,
                _ => {
                    let cubic_index = (target_index + index - 1) % count;
                    positive_modulo(
                        self.measured_cubics[cubic_index].end_outline_progress - cutting_point,
                        1.0,
                    )
                }
            });
        }

        let features = self
            .features
            .iter()
            .map(|f| MeasuredFeature {
                progress: positive_modulo(f.progress - cutting_point, 1.0),
                representative: f.representative,
                convex: f.convex,
            })
            .collect();

        let mut measured_cubics = Vec::with_capacity(ret_cubics.len());
        let mut start = 0.0_f32;
        for i in 0..ret_cubics.len() {
            if ret_outline_progress[i + 1] - ret_outline_progress[i] > DISTANCE_EPSILON {
                measured_cubics.push(MeasuredCubic {
                    cubic: ret_cubics[i],
                    start_outline_progress: start,
                    end_outline_progress: ret_outline_progress[i + 1],
                });
                start = ret_outline_progress[i + 1];
            }
        }
        if let Some(last) = measured_cubics.last_mut() {
            last.end_outline_progress = 1.0;
        }

        Self {
            measured_cubics,
            features,
        }
    }
}

struct LengthMeasurer;

impl LengthMeasurer {
    const SEGMENTS: usize = 3;

    fn measure_cubic(cubic: &Cubic) -> f32 {
        Self::closest_progress_to(cubic, f32::INFINITY).1
    }

    fn find_cubic_cut_point(cubic: &Cubic, m: f32) -> f32 {
        Self::closest_progress_to(cubic, m).0
    }

    fn closest_progress_to(cubic: &Cubic, threshold: f32) -> (f32, f32) {
        let mut total = 0.0_f32;
        let mut remainder = threshold;
        let mut prev = (cubic.anchor0_x, cubic.anchor0_y);
        let mut found = 0.0_f32;
        for i in 1..=Self::SEGMENTS {
            let progress = i as f32 / Self::SEGMENTS as f32;
            let curr = cubic.point_on_curve(progress);
            let dx = curr.0 - prev.0;
            let dy = curr.1 - prev.1;
            let distance = distance(dx, dy);
            if remainder < distance && total < threshold {
                found = (progress - 1.0 / Self::SEGMENTS as f32)
                    + (1.0 / Self::SEGMENTS as f32) * (remainder / distance);
                remainder = 0.0;
            } else {
                remainder -= distance;
            }
            total += distance;
            prev = curr;
        }
        if threshold.is_infinite() {
            return (0.0, total);
        }
        (found.clamp(0.0, 1.0), total)
    }
}

fn feature_dist_squared(f1: &MeasuredFeature, f2: &MeasuredFeature) -> f32 {
    if f1.convex != f2.convex {
        return f32::MAX;
    }
    distance_squared(
        f1.representative.0 - f2.representative.0,
        f1.representative.1 - f2.representative.1,
    )
}

fn linear_map(x_values: &[f32], y_values: &[f32], x: f32) -> f32 {
    assert!((0.0..=1.0).contains(&x), "Invalid progress: {x}");
    let n = x_values.len();
    let mut segment_start_index = 0;
    for i in 0..n {
        let next = (i + 1) % n;
        if progress_in_range(x, x_values[i], x_values[next]) {
            segment_start_index = i;
            break;
        }
    }
    let segment_end_index = (segment_start_index + 1) % n;
    let segment_size_x = positive_modulo(
        x_values[segment_end_index] - x_values[segment_start_index],
        1.0,
    );
    let segment_size_y = positive_modulo(
        y_values[segment_end_index] - y_values[segment_start_index],
        1.0,
    );
    let position_in_segment = if segment_size_x < 0.001 {
        0.5
    } else {
        positive_modulo(x - x_values[segment_start_index], 1.0) / segment_size_x
    };
    positive_modulo(
        y_values[segment_start_index] + segment_size_y * position_in_segment,
        1.0,
    )
}

struct DoubleMapper {
    source_values: Vec<f32>,
    target_values: Vec<f32>,
}

impl DoubleMapper {
    fn new(mappings: &[(f32, f32)]) -> Self {
        Self {
            source_values: mappings.iter().map(|m| m.0).collect(),
            target_values: mappings.iter().map(|m| m.1).collect(),
        }
    }

    fn map(&self, x: f32) -> f32 {
        linear_map(&self.source_values, &self.target_values, x)
    }

    fn map_back(&self, x: f32) -> f32 {
        linear_map(&self.target_values, &self.source_values, x)
    }
}

fn feature_mapper(features1: &[MeasuredFeature], features2: &[MeasuredFeature]) -> DoubleMapper {
    DoubleMapper::new(&do_mapping(features1, features2))
}

fn do_mapping(features1: &[MeasuredFeature], features2: &[MeasuredFeature]) -> Vec<(f32, f32)> {
    let mut distance_vertex_list: Vec<(f32, f32, f32, usize, usize)> = Vec::new();
    for (ix1, f1) in features1.iter().enumerate() {
        for (ix2, f2) in features2.iter().enumerate() {
            let d = feature_dist_squared(f1, f2);
            if d != f32::MAX {
                distance_vertex_list.push((d, f1.progress, f2.progress, ix1, ix2));
            }
        }
    }
    distance_vertex_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    if distance_vertex_list.is_empty() {
        return vec![(0.0, 0.0), (0.5, 0.5)];
    }
    if distance_vertex_list.len() == 1 {
        let (_, f1, f2, _, _) = distance_vertex_list[0];
        return vec![
            (f1, f2),
            (
                positive_modulo(f1 + 0.5, 1.0),
                positive_modulo(f2 + 0.5, 1.0),
            ),
        ];
    }

    let mut helper = MappingHelper::default();
    for (_, f1, f2, ix1, ix2) in &distance_vertex_list {
        helper.add_mapping(*f1, *f2, *ix1, *ix2);
    }
    helper.mapping
}

#[derive(Default)]
struct MappingHelper {
    mapping: Vec<(f32, f32)>,
    used_f1: Vec<usize>,
    used_f2: Vec<usize>,
}

impl MappingHelper {
    fn add_mapping(&mut self, f1: f32, f2: f32, ix1: usize, ix2: usize) {
        if self.used_f1.contains(&ix1) || self.used_f2.contains(&ix2) {
            return;
        }

        let insertion_index = self
            .mapping
            .binary_search_by(|(p, _)| p.partial_cmp(&f1).unwrap())
            .unwrap_or_else(|err| err);
        let n = self.mapping.len();

        if n >= 1 {
            let (before1, before2) = self.mapping[(insertion_index + n - 1) % n];
            let (after1, after2) = self.mapping[insertion_index % n];
            if progress_distance(f1, before1) < DISTANCE_EPSILON
                || progress_distance(f1, after1) < DISTANCE_EPSILON
                || progress_distance(f2, before2) < DISTANCE_EPSILON
                || progress_distance(f2, after2) < DISTANCE_EPSILON
            {
                return;
            }
            if n > 1 && !progress_in_range(f2, before2, after2) {
                return;
            }
        }

        self.mapping.insert(insertion_index, (f1, f2));
        self.used_f1.push(ix1);
        self.used_f2.push(ix2);
    }
}

pub struct Morph {
    morph_match: Vec<(Cubic, Cubic)>,
    start: RoundedPolygon,
    end: RoundedPolygon,
}

impl Morph {
    pub fn new(start: &RoundedPolygon, end: &RoundedPolygon) -> Self {
        Self {
            morph_match: Self::match_polygons(start, end),
            start: start.clone(),
            end: end.clone(),
        }
    }

    pub fn as_cubics(&self, progress: f32) -> Vec<Cubic> {
        let mut cubics = Vec::with_capacity(self.morph_match.len());
        let mut first: Option<Cubic> = None;
        let mut last: Option<Cubic> = None;
        for (start_cubic, end_cubic) in &self.morph_match {
            let mut points = [0.0_f32; 8];
            for (i, point) in points.iter_mut().enumerate() {
                let a = [
                    start_cubic.anchor0_x,
                    start_cubic.anchor0_y,
                    start_cubic.control0_x,
                    start_cubic.control0_y,
                    start_cubic.control1_x,
                    start_cubic.control1_y,
                    start_cubic.anchor1_x,
                    start_cubic.anchor1_y,
                ][i];
                let b = [
                    end_cubic.anchor0_x,
                    end_cubic.anchor0_y,
                    end_cubic.control0_x,
                    end_cubic.control0_y,
                    end_cubic.control1_x,
                    end_cubic.control1_y,
                    end_cubic.anchor1_x,
                    end_cubic.anchor1_y,
                ][i];
                *point = a + (b - a) * progress;
            }
            let cubic = Cubic::new(points);
            if first.is_none() {
                first = Some(cubic);
            }
            if let Some(prev) = last.take() {
                cubics.push(prev);
            }
            last = Some(cubic);
        }
        if let (Some(first), Some(last)) = (first, last) {
            cubics.push(Cubic::new([
                last.anchor0_x,
                last.anchor0_y,
                last.control0_x,
                last.control0_y,
                last.control1_x,
                last.control1_y,
                first.anchor0_x,
                first.anchor0_y,
            ]));
        }
        cubics
    }

    pub fn calculate_bounds(&self) -> (f32, f32, f32, f32) {
        let (s0, s1, s2, s3) = self.start.calculate_bounds(false);
        let (e0, e1, e2, e3) = self.end.calculate_bounds(false);
        (s0.min(e0), s1.min(e1), s2.max(e2), s3.max(e3))
    }

    pub fn calculate_max_bounds(&self) -> (f32, f32, f32, f32) {
        let (s0, s1, s2, s3) = self.start.calculate_max_bounds();
        let (e0, e1, e2, e3) = self.end.calculate_max_bounds();
        (s0.min(e0), s1.min(e1), s2.max(e2), s3.max(e3))
    }

    fn match_polygons(p1: &RoundedPolygon, p2: &RoundedPolygon) -> Vec<(Cubic, Cubic)> {
        let measured_polygon1 = MeasuredPolygon::measure_polygon(p1);
        let measured_polygon2 = MeasuredPolygon::measure_polygon(p2);

        let double_mapper =
            feature_mapper(&measured_polygon1.features, &measured_polygon2.features);

        let polygon2_cut_point = double_mapper.map(0.0);

        let bs1 = &measured_polygon1.measured_cubics;
        let shifted2 = measured_polygon2.cut_and_shift(polygon2_cut_point);
        let bs2 = &shifted2.measured_cubics;

        let mut ret: Vec<(Cubic, Cubic)> = Vec::new();
        let mut i1 = 0_usize;
        let mut i2 = 0_usize;
        let mut b1: Option<MeasuredCubic> = bs1.first().cloned();
        let mut b2: Option<MeasuredCubic> = bs2.first().cloned();
        if b1.is_some() {
            i1 += 1;
        }
        if b2.is_some() {
            i2 += 1;
        }

        while let (Some(current_b1), Some(current_b2)) = (b1, b2) {
            let b1a = if i1 == bs1.len() {
                1.0
            } else {
                current_b1.end_outline_progress
            };
            let b2a = if i2 == bs2.len() {
                1.0
            } else {
                double_mapper.map_back(positive_modulo(
                    current_b2.end_outline_progress + polygon2_cut_point,
                    1.0,
                ))
            };
            let min_b = b1a.min(b2a);

            let (seg1, new_b1) = if b1a > min_b + ANGLE_EPSILON {
                let (first, second) = MeasuredPolygon::cut_at_progress(&current_b1, min_b);
                (first, Some(second))
            } else {
                let next = bs1.get(i1).cloned();
                if next.is_some() {
                    i1 += 1;
                }
                (current_b1, next)
            };
            let (seg2, new_b2) = if b2a > min_b + ANGLE_EPSILON {
                let (first, second) = MeasuredPolygon::cut_at_progress(
                    &current_b2,
                    positive_modulo(double_mapper.map(min_b) - polygon2_cut_point, 1.0),
                );
                (first, Some(second))
            } else {
                let next = bs2.get(i2).cloned();
                if next.is_some() {
                    i2 += 1;
                }
                (current_b2, next)
            };

            ret.push((seg1.cubic, seg2.cubic));
            b1 = new_b1;
            b2 = new_b2;
        }

        ret
    }
}

pub fn circle_polygon(num_vertices: usize, radius: f32) -> RoundedPolygon {
    assert!(
        num_vertices >= 3,
        "Circle must have at least three vertices"
    );
    let theta = PI / num_vertices as f32;
    let polygon_radius = radius / theta.cos();
    let vertices: Vec<(f32, f32)> = (0..num_vertices)
        .map(|i| radial_to_cartesian(polygon_radius, PI / num_vertices as f32 * 2.0 * i as f32))
        .collect();
    RoundedPolygon::from_vertices(
        &vertices,
        CornerRounding::new(radius),
        None,
        Some((0.0, 0.0)),
    )
}

pub fn star_polygon(
    num_vertices_per_radius: usize,
    radius: f32,
    inner_radius: f32,
    rounding: CornerRounding,
) -> RoundedPolygon {
    assert!(
        radius > 0.0 && inner_radius > 0.0,
        "Star radii must both be greater than 0"
    );
    assert!(
        inner_radius < radius,
        "innerRadius must be less than radius"
    );
    let mut vertices = Vec::with_capacity(num_vertices_per_radius * 2);
    for i in 0..num_vertices_per_radius {
        let outer =
            radial_to_cartesian(radius, PI / num_vertices_per_radius as f32 * 2.0 * i as f32);
        vertices.push(outer);
        let inner = radial_to_cartesian(
            inner_radius,
            PI / num_vertices_per_radius as f32 * (2.0 * i as f32 + 1.0),
        );
        vertices.push(inner);
    }
    RoundedPolygon::from_vertices(&vertices, rounding, None, Some((0.0, 0.0)))
}
