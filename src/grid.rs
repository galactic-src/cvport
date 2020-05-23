use std::f64;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64
}

#[derive(Copy, Clone, Debug)]
struct Bounds {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64
}

fn nearly_equal(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b { // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (f64::EPSILON * f64::MIN_POSITIVE)
    } else { // Use relative error.
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
    }
}

impl PartialEq for Bounds {
    fn eq(&self, other: &Bounds) -> bool {
        nearly_equal(self.x_min, other.x_min)
        && nearly_equal(self.y_min, other.y_min)
        && nearly_equal(self.x_max, other.x_max)
        && nearly_equal(self.y_max, other.y_max)
    }
}

impl Bounds {
    fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Bounds {
        if x_max <= x_min {
            panic!("box x_max ({}) must be greater than x_min ({})", x_max, x_min);
        }
        else if y_max <= y_min {
            panic!("box y_max ({}) must be greater than y_min ({})", y_max, y_min);
        }

        Bounds { x_min, y_min, x_max, y_max }
    }

    fn width(&self) -> f64 { self.x_max - self.x_min }
    fn height(&self) -> f64 { self.y_max - self.y_min }

    fn contains(&self, point: &Point) -> bool {
        return point.x >= self.x_min
         && point.x < self.x_max
         && point.y >= self.y_min
         && point.y < self.y_max
    }
}

struct DemographicData {
    longitude_cut_line: Option<f64>,
    points: Vec<DemographicPoint>
}

struct DemographicPoint {
    x: f64,
    y: f64,
    relative_density: f64
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct MicroCell {
    population: u32
}

struct GridOptions {
    cell_edge_length: Option<f64>,
    area_to_model: Option<Bounds>,
    demographic_data: Option<DemographicData>
}

#[derive(PartialEq, Debug)]
struct Grid {
    cell_edge_length: f64,
    bounds: Bounds,
    width: usize,
    height: usize,
    micro_cells: Vec<Vec<MicroCell>>
}



impl Grid {
    fn new(bounds: Bounds, cell_edge_length: f64) -> Grid {
        let (width, height) = count_cells(cell_edge_length, &bounds);

        let mut micro_cells = Vec::with_capacity(width);

        for _ in 0..width {
            micro_cells.push(Vec::with_capacity(height));
        }

        Grid { cell_edge_length, bounds, width, height, micro_cells }
    }
}

const DELTA: f64 = 1e-6;

impl GridOptions {
    fn get_cell_edge_length(&self) -> f64 {
        self.cell_edge_length.unwrap_or(DEFAULT_CELL_EDGE)
    }

    fn get_area_to_model(&self) -> Bounds {
        let area_to_model = &self.area_to_model;
        let demographic_data = &self.demographic_data;

        match area_to_model {
            Some(area) => area.clone(),
            None => match demographic_data {
                Some(data) if !data.points.is_empty() => GridOptions::bounding_box_from_data(&data.points),
                _ => DEFAULT_BOUNDING_BOX.clone()
            }
        }
    }

