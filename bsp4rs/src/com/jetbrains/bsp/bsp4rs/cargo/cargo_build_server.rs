use crate::*;

#[derive(Debug)]
pub enum CargoFeaturesState {}

impl Request for CargoFeaturesState {
    type Params = ();
    type Result = CargoFeaturesStateResult;
    const METHOD: &'static str = "workspace/cargoFeaturesState";
}

#[derive(Debug)]
pub enum EnableCargoFeatures {}

impl Request for EnableCargoFeatures {
    type Params = EnableCargoFeaturesParams;
    type Result = ();
    const METHOD: &'static str = "workspace/enableCargoFeatures";
}

#[derive(Debug)]
pub enum DisableCargoFeatures {}

impl Request for DisableCargoFeatures {
    type Params = DisableCargoFeaturesParams;
    type Result = ();
    const METHOD: &'static str = "workspace/disableCargoFeatures";
}
