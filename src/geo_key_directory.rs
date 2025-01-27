use num_enum::{IntoPrimitive, TryFromPrimitive};
use tiff::tags::Tag;
use tiff::{TiffError, TiffFormatError, TiffResult};

/// The GeoKeyDirectoryTag Requirements Class specifies the requirements for
/// implementing the reserved GeoKeyDirectoryTag TIFF tag.
///
/// Ref: https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_geokeydirectorytag
#[derive(Debug, PartialEq)]
pub struct GeoKeyDirectory {
    pub key_directory_version: u16,
    pub key_revision: u16,
    pub minor_revision: u16,
    pub model_type: Option<u16>,
    pub raster_type: Option<RasterType>,
    pub citation: Option<String>,
    pub geographic_type: Option<u16>,
    pub geog_citation: Option<String>,
    pub geog_geodetic_datum: Option<u16>,
    pub geog_prime_meridian: Option<u16>,
    pub geog_linear_units: Option<u16>,
    pub geog_linear_unit_size: Option<f64>,
    pub geog_angular_units: Option<u16>,
    pub geog_angular_unit_size: Option<f64>,
    pub geog_ellipsoid: Option<u16>,
    pub geog_semi_major_axis: Option<f64>,
    pub geog_semi_minor_axis: Option<f64>,
    pub geog_inv_flattening: Option<f64>,
    pub geog_azimuth_units: Option<u16>,
    pub geog_prime_meridian_long: Option<f64>,
    pub projected_type: Option<u16>,
    pub proj_citation: Option<String>,
    pub projection: Option<u16>,
    pub proj_coord_trans: Option<u16>,
    pub proj_linear_units: Option<u16>,
    pub proj_linear_unit_size: Option<f64>,
    pub proj_std_parallel1: Option<f64>,
    pub proj_std_parallel2: Option<f64>,
    pub proj_nat_origin_long: Option<f64>,
    pub proj_nat_origin_lat: Option<f64>,
    pub proj_false_easting: Option<f64>,
    pub proj_false_northing: Option<f64>,
    pub proj_false_origin_long: Option<f64>,
    pub proj_false_origin_lat: Option<f64>,
    pub proj_false_origin_easting: Option<f64>,
    pub proj_false_origin_northing: Option<f64>,
    pub proj_center_long: Option<f64>,
    pub proj_center_lat: Option<f64>,
    pub proj_center_easting: Option<f64>,
    pub proj_center_northing: Option<f64>,
    pub proj_scale_at_nat_origin: Option<f64>,
    pub proj_scale_at_center: Option<f64>,
    pub proj_azimuth_angle: Option<f64>,
    pub proj_straight_vert_pole_long: Option<f64>,
    pub vertical: Option<u16>,
    pub vertical_citation: Option<String>,
    pub vertical_datum: Option<u16>,
    pub vertical_units: Option<u16>,
}