    fn bounding_box_from_data(points: &Vec<DemographicPoint>) -> Bounds {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for point in points.iter() {
            x_min = f64::min(point.x, x_min);
            x_max = f64::max(point.x + DELTA, x_max);
            y_min = f64::min(point.y, y_min);
            y_max = f64::max(point.y + DELTA, y_max);
        }

        Bounds::new(x_min, y_min, x_max, y_max)
    }
}

const DEFAULT_CELL_EDGE: f64 = 1.0/120.0;
const DEFAULT_BOUNDING_BOX: Bounds = Bounds {
    x_min: 0.0,
    y_min: 0.0,
    x_max: 1.0,
    y_max: 1.0
};

fn build_grid(options: &GridOptions) -> Grid {
    let normalised_area = normalise_area(options);
    let cell_edge_length = options.get_cell_edge_length();
    Grid::new(normalised_area, cell_edge_length)
}

fn normalise_area(options: &GridOptions) -> Bounds {
    let cell_edge_length = options.get_cell_edge_length();
    let aligned_area = align_area_to_cell_size(cell_edge_length, &options.get_area_to_model());
    ensure_dimensions_multiple_of_4_cells(cell_edge_length, &aligned_area)
}

fn align_area_to_cell_size(cell_edge_length: f64, raw_area_to_model: &Bounds) -> Bounds {
    Bounds::new(
        round_down_to_unit(raw_area_to_model.x_min, cell_edge_length),
        round_down_to_unit(raw_area_to_model.y_min, cell_edge_length),
        round_up_to_unit(raw_area_to_model.x_max, cell_edge_length),
        round_up_to_unit(raw_area_to_model.y_max, cell_edge_length)
    )
}

fn round_down_to_unit(raw_value: f64, unit: f64) -> f64 {
    (raw_value / unit).floor() * unit
}

fn round_up_to_unit(raw_value: f64, unit: f64) -> f64 {
    (raw_value / unit).ceil() * unit
}

fn ensure_dimensions_multiple_of_4_cells(cell_edge_length: f64, area: &Bounds) -> Bounds {
    let rounded_width = round_up_to_unit(area.width(), 4.0 * cell_edge_length);
    let rounded_height = round_up_to_unit(area.height(), 4.0 * cell_edge_length);

    Bounds::new(area.x_min, area.y_min, area.x_min + rounded_width, area.y_min + rounded_height)
}

fn count_cells(cell_edge_length: f64, area: &Bounds) -> (usize, usize) {
    let cells_x = (area.width() / cell_edge_length).round() as usize;
    let cells_y = (area.height() / cell_edge_length).round() as usize;
    (cells_x, cells_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_floats_equal(actual: f64, expected: f64) {

        if !nearly_equal(actual, expected) {
            panic!(r#"assertion failed: `(left nearly_equal right)`
  left: `{:?}`,
 right: `{:?}`"#, actual, expected)
        }
    }

    #[test]
    #[should_panic]
    fn test_create_box_x_equal_panic() {
        Bounds::new(1.0, 1.0, 1.0, 4.0);
    }

    #[test]
    fn test_round_down_to_unit_whole_number() {
        let actual = round_down_to_unit(5.2, 1.0);
        assert_floats_equal(actual, 5.0);
    }

    #[test]
    fn test_round_down_to_unit_fraction() {
        let actual = round_down_to_unit(5.5, 1.0/3.0);
        assert_floats_equal(actual, 5.0 + (1.0/3.0));
    }

    #[test]
    fn test_round_down_to_unit_nothing_to_do() {
        let actual = round_down_to_unit(5.0, 1.0);
        assert_floats_equal(actual, 5.0);
    }

    #[test]
    fn test_round_down_to_unit_zero() {
        let actual = round_down_to_unit(0.0, 1.0);
        assert_floats_equal(actual, 0.0);
    }

    #[test]
    fn test_round_up_to_unit_whole_number() {
        let actual = round_up_to_unit(5.2, 1.0);
        assert_floats_equal(actual, 6.0);
    }

    #[test]
    fn test_round_up_to_unit_fraction() {
        let actual = round_up_to_unit(5.5, 1.0/3.0);
        assert_floats_equal(actual, 5.0 + (2.0/3.0));
    }

    #[test]
    fn test_round_up_to_unit_nothing_to_do() {
        let actual = round_up_to_unit(5.0, 1.0);
        assert_floats_equal(actual, 5.0);
    }

    #[test]
    fn test_round_up_to_unit_zero() {
        let actual = round_up_to_unit(0.0, 1.0);
        assert_floats_equal(actual, 0.0);
    }

    #[test]
    fn test_align_area_to_cell_size_nothing_to_do() {
        let actual = align_area_to_cell_size(1.0, &Bounds::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(actual, Bounds::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_align_area_to_cell_size_round_min_down_max_up() {
        let actual = align_area_to_cell_size(1.0, &Bounds::new(1.9, 2.9, 2.1, 3.1));
        assert_eq!(actual, Bounds::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_align_area_to_cell_size_fractional_cell() {
        let actual = align_area_to_cell_size(1.0/3.0, &Bounds::new(1.9, 2.9, 2.1, 3.1));
        assert_eq!(actual, Bounds::new(1.0 + (2.0/3.0), 2.0 + (2.0/3.0), 2.0 + (1.0/3.0), 3.0 + (1.0/3.0)));
    }

    #[test]
    fn test_ensure_dimensions_multiple_of_4_cells_do_nothing() {
        let actual = ensure_dimensions_multiple_of_4_cells(0.1, &Bounds::new(0.0, 0.4, 0.8, 1.2));
        assert_eq!(actual, Bounds::new(0.0, 0.4, 0.8, 1.2));
    }

    #[test]
    fn test_ensure_dimensions_multiple_of_4_cells_rounds_up() {
        let x_min = 0.5;
        let y_min = 1.2;
        let actual = ensure_dimensions_multiple_of_4_cells(1.0, &Bounds::new(x_min, y_min, x_min + 0.001, y_min + 7.999));
        assert_eq!(actual, Bounds::new(0.5, 1.2, 0.5 + 4.0, 1.2 + 8.0));
    }

    #[test]
    fn test_count_cells() {
        let actual = count_cells(1.0, &Bounds::new(50.0, 101.5, 70.0, 104.5));
        assert_eq!(actual, (20, 3));
    }

    #[test]
    fn test_count_cells_fractional_edge() {
        let x_min = 2.75;
        let y_min = 21.5;
        let cell_edge_length = 0.25;
        let actual = count_cells(cell_edge_length, &Bounds::new(x_min, y_min, x_min + 3.0 * cell_edge_length, y_min + 7.0 * cell_edge_length));
        assert_eq!(actual, (3, 7));
    }

    #[test]
    fn test_get_cell_edge_length_default() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_cell_edge_length(), DEFAULT_CELL_EDGE);
    }

    #[test]
    fn test_get_cell_edge_length_non_default() {
        let options = GridOptions {
            cell_edge_length: Some(2.5),
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_cell_edge_length(), 2.5);
    }

    #[test]
    fn test_get_area_to_model_specified_area() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: Some(Bounds::new(1.0, 1.0, 2.0, 2.0)),
            demographic_data: Some(DemographicData {
                longitude_cut_line: None,
                points: vec!(
                    DemographicPoint {x: 0.0, y: 9.25, relative_density: 1.0},
                    DemographicPoint {x: 10.1, y: 8.01, relative_density: 1.0}
                )
            })
        };

        assert_eq!(options.get_area_to_model(), Bounds { x_min: 1.0, y_min: 1.0, x_max: 2.0, y_max: 2.0 });
    }

    #[test]
    fn test_get_area_to_model_demographic_data() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: None,
            demographic_data: Some(DemographicData {
                longitude_cut_line: None,
                points: vec!(
                    DemographicPoint {x: 0.0, y: 9.25, relative_density: 1.0},
                    DemographicPoint {x: 10.1, y: 8.01, relative_density: 1.0}
                )
            })
        };

        assert_eq!(options.get_area_to_model(), Bounds { x_min: 0.0, y_min: 8.01, x_max: 10.1 + DELTA, y_max: 9.25 + DELTA });
    }

    #[test]
    fn test_get_area_to_model_default() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_area_to_model(), DEFAULT_BOUNDING_BOX);
    }

