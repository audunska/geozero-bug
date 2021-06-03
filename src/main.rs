use geo_types::{Geometry, GeometryCollection, Point};
use geozero::GeozeroGeometry;
use geozero::ToGeo;

fn main() {
    let geo = Geometry::GeometryCollection(GeometryCollection(vec![
        Geometry::Point(Point::new(0., 0.)),
        Geometry::Point(Point::new(0., 1.)),
    ]));

    // Write geometry to wkb
    let mut buf = Vec::new();
    let mut writer = geozero::wkb::WkbWriter::new(&mut buf, geozero::wkb::WkbDialect::Ewkb);
    geo.process_geom(&mut writer).unwrap();

    // Read wkb to geometry
    let wkb = geozero::wkb::Ewkb(buf);
    let geo2 = wkb.to_geo().unwrap();
    // This fails because geo2 is only POINT(0 1)
    assert_eq!(geo, geo2)
}