impl GeoKeyDirectory {
    pub fn from_tag_data(
        directory_data: &[u16],
        double_params: &[f64],
        ascii_params: &str,
    ) -> TiffResult<Self> {
        let mut directory = Self::default();
        if directory_data.len() < 4 {
            return Err(TiffError::FormatError(TiffFormatError::Format(
                "Unexpected length of directory data: must be at least 4.".into(),
            )));
        }

        directory.key_directory_version = directory_data[0];
        directory.key_revision = directory_data[1];
        directory.minor_revision = directory_data[2];
        let number_of_keys = directory_data[3] as usize;

        if directory_data.len() - 4 != 4 * number_of_keys {
            return Err(TiffError::FormatError(TiffFormatError::Format(
                "Unexpected length of directory data: number of keys does not match length of directory data.".into())
            ));
        }

        for entry in directory_data[4..].chunks_exact(4) {
            let entry = DirectoryEntry {
                key_tag: GeoKeyDirectoryTag::try_from(entry[0]).map_err(|_| {
                    TiffError::FormatError(TiffFormatError::Format(format!(
                        "Unknown GeoKeyDirectoryTag: {}",
                        entry[0]
                    )))
                })?,
                location_tag: Tag::from_u16(entry[1]),
                count: entry[2],
                value_or_offset: entry[3],
            };

            match entry.key_tag {
                GeoKeyDirectoryTag::ModelType => directory.model_type = Some(entry.short()?),
                GeoKeyDirectoryTag::RasterType => {
                    let raster_type = entry.short()?;
                    directory.raster_type =
                        Some(RasterType::try_from(raster_type).map_err(|_| {
                            TiffError::FormatError(TiffFormatError::Format(format!(
                                "Unknown raster type: {raster_type}"
                            )))
                        })?)
                }
                GeoKeyDirectoryTag::Citation => {
                    directory.citation = Some(entry.string(ascii_params)?)
                }
                GeoKeyDirectoryTag::GeographicType => {
                    directory.geographic_type = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogCitation => {
                    directory.geog_citation = Some(entry.string(ascii_params)?)
                }
                GeoKeyDirectoryTag::GeogGeodeticDatum => {
                    directory.geog_geodetic_datum = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogPrimeMeridian => {
                    directory.geog_prime_meridian = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogLinearUnits => {
                    directory.geog_linear_units = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogLinearUnitSize => {
                    directory.geog_linear_unit_size = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::GeogAngularUnits => {
                    directory.geog_angular_units = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogAngularUnitSize => {
                    directory.geog_angular_unit_size = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::GeogEllipsoid => {
                    directory.geog_ellipsoid = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogSemiMajorAxis => {
                    directory.geog_semi_major_axis = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::GeogSemiMinorAxis => {
                    directory.geog_semi_minor_axis = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::GeogInvFlattening => {
                    directory.geog_inv_flattening = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::GeogAzimuthUnits => {
                    directory.geog_azimuth_units = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::GeogPrimeMeridianLong => {
                    directory.geog_prime_meridian_long = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjectedType => {
                    directory.projected_type = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::ProjCitation => {
                    directory.proj_citation = Some(entry.string(ascii_params)?)
                }
                GeoKeyDirectoryTag::Projection => directory.projection = Some(entry.short()?),
                GeoKeyDirectoryTag::ProjCoordTrans => {
                    directory.proj_coord_trans = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::ProjLinearUnits => {
                    directory.proj_linear_units = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::ProjLinearUnitSize => {
                    directory.proj_linear_unit_size = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjStdParallel1 => {
                    directory.proj_std_parallel1 = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjStdParallel2 => {
                    directory.proj_std_parallel2 = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjNatOriginLong => {
                    directory.proj_nat_origin_long = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjNatOriginLat => {
                    directory.proj_nat_origin_lat = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseEasting => {
                    directory.proj_false_easting = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseNorthing => {
                    directory.proj_false_northing = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseOriginLong => {
                    directory.proj_false_origin_long = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseOriginLat => {
                    directory.proj_false_origin_lat = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseOriginEasting => {
                    directory.proj_false_origin_easting = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjFalseOriginNorthing => {
                    directory.proj_false_origin_northing = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjCenterLong => {
                    directory.proj_center_long = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjCenterLat => {
                    directory.proj_center_lat = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjCenterEasting => {
                    directory.proj_center_easting = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjCenterNorthing => {
                    directory.proj_center_northing = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjScaleAtNatOrigin => {
                    directory.proj_scale_at_nat_origin = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjScaleAtCenter => {
                    directory.proj_scale_at_center = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjAzimuthAngle => {
                    directory.proj_azimuth_angle = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::ProjStraightVertPoleLong => {
                    directory.proj_straight_vert_pole_long = Some(entry.double(double_params)?)
                }
                GeoKeyDirectoryTag::Vertical => directory.vertical = Some(entry.short()?),
                GeoKeyDirectoryTag::VerticalCitation => {
                    directory.vertical_citation = Some(entry.string(ascii_params)?)
                }
                GeoKeyDirectoryTag::VerticalDatum => {
                    directory.vertical_datum = Some(entry.short()?)
                }
                GeoKeyDirectoryTag::VerticalUnits => {
                    directory.vertical_units = Some(entry.short()?)
                }
            }
        }

        Ok(directory)
    }
}

struct DirectoryEntry {
    key_tag: GeoKeyDirectoryTag,
    location_tag: Option<Tag>,
    count: u16,
    value_or_offset: u16,
}

impl DirectoryEntry {
    fn short(&self) -> TiffResult<u16> {
        // Check that TIFFTagLocation == 0 so value is of SHORT type
        if self.location_tag.is_some() {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Key `{:?}` did not have the expected SHORT value type.",
                self.key_tag
            ))));
        }

        if self.count != 1 {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Unexpected count: expected 1, got {}.",
                self.count
            ))));
        }

        Ok(self.value_or_offset)
    }

    fn double(&self, data: &[f64]) -> TiffResult<f64> {
        if self.location_tag != Some(Tag::GeoDoubleParamsTag) {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Key `{:?}` did not have the expected DOUBLE value type.",
                self.key_tag
            ))));
        }

        if self.count != 1 {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Unexpected count: expected 1, got {}.",
                self.count
            ))));
        }

        match data.get(self.value_or_offset as usize) {
            None => Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Offset out of bounds: the length is {} but the offset is {}",
                data.len(),
                self.value_or_offset
            )))),
            Some(value) => Ok(*value),
        }
    }

    fn string(&self, data: &str) -> TiffResult<String> {
        if self.location_tag != Some(Tag::GeoAsciiParamsTag) {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Key `{:?}` did not have the expected ASCII value type.",
                self.key_tag
            ))));
        }

        let start = self.value_or_offset as usize;
        if start >= data.len() {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "Start offset out of bounds: the length is {} but the offset is {}.",
                data.len(),
                self.value_or_offset
            ))));
        }

        let end = (self.value_or_offset + self.count - 1) as usize;
        if end >= data.len() {
            return Err(TiffError::FormatError(TiffFormatError::Format(format!(
                "End offset out of bounds: the length is {} but the offset is {}.",
                data.len(),
                self.value_or_offset
            ))));
        }

        Ok(String::from(&data[start..end]))
    }
}

impl Default for GeoKeyDirectory {
    fn default() -> Self {
        // According to https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_geokeydirectorytag,
        // - the value of KeyDirectoryVersion SHALL be 1
        // - the value of KeyRevision SHALL be 1
        // - a MinorRevision of 1 indicates GeoTIFF 1.1, which is recommended for production/writing a GeoTIFF file
        Self {
            key_directory_version: 1,
            key_revision: 1,
            minor_revision: 1,
            model_type: None,
            raster_type: None,
            citation: None,
            geographic_type: None,
            geog_citation: None,
            geog_geodetic_datum: None,
            geog_prime_meridian: None,
            geog_linear_units: None,
            geog_linear_unit_size: None,
            geog_angular_units: None,
            geog_angular_unit_size: None,
            geog_ellipsoid: None,
            geog_semi_major_axis: None,
            geog_semi_minor_axis: None,
            geog_inv_flattening: None,
            geog_azimuth_units: None,
            geog_prime_meridian_long: None,
            projected_type: None,
            proj_citation: None,
            projection: None,
            proj_coord_trans: None,
            proj_linear_units: None,
            proj_linear_unit_size: None,
            proj_std_parallel1: None,
            proj_std_parallel2: None,
            proj_nat_origin_long: None,
            proj_nat_origin_lat: None,
            proj_false_easting: None,
            proj_false_northing: None,
            proj_false_origin_long: None,
            proj_false_origin_lat: None,
            proj_false_origin_easting: None,
            proj_false_origin_northing: None,
            proj_center_long: None,
            proj_center_lat: None,
            proj_center_easting: None,
            proj_center_northing: None,
            proj_scale_at_nat_origin: None,
            proj_scale_at_center: None,
            proj_azimuth_angle: None,
            proj_straight_vert_pole_long: None,
            vertical: None,
            vertical_citation: None,
            vertical_datum: None,
            vertical_units: None,
        }
    }
}

/// GeoTIFF key names and IDs.
///
/// Ref: https://docs.ogc.org/is/19-008r4/19-008r4.html#_summary_of_geokey_ids_and_names
#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
enum GeoKeyDirectoryTag {
    // GeoTIFF configuration keys
    ModelType = 1024,
    RasterType = 1025,
    Citation = 1026,

    // Geodetic CRS Parameter Keys
    GeographicType = 2048,
    GeogCitation = 2049,
    GeogGeodeticDatum = 2050,
    GeogPrimeMeridian = 2051,
    GeogLinearUnits = 2052,
    GeogLinearUnitSize = 2053,
    GeogAngularUnits = 2054,
    GeogAngularUnitSize = 2055,
    GeogEllipsoid = 2056,
    GeogSemiMajorAxis = 2057,
    GeogSemiMinorAxis = 2058,
    GeogInvFlattening = 2059,
    GeogAzimuthUnits = 2060,
    GeogPrimeMeridianLong = 2061,

    // Projected CRS Parameter Keys
    ProjectedType = 3072,
    ProjCitation = 3073,
    Projection = 3074,
    ProjCoordTrans = 3075,
    ProjLinearUnits = 3076,
    ProjLinearUnitSize = 3077,
    ProjStdParallel1 = 3078,
    ProjStdParallel2 = 3079,
    ProjNatOriginLong = 3080,
    ProjNatOriginLat = 3081,
    ProjFalseEasting = 3082,
    ProjFalseNorthing = 3083,
    ProjFalseOriginLong = 3084,
    ProjFalseOriginLat = 3085,
    ProjFalseOriginEasting = 3086,
    ProjFalseOriginNorthing = 3087,
    ProjCenterLong = 3088,
    ProjCenterLat = 3089,
    ProjCenterEasting = 3090,
    ProjCenterNorthing = 3091,
    ProjScaleAtNatOrigin = 3092,
    ProjScaleAtCenter = 3093,
    ProjAzimuthAngle = 3094,
    ProjStraightVertPoleLong = 3095,

    // Vertical CRS Parameter Keys (4096-5119)
    Vertical = 4096,
    VerticalCitation = 4097,
    VerticalDatum = 4098,
    VerticalUnits = 4099,
}

/// The raster type establishes the raster space used.
///
/// Ref: https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_gtrastertypegeokey
#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum RasterType {
    Undefined = 0,
    RasterPixelIsArea = 1,
    RasterPixelIsPoint = 2,
    UserDefined = 32767,
}
