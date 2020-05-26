use geo::{Coordinate, Rect};

pub struct DemographicPoint {
    pub location: Coordinate<f64>,
    pub relative_density: f64
}

impl DemographicPoint {
    pub fn new(x: f64, y: f64, relative_density: f64) -> DemographicPoint {
        let location = Coordinate{ x, y };
        DemographicPoint { location, relative_density}
    }
    pub fn get_location(&self) -> Coordinate<f64> { self.location }
    pub fn get_relative_density(&self) -> f64 { self.relative_density }
}

pub struct DemographicData {
    pub points: Vec<DemographicPoint>,
    pub longitude_cut_line: Option<f64>,
    pub total_density_points: f64
}

impl DemographicData {
    pub fn new(points: Vec<DemographicPoint>, longitude_cut_line: Option<f64>) -> DemographicData {
        let total_density_points = points.iter().map(|p| p.relative_density).sum();
        DemographicData { points, longitude_cut_line, total_density_points }
    }
}