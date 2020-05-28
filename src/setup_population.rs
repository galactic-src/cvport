use geo::Coordinate;
use log::info;
use rand::prelude::*;
use rand_distr::{Binomial, Distribution};

use crate::grid::{Grid, Bounds};
use crate::demographic_data::{ DemographicData, DemographicPoint };


#[derive(Copy, Clone, Debug)]
pub struct AggregatedDemographicData {
    total_density: f64,
    density_count: u64
}

impl PartialEq for AggregatedDemographicData {
    fn eq(&self, other: &Self) -> bool {
        ulps_eq!(self.total_density, other.total_density)
          && self.density_count == other.density_count
    }
}

impl AggregatedDemographicData {
    pub fn new() -> AggregatedDemographicData {
        AggregatedDemographicData {
            total_density: 0.,
            density_count: 0
        }
    }

    pub fn add_density(&mut self, density: f64) {
        self.total_density += density;
        self.density_count += 1;
    }

    pub fn mean(&self) -> f64 {
        if self.density_count == 0 {
            0.
        } else {
            self.total_density / self.density_count as f64
        }
    }
}

pub struct PopulationAllocator {
    compatibility: bool
}

impl PopulationAllocator {
    pub fn new(compatibility: bool) -> PopulationAllocator {
        PopulationAllocator {
            compatibility
        }
    }

    pub fn allocate_population<R: Rng>(&self, grid: &mut Grid, demographic_data: &DemographicData, population: Option<u64>, rng: &mut R) {
        let population = population.unwrap_or(demographic_data.total_density_points as u64);
        info!("Population size = {}", population);

        let density_map = self.aggregate_densities(&demographic_data.points, grid);
        self.allocate_individuals(grid, &density_map, population, rng);
    }

    fn aggregate_densities(&self, points: &Vec<DemographicPoint>, grid: &Grid) -> Vec<Vec<AggregatedDemographicData>> {
        let mut aggregated_densities = Vec::with_capacity(grid.width());

        for _ in 0..grid.width() {
            aggregated_densities.push(vec![AggregatedDemographicData::new() ; grid.height()]);
        }

        points.iter()
            .for_each(|point|
                self.add_density(point, grid, &mut aggregated_densities)
            );

        aggregated_densities
    }

