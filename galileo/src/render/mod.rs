use crate::primitives::{Color, DecodedImage};
use galileo_types::cartesian::impls::contour::Contour;
use galileo_types::cartesian::impls::point::Point2d;
use galileo_types::cartesian::impls::polygon::Polygon;
use galileo_types::cartesian::size::Size;
use maybe_sync::{MaybeSend, MaybeSync};
use nalgebra::Point3;
use std::any::Any;

#[cfg(feature = "wgpu")]
pub mod wgpu;

pub trait Renderer: MaybeSend + MaybeSync {
    fn create_bundle(&self) -> Box<dyn RenderBundle>;
    fn pack_bundle(&self, bundle: Box<dyn RenderBundle>) -> Box<dyn PackedBundle>;

    fn as_any(&self) -> &dyn Any;
}

pub trait Canvas {
    fn size(&self) -> Size;
    fn create_bundle(&self) -> Box<dyn RenderBundle>;
    fn pack_bundle(&self, bundle: Box<dyn RenderBundle>) -> Box<dyn PackedBundle>;
    fn draw_bundles(&mut self, bundles: &[&Box<dyn PackedBundle>]);
}

pub trait RenderBundle {
    fn add_image(
        &mut self,
        image: DecodedImage,
        vertices: [Point2d; 4],
        paint: ImagePaint,
    ) -> usize;
    fn add_points(&mut self, points: &[Point3<f64>], paint: PointPaint);
    fn add_line(&mut self, line: &Contour<Point2d>, paint: LinePaint, resolution: f64) -> usize;
    fn add_polygon(&mut self, polygon: &Polygon<Point2d>, paint: Paint, resolution: f64) -> usize;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn is_empty(&self) -> bool;
}

pub trait PackedBundle: MaybeSend + MaybeSync {
    fn as_any(&self) -> &dyn Any;
    fn unpack(self: Box<Self>) -> Box<dyn UnpackedBundle>;
}

pub trait UnpackedBundle {
    fn modify_line(&mut self, id: usize, paint: LinePaint);
    fn modify_polygon(&mut self, id: usize, paint: Paint);
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

#[derive(Debug, Clone, Copy)]
pub struct Paint {
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct PointPaint {
    pub color: Color,
    pub size: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct LinePaint {
    pub color: Color,
    pub width: f64,
    pub offset: f64,
    pub line_cap: LineCap,
}

#[derive(Debug, Clone, Copy)]
pub struct PolygonPaint {
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
    Round,
    Butt,
}

impl Into<lyon::path::LineCap> for LineCap {
    fn into(self) -> lyon::lyon_tessellation::LineCap {
        match self {
            LineCap::Round => lyon::lyon_tessellation::LineCap::Round,
            LineCap::Butt => lyon::lyon_tessellation::LineCap::Butt,
        }
    }
}

pub struct ImagePaint {
    pub opacity: u8,
}
