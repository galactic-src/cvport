use std::f64;
use log::{info, warn};
use geo::{Coordinate, Rect};

use crate::demographic_data::{DemographicPoint, DemographicData};

#[derive(Copy, Clone, Debug)]
pub struct Bounds {
    rect: Rect<f64>
}

impl PartialEq for Bounds {
    fn eq(&self, other: &Bounds) -> bool {
        ulps_eq!(self.rect.min().x, other.rect.min().x)
        && ulps_eq!(self.rect.min().y, other.rect.min().y)
        && ulps_eq!(self.rect.max().x, other.rect.max().x)
        && ulps_eq!(self.rect.max().y, other.rect.max().y)
    }
}

impl Bounds {
    pub fn new(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Bounds {
        Bounds {
            rect: Rect::new(Coordinate { x: x_min, y: y_min }, Coordinate { x: x_max, y: y_max })
        }
    }

    fn width(&self) -> f64 { self.rect.width() }
    fn height(&self) -> f64 { self.rect.height() }

    pub fn x_min(&self) -> f64 { self.rect.min().x }
    pub fn y_min(&self) -> f64 { self.rect.min().y }
    pub fn x_max(&self) -> f64 { self.rect.max().x }
    pub fn y_max(&self) -> f64 { self.rect.max().y }

    fn default() -> Bounds {
        Bounds {
            rect: Rect::new(Coordinate { x: 0.0, y: 0.0}, Coordinate { x: 1.0, y: 1.0 })
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
struct MicroCell {
    population: u32
}

const DELTA: f64 = 1e-6;

pub struct GridOptions {
    pub micro_cell_edge_length: Option<f64>,
    pub area_to_model: Option<Bounds>,
    pub demographic_data: Option<DemographicData>
}

impl GridOptions {
    fn get_micro_cell_edge_length(&self) -> f64 {
        self.micro_cell_edge_length.unwrap_or(DEFAULT_CELL_EDGE)
    }

    fn get_area_to_model(&self) -> Bounds {
        let area_to_model = &self.area_to_model;
        let demographic_data = &self.demographic_data;

        match area_to_model {
            Some(area) => area.clone(),
            None => match demographic_data {
                Some(data) if !data.points.is_empty() => GridOptions::bounding_box_from_data(&data.points),
                _ => Bounds::default()
            }
        }
    }

    fn bounding_box_from_data(points: &Vec<DemographicPoint>) -> Bounds {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for point in points.iter() {
            x_min = f64::min(point.location.x, x_min);
            x_max = f64::max(point.location.x + DELTA, x_max);
            y_min = f64::min(point.location.y, y_min);
            y_max = f64::max(point.location.y + DELTA, y_max);
        }

        Bounds::new(x_min, y_min, x_max, y_max)
    }
}

#[derive(PartialEq, Debug)]
pub struct Grid {
    micro_cell_edge_length: f64,
    bounds: Bounds,
    width: usize,
    height: usize,
    micro_cells: Vec<Vec<MicroCell>>
}

pub struct GridOptionsBuilder {
    options: GridOptions
}

impl GridOptionsBuilder {
    pub fn new() -> GridOptionsBuilder {
        GridOptionsBuilder {
            options: GridOptions {
                micro_cell_edge_length: None,
                area_to_model: None,
                demographic_data: None
            }
        }
    }

    pub fn with_micro_cell_edge_length(mut self, micro_cell_edge_length: f64) -> GridOptionsBuilder {
        self.options.micro_cell_edge_length = Some(micro_cell_edge_length);
        self
    }

    pub fn with_area_to_model(mut self, area_to_model: Bounds) -> GridOptionsBuilder {
        self.options.area_to_model = Some(area_to_model);
        self
    }

    pub fn with_demographic_data(mut self, demographic_data: DemographicData) -> GridOptionsBuilder {
        self.options.demographic_data = Some(demographic_data);
        self
    }

    pub fn build(self) -> GridOptions {
        self.options
    }
}

impl Grid {
    fn new(bounds: Bounds, micro_cell_edge_length: f64) -> Grid {
        let (width, height) = count_micro_cells(micro_cell_edge_length, &bounds);

        let mut micro_cells = Vec::with_capacity(width);

        for _ in 0..width {
            micro_cells.push(Vec::with_capacity(height));
        }

        Grid { micro_cell_edge_length, bounds, width, height, micro_cells }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn bounds(&self) -> &Bounds { &self.bounds }
    pub fn micro_cell_edge_length(&self) -> f64 { self.micro_cell_edge_length }
}

const DEFAULT_CELL_EDGE: f64 = 1.0/120.0;

pub fn build_grid(options: &GridOptions) -> Grid {
    let normalised_area = normalise_area(options);
    let micro_cell_edge_length = options.get_micro_cell_edge_length();
    let grid = Grid::new(normalised_area, micro_cell_edge_length);
    log_create_grid_info(&grid, &normalised_area);

    grid
}

fn log_create_grid_info(grid: &Grid, bounds: &Bounds) {
    info!("Adjusted bounding box = ({}, {}) - ({}, {})", bounds.rect.min().x, bounds.rect.min().y, bounds.rect.max().x, bounds.rect.max().y );
    info!("Number of cells = {} ({} x {})", grid.width() * grid.height(), grid.width(), grid.height());
    if grid.width > 180 {
        warn!("Width of bounding box > 180 degrees.  Results may be inaccurate.");
    }
    if grid.height > 90 {
        warn!("Height of bounding box > 90 degrees.  Results may be inaccurate.");
    }
}

fn normalise_area(options: &GridOptions) -> Bounds {
    let micro_cell_edge_length = options.get_micro_cell_edge_length();
    let aligned_area = align_area_to_cell_size(micro_cell_edge_length, &options.get_area_to_model());
    ensure_dimensions_multiple_of_4_cells(micro_cell_edge_length, &aligned_area)
}

fn align_area_to_cell_size(micro_cell_edge_length: f64, raw_area_to_model: &Bounds) -> Bounds {
    Bounds::new(
        round_down_to_unit(raw_area_to_model.x_min(), micro_cell_edge_length),
        round_down_to_unit(raw_area_to_model.y_min(), micro_cell_edge_length),
        round_up_to_unit(raw_area_to_model.x_max(), micro_cell_edge_length),
        round_up_to_unit(raw_area_to_model.y_max(), micro_cell_edge_length)
    )
}

fn round_down_to_unit(raw_value: f64, unit: f64) -> f64 {
    (raw_value / unit).floor() * unit
}

fn round_up_to_unit(raw_value: f64, unit: f64) -> f64 {
    (raw_value / unit).ceil() * unit
}

fn ensure_dimensions_multiple_of_4_cells(micro_cell_edge_length: f64, area: &Bounds) -> Bounds {
    let rounded_width = round_up_to_unit(area.width(), 4.0 * micro_cell_edge_length);
    let rounded_height = round_up_to_unit(area.height(), 4.0 * micro_cell_edge_length);

    Bounds::new(area.x_min(), area.y_min(), area.x_min() + rounded_width, area.y_min() + rounded_height)
}

fn count_micro_cells(micro_cell_edge_length: f64, area: &Bounds) -> (usize, usize) {
    let cells_x = (area.width() / micro_cell_edge_length).round() as usize;
    let cells_y = (area.height() / micro_cell_edge_length).round() as usize;
    (cells_x, cells_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_down_to_unit_whole_number() {
        let actual = round_down_to_unit(5.2, 1.0);
        assert_ulps_eq!(actual, 5.0);
    }

    #[test]
    fn test_round_down_to_unit_fraction() {
        let actual = round_down_to_unit(5.5, 1.0/3.0);
        assert_ulps_eq!(actual, 5.0 + (1.0/3.0));
    }

    #[test]
    fn test_round_down_to_unit_nothing_to_do() {
        let actual = round_down_to_unit(5.0, 1.0);
        assert_ulps_eq!(actual, 5.0);
    }

    #[test]
    fn test_round_down_to_unit_zero() {
        let actual = round_down_to_unit(0.0, 1.0);
        assert_ulps_eq!(actual, 0.0);
    }

    #[test]
    fn test_round_up_to_unit_whole_number() {
        let actual = round_up_to_unit(5.2, 1.0);
        assert_ulps_eq!(actual, 6.0);
    }

    #[test]
    fn test_round_up_to_unit_fraction() {
        let actual = round_up_to_unit(5.5, 1.0/3.0);
        assert_ulps_eq!(actual, 5.0 + (2.0/3.0));
    }

    #[test]
    fn test_round_up_to_unit_nothing_to_do() {
        let actual = round_up_to_unit(5.0, 1.0);
        assert_ulps_eq!(actual, 5.0);
    }

    #[test]
    fn test_round_up_to_unit_zero() {
        let actual = round_up_to_unit(0.0, 1.0);
        assert_ulps_eq!(actual, 0.0);
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
        let actual = count_micro_cells(1.0, &Bounds::new(50.0, 101.5, 70.0, 104.5));
        assert_eq!(actual, (20, 3));
    }

    #[test]
    fn test_count_cells_fractional_edge() {
        let x_min = 2.75;
        let y_min = 21.5;
        let micro_cell_edge_length = 0.25;
        let actual = count_micro_cells(micro_cell_edge_length, &Bounds::new(x_min, y_min, x_min + 3.0 * micro_cell_edge_length, y_min + 7.0 * micro_cell_edge_length));
        assert_eq!(actual, (3, 7));
    }

    #[test]
    fn test_get_micro_cell_edge_length_default() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_micro_cell_edge_length(), DEFAULT_CELL_EDGE);
    }

    #[test]
    fn test_get_micro_cell_edge_length_non_default() {
        let options = GridOptions {
            micro_cell_edge_length: Some(2.5),
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_micro_cell_edge_length(), 2.5);
    }

    #[test]
    fn test_get_area_to_model_specified_area() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: Some(Bounds::new(1.0, 1.0, 2.0, 2.0)),
            demographic_data: Some(DemographicData {
                longitude_cut_line: None,
                points: vec!(
                    DemographicPoint { location: Coordinate{ x: 0.0, y: 9.25 }, relative_density: 1.0},
                    DemographicPoint { location: Coordinate{ x: 10.1, y: 8.01 }, relative_density: 1.0}
                ),
                total_density_points: 2.0
            })
        };

        assert_eq!(options.get_area_to_model(), Bounds::new( 1.0, 1.0, 2.0, 2.0 ));
    }

    #[test]
    fn test_get_area_to_model_demographic_data() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: None,
            demographic_data: Some(DemographicData {
                longitude_cut_line: None,
                points: vec!(
                    DemographicPoint { location: Coordinate{ x: 0.0, y: 9.25 }, relative_density: 1.0},
                    DemographicPoint { location: Coordinate{ x: 10.1, y: 8.01 }, relative_density: 1.0}
                ),
                total_density_points: 2.0
            })
        };

        assert_eq!(options.get_area_to_model(), Bounds::new( 0.0, 8.01, 10.1 + DELTA, 9.25 + DELTA ));
    }