    fn add_density(&self, demographic_point: &DemographicPoint, grid: &Grid, aggregated_densities: &mut Vec<Vec<AggregatedDemographicData>>) {
        let density = demographic_point.get_relative_density();
        let location = demographic_point.get_location();
        let x_min = grid.bounds().x_min();
        let y_min = grid.bounds().y_min();
        let micro_cell_edge_length = grid.micro_cell_edge_length();
        let (x, y) = self.map_point_to_coordinates(&location, x_min, y_min, micro_cell_edge_length);

        aggregated_densities[x][y].add_density(density);
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

    fn sum_density_means(&self, density_map: &Vec<Vec<AggregatedDemographicData>>) -> f64 {
        density_map.iter()
            .flat_map(|v| v.iter())
            .map(|demographic_data| demographic_data.mean())
            .sum()
    }

    fn allocate_individuals<R: Rng>(&self, grid: &mut Grid, aggregated_data: &Vec<Vec<AggregatedDemographicData>>, total_population: u64, rng: &mut R) {
        let sum_density_means = self.sum_density_means(aggregated_data);
        let x_max = grid.width() - 1;
        let y_max = grid.height() - 1;

        let mut remaining_density_fraction: f64 = 1.;
        let mut remaining_population: u64 = total_population;

        for x in 0..=x_max {
            for y in 0..=y_max {
                if x == x_max && y == y_max {
                    grid.micro_cell_mut(x, y).population = remaining_population;
                } else {
                    let mean_population_density_from_demographics = aggregated_data[x][y].mean();
                    let micro_cell_density_fraction = mean_population_density_from_demographics / sum_density_means;
                    let probability_each_person_found_in_this_micro_cell = (micro_cell_density_fraction / remaining_density_fraction);

                    // Clamp to max 1.0 to cater for floating point rounding errors for the last cell containing population
                    // to prevent probability > 1. Only applicable if the actual last cell contains population fraction 0.
                    let clamped_probability = probability_each_person_found_in_this_micro_cell.min(1.);

                    let bin = Binomial::new(remaining_population, clamped_probability).unwrap();
                    let micro_cell_population = bin.sample(rng);

                    grid.micro_cell_mut(x, y).population = micro_cell_population;

                    remaining_population -= micro_cell_population;
                    remaining_density_fraction -= micro_cell_density_fraction;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg32;

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

    #[test]
    fn test_sum_density_means_empty() {
        let data = vec!();
        let allocator = PopulationAllocator { compatibility: false };
        let mean = allocator.sum_density_means(&data);
        assert_ulps_eq!(mean, 0.0);
    }

    #[test]
    fn test_sum_density_means_1_item() {
        let data = vec!(vec!(AggregatedDemographicData { total_density: 1.5, density_count: 1 }));
        let allocator = PopulationAllocator { compatibility: false };
        let mean = allocator.sum_density_means(&data);
        assert_ulps_eq!(mean, 1.5);
    }

    #[test]
    fn test_sum_density_means_2_items() {
        let data = vec!(vec!(
            AggregatedDemographicData { total_density: 1.5, density_count: 1 },
            AggregatedDemographicData { total_density: 2.5, density_count: 1 }
        ));
        let allocator = PopulationAllocator { compatibility: false };
        let mean = allocator.sum_density_means(&data);
        assert_ulps_eq!(mean, 4.0);
    }

    #[test]
    fn test_sum_density_means_many_items() {
        let data = vec!(
            vec!(AggregatedDemographicData { total_density: 1.5, density_count: 1 }, AggregatedDemographicData { total_density: 2.5, density_count: 1 }),
            vec!(AggregatedDemographicData { total_density: 3.5, density_count: 1 }, AggregatedDemographicData { total_density: 4.5, density_count: 1 })
        );
        let allocator = PopulationAllocator { compatibility: false };
        let mean = allocator.sum_density_means(&data);
        assert_ulps_eq!(mean, 12.0);
    }

    #[test]
    fn test_add_density_none() {
        let grid = Grid::new(Bounds::new(0., 0., 0.2, 0.2), 0.1);
        let demographic_data = vec!();

        let allocator = PopulationAllocator { compatibility: false };
        let aggregated_data = allocator.aggregate_densities(&demographic_data, &grid);
        assert_eq!(aggregated_data[0][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[0][1], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[1][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[1][1], AggregatedDemographicData { total_density: 0., density_count: 0});
    }

    #[test]
    fn test_add_density_one() {
        let grid = Grid::new(Bounds::new(0., 0., 0.2, 0.2), 0.1);
        let demographic_data = vec!(DemographicPoint::new(0.07, 0.15, 1.2));

        let allocator = PopulationAllocator { compatibility: false };
        let aggregated_data = allocator.aggregate_densities(&demographic_data, &grid);
        assert_eq!(aggregated_data[0][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[0][1], AggregatedDemographicData { total_density: 1.2, density_count: 1});
        assert_eq!(aggregated_data[1][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[1][1], AggregatedDemographicData { total_density: 0., density_count: 0});
    }

    #[test]
    fn test_add_density_combine_2() {
        let grid = Grid::new(Bounds::new(0., 0., 0.2, 0.2), 0.1);
        let demographic_data = vec!(DemographicPoint::new(0.07, 0.15, 1.2), DemographicPoint::new(0.02, 0.13, 0.7));

        let allocator = PopulationAllocator { compatibility: false };
        let aggregated_data = allocator.aggregate_densities(&demographic_data, &grid);
        assert_eq!(aggregated_data[0][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[0][1], AggregatedDemographicData { total_density: 1.9, density_count: 2});
        assert_eq!(aggregated_data[1][0], AggregatedDemographicData { total_density: 0., density_count: 0});
        assert_eq!(aggregated_data[1][1], AggregatedDemographicData { total_density: 0., density_count: 0});
    }

    #[test]
    fn test_add_density_combine_several() {
        let grid = Grid::new(Bounds::new(0., 0., 1., 1.), 0.5);
        let demographic_data = vec!(
            DemographicPoint::new(0.13, 0.15, 1.2),
            DemographicPoint::new(0.72, 0.33, 0.7),
            DemographicPoint::new(0.88, 0.1, 0.3),
            DemographicPoint::new(0.51, 0.44, 0.2),
            DemographicPoint::new(0.41, 0.94, 0.1),
            DemographicPoint::new(0.17, 0.72, 0.5),
            DemographicPoint::new(0.37, 0.68, 0.3),
            DemographicPoint::new(0.22, 0.56, 0.9)
        );

        let allocator = PopulationAllocator { compatibility: false };
        let aggregated_data = allocator.aggregate_densities(&demographic_data, &grid);
        assert_eq!(aggregated_data[0][0], AggregatedDemographicData { total_density: 1.2, density_count: 1});
        assert_eq!(aggregated_data[1][0], AggregatedDemographicData { total_density: 1.2, density_count: 3});
        assert_eq!(aggregated_data[0][1], AggregatedDemographicData { total_density: 1.8, density_count: 4});
        assert_eq!(aggregated_data[1][1], AggregatedDemographicData { total_density: 0., density_count: 0});
    }

    #[test]
    fn test_allocate_individuals_one_micro_cell() {
        let allocator = PopulationAllocator { compatibility: false };
        let mut grid = Grid::new(Bounds::new(0., 0., 1., 1.), 1.);
        let aggregated_data = vec!(vec!(AggregatedDemographicData {total_density: 1.0, density_count: 1}));
        let population = 1;
        let mut rng = Pcg32::seed_from_u64(0);

        allocator.allocate_individuals(&mut grid, &aggregated_data, population, &mut rng);

        assert_eq!(grid.micro_cell(0,0).population, 1);
    }

    #[test]
    fn test_allocate_individuals_all_people_into_one_microcell() {
        let allocator = PopulationAllocator { compatibility: false };
        let mut grid = Grid::new(Bounds::new(0., 0., 1., 2.), 1.);

        let aggregated_data = vec!(vec!(
            AggregatedDemographicData {total_density: 1.0, density_count: 1},
            AggregatedDemographicData {total_density: 0.0, density_count: 0}
        ));
        let population = 3;
        let mut rng = Pcg32::seed_from_u64(0);

        allocator.allocate_individuals(&mut grid, &aggregated_data, population, &mut rng);

        assert_eq!(grid.micro_cell(0,0).population, 3);
        assert_eq!(grid.micro_cell(0,1).population, 0);
    }

    #[test]
    fn test_allocate_individuals_rough_split_across_two_micro_cells() {
        let allocator = PopulationAllocator { compatibility: false };
        let mut grid = Grid::new(Bounds::new(0., 0., 1., 2.), 1.);

        let aggregated_data = vec!(vec!(
            AggregatedDemographicData {total_density: 1.0, density_count: 1},
            AggregatedDemographicData {total_density: 1.0, density_count: 1}
        ));
        let population = 10;
        let mut rng = Pcg32::seed_from_u64(1);

        allocator.allocate_individuals(&mut grid, &aggregated_data, population, &mut rng);

        assert_eq!(grid.micro_cell(0,0).population, 5);
        assert_eq!(grid.micro_cell(0,1).population, 5);
    }

    #[test]
    fn test_allocate_individuals_rough_split_across_four_micro_cells() {
        let allocator = PopulationAllocator { compatibility: false };
        let mut grid = Grid::new(Bounds::new(0., 0., 2., 2.), 1.);

        let aggregated_data = vec!(
            vec!(
                AggregatedDemographicData {total_density: 1.0, density_count: 1},
                AggregatedDemographicData {total_density: 1.0, density_count: 1}
            ),
            vec!(
                AggregatedDemographicData {total_density: 1.0, density_count: 1},
                AggregatedDemographicData {total_density: 1.0, density_count: 1}));
        let population = 100;
        let mut rng = Pcg32::seed_from_u64(1);

        allocator.allocate_individuals(&mut grid, &aggregated_data, population, &mut rng);

        assert_eq!(grid.micro_cell(0,0).population, 23);
        assert_eq!(grid.micro_cell(0,1).population, 27);
        assert_eq!(grid.micro_cell(1,0).population, 24);
        assert_eq!(grid.micro_cell(1,1).population, 26);
    }

    #[test]
    fn test_allocate_lots_of_people() {
        let allocator = PopulationAllocator { compatibility: false };
        let mut grid = Grid::new(Bounds::new(0., 0., 2., 2.), 1.);

        let aggregated_data = vec!(
            vec!(
                AggregatedDemographicData {total_density: 0.1, density_count: 1},
                AggregatedDemographicData {total_density: 0.1, density_count: 1}
            ),
            vec!(
                AggregatedDemographicData {total_density: 0.3, density_count: 1},
                AggregatedDemographicData {total_density: 0.5, density_count: 1}));
        let total_population = 100000000000;
        let mut rng = Pcg32::seed_from_u64(1234567890);

        allocator.allocate_individuals(&mut grid, &aggregated_data, total_population, &mut rng);

        assert_eq!(grid.micro_cell(0,0).population, 10000230904);
        assert_eq!(grid.micro_cell(0,1).population, 9999969889);
        assert_eq!(grid.micro_cell(1,0).population, 29999953018);
        assert_eq!(grid.micro_cell(1,1).population, 49999846189);

        assert_eq!(
            grid.micro_cell(0,0).population
                + grid.micro_cell(0,1).population
                + grid.micro_cell(1,0).population
                + grid.micro_cell(1,1).population,
            total_population);
    }
}