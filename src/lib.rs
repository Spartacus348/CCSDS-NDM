/*
Attempt the same thing, but with nested traits

The idea being that inheritance can be approximated by an object holding an instance of the object
it "inherits" from
 */

struct Document{
    elements:[Segment]
}

trait Segment{}

struct SingleSegment{
    header:SegmentHeader,
    body:NPMSegment
}
impl Segment for SingleSegment{}

struct MultiSegment{
    header:SegmentHeader,
    body:[NPMSegment]
}
impl Segment for MultiSegment{}

struct CDMSegment{
    header:SegmentHeader,
    body:CDMBody

}
impl Segment for CDMSegment{}

struct CDMBody{
    relative_metadata:RelativeMetadata,
    segment1:NPMSegment,
    segment2:NPMSegment
}

struct RelativeMetadata{}

struct NPMHeader{
    comments:[Comment],
    creation_date:CreationDate,
    originator:str
}

struct Comment(str);
struct CreationDate{}

struct SegmentHeader{
    tag:str,
    id:str,
    version:str,
}

trait NPMSegment{
    fn get_meta_data(self) -> XmlMap;
    fn get_data(self) -> XmLMap;
}

struct APM{}
impl SingleSegment for APM{}

struct AEM{}
impl MultiSegment for AEM{}

struct OPM{}
impl SingleSegment for OPM{}

struct OMM{}
impl SingleSegment for OMM{}

struct OEM{}
impl MultiSegment for OEM{}

struct OCM{}
impl SingleSegment for OCM{}

struct TDM{}
impl MultiSegment for TDM{}

struct CDM{}
impl TwoSegment for CDM{}

struct RDM{}
impl SingleSegment for RDM{}

trait SingleSegment{}
trait MultiSegment{}
trait TwoSegment{}


