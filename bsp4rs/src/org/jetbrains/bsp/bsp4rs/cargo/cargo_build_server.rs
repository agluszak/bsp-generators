use crate::*;

/// The cargo features state request is sent from the client to the server to
/// query for the current state of the Cargo features. Provides also mapping
/// between Cargo packages and build target identifiers.
#[derive(Debug)]
pub enum CargoFeaturesState {}

impl Request for CargoFeaturesState {
    type Params = ();
    type Result = CargoFeaturesStateResult;
    const METHOD: &'static str = "workspace/cargoFeaturesState";
}

/// The enable cargo features request is sent from the client to the server to
/// set provided features collection as a new state for
/// the specified Cargo package.
#[derive(Debug)]
pub enum SetCargoFeatures {}

impl Request for SetCargoFeatures {
    type Params = SetCargoFeaturesParams;
    type Result = SetCargoFeaturesResult;
    const METHOD: &'static str = "workspace/setCargoFeatures";
}
