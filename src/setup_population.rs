use geo::Coordinate;
use log::info;

use crate::grid::{Grid, DemographicData, DemographicPoint};

pub struct PopulationAllocator {
    compatibility: bool
}

impl PopulationAllocator {
    pub fn new(compatibility: bool) -> PopulationAllocator {
        PopulationAllocator {
            compatibility
        }
    }

    pub fn allocate_population(&self, grid: &mut Grid, demographic_data: &DemographicData, population: Option<u64>) {
        let population = population.unwrap_or(demographic_data.total_density_points as u64);
        info!("Population size = {}", population);

        let (density_map, total_density_points) = self.aggregate_densities(&demographic_data.points, grid);


    }

    fn aggregate_densities(&self, points: &Vec<DemographicPoint>, grid: &Grid) -> (Vec<Vec<f64>>, f64) {
        let mut aggregated_densities = Vec::with_capacity(grid.width());
        let mut total_density_points: f64 = 0.0;

        for _ in 0..grid.width() {
            aggregated_densities.push(vec![0.0; grid.height()]);
        }

        points.iter()
            .for_each(|point| {
                self.add_density(point, grid, &mut aggregated_densities, &mut total_density_points);
            });

        (aggregated_densities, total_density_points)
    }

    fn add_density(&self, demographic_point: &DemographicPoint, grid: &Grid, aggregated_densities: &mut Vec<Vec<f64>>, total_density_points: &mut f64) {
        let density = demographic_point.get_relative_density();
        let location = demographic_point.get_location();
        let x_min = grid.bounds().x_min();
        let y_min = grid.bounds().y_min();
        let micro_cell_edge_length = grid.micro_cell_edge_length();
        let (x, y) = self.map_point_to_coordinates(&location, x_min, y_min, micro_cell_edge_length);

        aggregated_densities[x][y] += density;
        *total_density_points += density;
    }

    fn map_point_to_coordinates(&self, point: &Coordinate<f64>, base_x: f64, base_y: f64, micro_cell_edge_length: f64) -> (usize, usize) {
        (
            self.map_axis_to_micro_cell_index(point.x, base_x, micro_cell_edge_length),
            self.map_axis_to_micro_cell_index(point.y, base_y, micro_cell_edge_length)
        )
    }

    fn map_axis_to_micro_cell_index(&self, location: f64, base: f64, micro_cell_edge_length: f64) -> usize {
        if location < base {
            panic!("cannot map location {} because it is less than the base axis value {} (out of bounds)", location, base)
        }

        let translated = location - base;

        let fractional_micro_cells = translated / micro_cell_edge_length
            + if self.compatibility { 1.0 } else { 0.0 };

        fractional_micro_cells.floor() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_axis_to_micro_cell_index_simple_no_compatibility() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(0.0, 0.0, 1.0);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_map_axis_to_micro_cell_rounds_down() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(0.99, 0.0, 1.0);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_map_axis_to_micro_cell_compatibility() {
        let allocator = PopulationAllocator { compatibility: true };
        let actual = allocator.map_axis_to_micro_cell_index(0.99, 0.0, 1.0);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_map_axis_to_micro_cell_subtracts_base() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(1.9, 1.0, 1.0);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_map_axis_to_micro_cell_bigger_than_zero() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(2.9, 0.5, 1.0);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_map_axis_to_micro_cell_scales_with_micro_cell_edge() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(3.45, 0.0, 0.1);
        assert_eq!(actual, 34);
    }

    #[test]
    fn test_map_axis_to_micro_cell_complex() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_axis_to_micro_cell_index(14.7, -3.4, 0.5);
        assert_eq!(actual, 36);
    }

    #[test]
    fn test_map_point_to_coordinates() {
        let allocator = PopulationAllocator { compatibility: false };
        let actual = allocator.map_point_to_coordinates(&Coordinate{ x: 5.0, y: 15.0 }, 2.5, 11.5, 0.5);
        assert_eq!(actual, (5, 7));
    }
}