    #[test]
    fn test_get_area_to_model_default() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        assert_eq!(options.get_area_to_model(), Bounds::default());
    }

    #[test]
    fn test_bounding_box_from_data_1_point() {
        let points = vec!(DemographicPoint{ location: Coordinate{ x: 0.0, y: 1.0 }, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds::new( 0.0, 1.0, 0.0 + DELTA, 1.0 + DELTA ))
    }

    #[test]
    fn test_bounding_box_from_data_2_points() {
        let points = vec!(
            DemographicPoint{ location: Coordinate{ x: 0.0, y: 1.0 }, relative_density: 1.0},
            DemographicPoint{ location: Coordinate{ x: 1.0, y: 0.0 }, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds::new( 0.0, 0.0, 1.0 + DELTA, 1.0 + DELTA ))
    }

    #[test]
    fn test_bounding_box_from_data_4_points() {
        let points = vec!(
            DemographicPoint{ location: Coordinate{ x: 0.0, y: -3.0 }, relative_density: 1.0},
            DemographicPoint{ location: Coordinate{ x: -7.0, y: -1.0 }, relative_density: 1.0},
            DemographicPoint{ location: Coordinate{ x: 5.0, y: 1.0 }, relative_density: 1.0},
            DemographicPoint{ location: Coordinate{ x: 1.0, y: 0.0 }, relative_density: 1.0});
        let actual = GridOptions::bounding_box_from_data(&points);
        assert_eq!(actual, Bounds::new( -7.0, -3.0, 5.0 + DELTA, 1.0 + DELTA ))
    }

    #[test]
    fn test_build_grid_default() {
        let default_options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: None,
            demographic_data: None
        };

        let default_grid = build_grid(&default_options);
        assert_eq!(Grid::new(Bounds::new( 0.0, 0.0, 1.0, 1.0), DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_edge_only() {
        let default_options = GridOptions {
            micro_cell_edge_length: Some(0.1),
            area_to_model: None,
            demographic_data: None
        };

        let default_grid = build_grid(&default_options);
        assert_eq!(Grid::new(Bounds::new( 0.0, 0.0, 1.2, 1.2), 0.1), default_grid)
    }

    #[test]
    fn test_build_grid_from_area_only() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: Some(Bounds::new( 1.5, 2.5, 3.5, 5.5)),
            demographic_data: None
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds::new( 1.5, 2.5, 3.5, 5.5 ), DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_demographic_data_only() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: None,
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { location: Coordinate{ x: 7.0, y: 8.0 }, relative_density: 1.0 }
            ),
            total_density_points: 2.0})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds::new( 7.0, 8.0, 7.0 + 4.0 * DEFAULT_CELL_EDGE, 8.0 + 4.0 * DEFAULT_CELL_EDGE ), DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_prefers_supplied_area() {
        let options = GridOptions {
            micro_cell_edge_length: None,
            area_to_model: Some(Bounds::new( 1.5, 2.5, 3.5, 5.5)),
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { location: Coordinate{ x: 7.0, y: 8.0 }, relative_density: 1.0 }
            ),
            total_density_points: 2.0})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds::new( 1.5, 2.5, 3.5, 5.5 ), DEFAULT_CELL_EDGE), default_grid)
    }

    #[test]
    fn test_build_grid_from_area_uses_supplied_edge() {
        let options = GridOptions {
            micro_cell_edge_length: Some(0.1),
            area_to_model: Some(Bounds::new( 1.5, 2.5, 3.5, 5.5 )),
            demographic_data: None
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds::new( 1.5, 2.5, 3.5, 5.7 ), 0.1), default_grid)
    }

    #[test]
    fn test_build_grid_from_data_uses_supplied_edge() {
        let options = GridOptions {
            micro_cell_edge_length: Some(0.1),
            area_to_model: None,
            demographic_data: Some(DemographicData{ longitude_cut_line: None, points: vec!(
                DemographicPoint { location: Coordinate{ x: 7.0, y: 8.0 }, relative_density: 1.0 },
                DemographicPoint { location: Coordinate{ x: 8.0, y: 9.0 }, relative_density: 1.0 },
            ),
                total_density_points: 2.0})
        };

        let default_grid = build_grid(&options);
        assert_eq!(Grid::new(Bounds::new( 7.0, 8.0, 8.2, 9.2 ), 0.1), default_grid)
    }

    #[test]
    #[should_panic]
    fn test_bounds_min_gt_max_panics() {
        Bounds::new(0., 5., 5., 0.);
    }
}