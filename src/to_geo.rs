use crate::error::Error;
use crate::{ConstGeometry, Geom, Geometry as GGeometry};
use geo_types::Geometry;
use std::str::FromStr;
use wkt;

use std::convert::TryFrom;

macro_rules! impl_try_into {
    ($ty_name:ident $(,$lt:lifetime)?) => (
impl<'a, 'b$(,$lt)?> TryFrom<&'b $ty_name<'a$(,$lt)?>> for Geometry<f64> {
    type Error = Error;

    fn try_from(other: &'b $ty_name<'a$(,$lt)?>) -> Result<Geometry<f64>, Self::Error> {
        // This is a first draft, it's very inefficient, we use wkt as a pivot format to
        // translate the geometry.
        // We should at least use wkb, or even better implement a direct translation
        let wkt_str = other.to_wkt()?;
        let wkt_obj = wkt::Wkt::from_str(&wkt_str)
            .map_err(|e| Error::ConversionError(format!("impossible to read wkt: {}", e)))?;

        let o: wkt::Geometry<f64> = wkt_obj.item;

        o.try_into()
            .map_err(|e| Error::ConversionError(format!("impossible to built from wkt: {}", e)))
    }
}
impl<'a$(,$lt)?> TryFrom<$ty_name<'a$(,$lt)?>> for Geometry<f64> {
    type Error = Error;

    fn try_from(other: $ty_name<'a$(,$lt)?>) -> Result<Geometry<f64>, Self::Error> {
        Geometry::try_from(&other)
    }
}
    );
}

impl_try_into!(GGeometry);
impl_try_into!(ConstGeometry, 'c);

#[cfg(test)]
mod test {
    use crate::Geometry as GGeometry;
    use geo_types::{Coordinate, Geometry, LineString, MultiPoint, MultiPolygon, Point, Polygon};
    use std::convert::TryInto;

    fn coords(tuples: Vec<(f64, f64)>) -> Vec<Coordinate<f64>> {
        tuples.into_iter().map(Coordinate::from).collect()
    }

    #[test]
    fn geom_to_geo_polygon() {
        let poly = "MULTIPOLYGON(((0 0, 0 1, 1 1, 1 0, 0 0)))";
        let poly = GGeometry::new_from_wkt(poly).unwrap();

        let geo_polygon: Geometry<f64> = (&poly).try_into().unwrap();

        let exterior = LineString(coords(vec![
            (0., 0.),
            (0., 1.),
            (1., 1.),
            (1., 0.),
            (0., 0.),
        ]));
        let expected_poly = MultiPolygon(vec![Polygon::new(exterior, vec![])]);
        let expected: Geometry<_> = expected_poly.into();
        assert_eq!(expected, geo_polygon);
        // This check is to enforce that `TryFrom` is implemented for both reference and value.
        assert_eq!(expected, poly.try_into().unwrap());
    }

    #[test]
    fn geom_to_geo_multipoint() {
        let mp = "MULTIPOINT (33.6894226736894140 31.2137365763723125, 61.8251328250639602 -15.0881732790307694)";
        let mp = GGeometry::new_from_wkt(mp).unwrap();

        let geo_polygon: Geometry<f64> = (&mp).try_into().unwrap();

        let expected_multipoint = MultiPoint(vec![
            Point(Coordinate::from((33.6894226736894140, 31.2137365763723125))),
            Point(Coordinate::from((
                61.8251328250639602,
                -15.0881732790307694,
            ))),
        ]);
        let expected: Geometry<_> = expected_multipoint.into();
        assert_eq!(expected, geo_polygon);
        // This check is to enforce that `TryFrom` is implemented for both reference and value.
        assert_eq!(expected, mp.try_into().unwrap());
    }
}
