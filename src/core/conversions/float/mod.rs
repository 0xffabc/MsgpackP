use crate::core::conversions::Serialized;

impl From<f32> for Serialized {
    fn from(value: f32) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<f64> for Serialized {
    fn from(value: f64) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl Into<f32> for Serialized {
    fn into(self) -> f32 {
        if self.0.len() != 4 {
            return 0.0;
        }

        f32::from_be_bytes(self.0.try_into().unwrap_or([0; 4]))
    }
}

impl Into<f64> for Serialized {
    fn into(self) -> f64 {
        if self.0.len() != 8 {
            return 0.0;
        }

        f64::from_be_bytes(self.0.try_into().unwrap_or([0; 8]))
    }
}