    #[test]
    fn test_bounding_box_from_data_1_point() {
        let points = vec!(DemographicPoint{x: 0.0, y: 1.0, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds { x_min: 0.0, y_min: 1.0, x_max: 0.0 + DELTA, y_max: 1.0 + DELTA })
    }

    #[test]
    fn test_bounding_box_from_data_2_points() {
        let points = vec!(
            DemographicPoint{x: 0.0, y: 1.0, relative_density: 1.0},
            DemographicPoint{x: 1.0, y: 0.0, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds { x_min: 0.0, y_min: 0.0, x_max: 1.0 + DELTA, y_max: 1.0 + DELTA })
    }

    #[test]
    fn test_bounding_box_from_data_4_points() {
        let points = vec!(
            DemographicPoint{x: 0.0, y: -3.0, relative_density: 1.0},
            DemographicPoint{x: -7.0, y: -1.0, relative_density: 1.0},
            DemographicPoint{x: 5.0, y: 1.0, relative_density: 1.0},
            DemographicPoint{x: 1.0, y: 0.0, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds { x_min: -7.0, y_min: -3.0, x_max: 5.0 + DELTA, y_max: 1.0 + DELTA })
    }

    #[test]
    fn test_build_grid_default() {
        let default_options = GridOptions {
            cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        let default_grid = build_grid(&default_options);
        assert_eq!(Grid::new(Bounds{ x_min: 0.0, y_min: 0.0, x_max: 1.0, y_max: 1.0}, DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_edge_only() {
        let default_options = GridOptions {
            cell_edge_length: Some(0.1),
            area_to_model: None,
            demographic_data: None
        };

        let default_grid = build_grid(&default_options);
        assert_eq!(Grid::new(Bounds{ x_min: 0.0, y_min: 0.0, x_max: 1.2, y_max: 1.2}, 0.1), default_grid)
    }

    #[test]
    fn test_build_grid_from_area_only() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: Some(Bounds {x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.5}),
            demographic_data: None
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds{ x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.5 }, DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_demographic_data_only() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: None,
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { x: 7.0, y: 8.0, relative_density: 1.0 }
            )})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds{ x_min: 7.0, y_min: 8.0, x_max: 7.0 + 4.0 * DEFAULT_CELL_EDGE, y_max: 8.0 + 4.0 * DEFAULT_CELL_EDGE }, DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_prefers_supplied_area() {
        let options = GridOptions {
            cell_edge_length: None,
            area_to_model: Some(Bounds {x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.5}),
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { x: 7.0, y: 8.0, relative_density: 1.0 }
            )})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds{ x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.5 }, DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_area_uses_supplied_edge() {
        let options = GridOptions {
            cell_edge_length: Some(0.1),
            area_to_model: Some(Bounds { x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.5 }),
            demographic_data: None
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds{ x_min: 1.5, y_min: 2.5, x_max: 3.5, y_max: 5.7 }, 0.1), default_grid)
    }

    #[test]
    fn test_build_grid_from_data_uses_supplied_edge() {
        let options = GridOptions {
            cell_edge_length: Some(0.1),
            area_to_model: None,
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { x: 7.0, y: 8.0, relative_density: 1.0 },
                DemographicPoint { x: 8.0, y: 9.0, relative_density: 1.0 },
            )})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds{ x_min: 7.0, y_min: 8.0, x_max: 8.2, y_max: 9.2 }, 0.1), default_grid)
    }
}