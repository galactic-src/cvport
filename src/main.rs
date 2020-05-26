mod grid;
mod setup_population;
mod compatibility;
mod demographic_data;

#[macro_use]
extern crate approx;

use grid::{ GridOptionsBuilder, build_grid, Bounds, GridOptions };
use demographic_data::{DemographicData, DemographicPoint};
use setup_population::{ PopulationAllocator };
use crate::compatibility::USE_COMPATIBILITY;

fn main() {
    let options = get_options();

    let mut grid = build_grid(&options);

    let population_override = None;

    PopulationAllocator::new(USE_COMPATIBILITY)
        .allocate_population(&mut grid, &options.demographic_data.unwrap(), population_override);
}

fn get_options() -> GridOptions {
    let demographic_data = DemographicData::new(get_demographic_points(), None);

    GridOptionsBuilder::new()
        .with_area_to_model(Bounds::new(0.0, 0.0, 0.2, 0.2))
        .with_demographic_data(demographic_data)
        .with_micro_cell_edge_length(0.1)
        .build()
}

fn get_demographic_points() -> Vec<DemographicPoint> {
    vec!(
        DemographicPoint::new(0.0, 0.0, 0.1),
        DemographicPoint::new(0.0, 0.1, 0.03),
        DemographicPoint::new(0.0, 0.2, 0.22),
        DemographicPoint::new(0.1, 0.0, 0.31),
        DemographicPoint::new(0.1, 0.1, 0.015),
        DemographicPoint::new(0.1, 0.2, 0.27),
        DemographicPoint::new(0.2, 0.0, 0.11),
        DemographicPoint::new(0.2, 0.1, 0.1),
        DemographicPoint::new(0.2, 0.2, 0.62)
    )
}