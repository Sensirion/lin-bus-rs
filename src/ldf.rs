//! Contains structs representing data of the LDF file

use crate::frame::diagnostic::ProductId;
use crate::frame::transport::NAD;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct P2Min(pub f32);

impl Default for P2Min {
    fn default() -> P2Min {
        P2Min(50.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct STMin(pub f32);

impl Default for STMin {
    fn default() -> STMin {
        STMin(0.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NAsTimeout(pub f32);

impl Default for NAsTimeout {
    fn default() -> NAsTimeout {
        NAsTimeout(1000.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct NCrTimeout(pub f32);

impl Default for NCrTimeout {
    fn default() -> NCrTimeout {
        NCrTimeout(1000.0)
    }
}

/// Holds the most important node attributes
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NodeAttributes {
    pub configured_nad: NAD,
    pub initial_nad: NAD,
    pub product_id: ProductId,
    pub p2_min: P2Min,
    pub st_min: STMin,
    pub n_as_timeout: NAsTimeout,
    pub n_cr_timeout: NCrTimeout,
}

impl NodeAttributes {
    pub fn with_default_timing(
        configured_nad: NAD,
        initial_nad: NAD,
        product_id: ProductId,
    ) -> NodeAttributes {
        NodeAttributes {
            configured_nad,
            initial_nad,
            product_id,
            p2_min: P2Min::default(),
            st_min: STMin::default(),
            n_as_timeout: NAsTimeout::default(),
            n_cr_timeout: NCrTimeout::default(),
        }
    }
}
