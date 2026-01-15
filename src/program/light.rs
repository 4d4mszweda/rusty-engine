pub struct WorldLight {
    pub is_on: bool,
    pub spec_model: SpecModel,
    pub light_type: LightType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
}

pub struct PointLight {}

pub struct DirectionalLight {}

/// SpecModel its a  spectral model
pub enum SpecModel {
    Phong,
    BlinnPhong,
}
