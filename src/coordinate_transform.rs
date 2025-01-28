use geo_types::Coord;
use tiff::{TiffError, TiffFormatError, TiffResult};

#[cfg(feature = "tie-points")]
mod tie_points;

const MODEL_TIE_POINT_TAG: &str = "ModelTiePointTag";
const MODEL_PIXEL_SCALE_TAG: &str = "ModelPixelScaleTag";
const MODEL_TRANSFORMATION_TAG: &str = "ModelTransformationTag";

/// Defines the transformation between raster space and model space.
///
/// Ref: https://docs.ogc.org/is/19-008r4/19-008r4.html#_raster_to_model_coordinate_transformation_requirements
#[derive(Debug)]
pub enum CoordinateTransform {
    AffineTransform(AffineTransform),
    TiePointAndPixelScale(TiePointAndPixelScale),
    #[cfg(feature = "tie-points")]
    TiePoints(tie_points::TiePoints),
}

impl CoordinateTransform {
    pub(super) fn from_tag_data(
        pixel_scale_data: Option<Vec<f64>>,
        model_tie_points_data: Option<Vec<f64>>,
        model_transformation_data: Option<Vec<f64>>,
    ) -> TiffResult<Self> {
        let pixel_scale = pixel_scale_data
            .map(|data| {
                <[f64; 3]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number values in {MODEL_PIXEL_SCALE_TAG} must be equal to 3"
                    )))
                })
            })
            .transpose()?;
        let tie_points = model_tie_points_data
            .map(|data| {
                let len = data.len();
                if len == 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be greater than 0"
                    ))));
                }

                if len % 6 != 0 {
                    return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TIE_POINT_TAG} must be divisible by 6"
                    ))));
                }

                Ok(data)
            })
            .transpose()?;
        let transformation_matrix = model_transformation_data
            .map(|data| {
                <[f64; 16]>::try_from(data).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Number of values in {MODEL_TRANSFORMATION_TAG} must be equal to 16"
                    )))
                })
            })
            .transpose()?;

        if let Some(transformation_matrix) = transformation_matrix {
            if pixel_scale.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_PIXEL_SCALE_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }
            if tie_points.is_some() {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must not be specified when {MODEL_TRANSFORMATION_TAG} is present"),
                )));
            }

            Ok(CoordinateTransform::AffineTransform(
                AffineTransform::from_tag_matrix(transformation_matrix)?,
            ))
        } else {
            let Some(tie_points) = tie_points else {
                return Err(TiffError::FormatError(TiffFormatError::Format(
                    format!("{MODEL_TIE_POINT_TAG} must be present when {MODEL_TRANSFORMATION_TAG} is missing"),
                )));
            };

            if tie_points.len() == 6 {
                let Some(pixel_scale) = pixel_scale else {
                    return Err(TiffError::FormatError(TiffFormatError::Format(
                        format!("{MODEL_PIXEL_SCALE_TAG} must be specified when {MODEL_TIE_POINT_TAG} contains 6 values"),
                    )));
                };

                Ok(CoordinateTransform::TiePointAndPixelScale(
                    TiePointAndPixelScale::from_tag_data(&tie_points, &pixel_scale),
                ))
            } else {
                #[cfg(feature = "tie-points")]
                {
                    Ok(CoordinateTransform::TiePoints(
                        tie_points::TiePoints::from_tie_points(&tie_points),
                    ))
                }
                #[cfg(not(feature = "tie-points"))]
                {
                    Err(TiffError::FormatError(TiffFormatError::Format(
                        "Transformation by tie points is not supported".into(),
                    )))
                }
            }
        }
    }

    pub fn transform_to_model(&self, coord: &Coord) -> Coord {
        match self {
            CoordinateTransform::AffineTransform(transform) => transform.to_model(coord),
            CoordinateTransform::TiePointAndPixelScale(transform) => transform.to_model(coord),
            #[cfg(feature = "tie-points")]
            CoordinateTransform::TiePoints(transform) => transform.to_model(coord),
        }
    }

    pub(super) fn transform_to_raster(&self, coord: &Coord) -> Coord {
        match self {
            CoordinateTransform::AffineTransform(transform) => transform.to_raster(coord),
            CoordinateTransform::TiePointAndPixelScale(transform) => transform.to_raster(coord),
            #[cfg(feature = "tie-points")]
            CoordinateTransform::TiePoints(transform) => transform.to_raster(coord),
        }
    }
}

#[derive(Debug)]
pub struct AffineTransform {
    transform: [f64; 6],
    inverse_transform: [f64; 6],
}

impl AffineTransform {
    pub fn from_tag_matrix(matrix: [f64; 16]) -> TiffResult<Self> {
        let transform = [
            matrix[0], matrix[1], matrix[3], matrix[4], matrix[5], matrix[7],
        ];

        let det = transform[0] * transform[4] - transform[1] * transform[3];
        if det.abs() < 0.000000000000001 {
            return Err(TiffError::FormatError(TiffFormatError::Format(
                String::from("Provided transformation matrix is not invertible"),
            )));
        }

        let inverse_transform = [
            transform[4] / det,
            -transform[1] / det,
            (transform[1] * transform[5] - transform[2] * transform[4]) / det,
            -transform[3] / det,
            transform[0] / det,
            (-transform[0] * transform[5] + transform[2] * transform[3]) / det,
        ];

        Ok(AffineTransform {
            transform,
            inverse_transform,
        })
    }

    fn transform(matrix: &[f64; 6], coord: &Coord) -> Coord {
        Coord {
            x: coord.x * matrix[0] + coord.y * matrix[1] + matrix[2],
            y: coord.x * matrix[3] + coord.y * matrix[4] + matrix[5],
        }
    }

    pub fn to_model(&self, coord: &Coord) -> Coord {
        Self::transform(&self.transform, coord)
    }

    pub fn to_raster(&self, coord: &Coord) -> Coord {
        Self::transform(&self.inverse_transform, coord)
    }
}

#[derive(Debug)]
pub struct TiePointAndPixelScale {
    raster_point: Coord,
    model_point: Coord,
    pixel_scale: Coord,
}

impl TiePointAndPixelScale {
    pub fn from_tag_data(tie_points: &[f64], pixel_scale: &[f64]) -> Self {
        TiePointAndPixelScale {
            raster_point: Coord {
                x: tie_points[0],
                y: tie_points[1],
            },
            model_point: Coord {
                x: tie_points[3],
                y: tie_points[4],
            },
            pixel_scale: Coord {
                x: pixel_scale[0],
                y: pixel_scale[1],
            },
        }
    }

    pub fn to_model(&self, coord: &Coord) -> Coord {
        Coord {
            x: (coord.x - self.raster_point.x) * self.pixel_scale.x + self.model_point.x,
            y: (coord.y - self.raster_point.y) * -self.pixel_scale.y + self.model_point.y,
        }
    }

    pub fn to_raster(&self, coord: &Coord) -> Coord {
        Coord {
            x: (coord.x - self.model_point.x) / self.pixel_scale.x + self.raster_point.x,
            y: (coord.y - self.model_point.y) / -self.pixel_scale.y + self.raster_point.y,
        }
    }
}
