use float_cmp::F64Margin;

pub const EPSILON: f64 = 1e-5;

pub const FLOAT_MARGIN: F64Margin = F64Margin {
    epsilon: EPSILON,
    ulps: 4,
};
