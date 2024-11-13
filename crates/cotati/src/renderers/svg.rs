//! An implementation of [`Renderer`](super::Renderer) that supports svg image rendering target.

use xml_dom::level2::{get_implementation, Document, Element, Node, RefNode};

use crate::{Error, Result};

use super::Renderer;

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum ScopeInstructon {
    Canvas(RefNode),
    Path,
    Transform(RefNode),
    Fill(RefNode),
    Stroke(RefNode),
    Entity(String, RefNode),
}

struct RawSvgRenderer {
    document: RefNode,
    scoped_instructions: Vec<ScopeInstructon>,
}

#[allow(unused)]
impl RawSvgRenderer {
    fn new() -> Result<Self> {
        let implementation = get_implementation();

        let document =
            implementation.create_document(Some("http://www.w3.org/2000/svg"), None, None)?;

        Ok(Self {
            document,
            scoped_instructions: Default::default(),
        })
    }

    fn get_parent(&mut self) -> &mut RefNode {
        match self.scoped_instructions.last_mut() {
            Some(ScopeInstructon::Entity(_, node)) => node,
            Some(ScopeInstructon::Canvas(node)) => node,
            Some(ScopeInstructon::Transform(node)) => node,
            Some(ScopeInstructon::Fill(node)) => node,
            Some(ScopeInstructon::Stroke(node)) => node,
            _ => &mut self.document,
        }
    }

    fn is_in_path(&self) -> bool {
        self.scoped_instructions.last() == Some(&ScopeInstructon::Path)
    }

    fn pop(&mut self, n: usize) -> Result<()> {
        if self.scoped_instructions.len() < n {
            return Err(Error::Pop(n));
        }

        for _ in 0..n {
            self.scoped_instructions.pop();
        }

        Ok(())
    }

    fn push_entity(&mut self, id: &str) -> Result<()> {
        let implementation = get_implementation();

        let document =
            implementation.create_document(Some("http://www.w3.org/2000/svg"), None, None)?;

        self.scoped_instructions
            .push(ScopeInstructon::Entity(id.to_owned(), document));

        Ok(())
    }

    fn push_canvas(&mut self, canvas: crate::Canvas) -> Result<()> {
        let parent = self.get_parent();

        let mut el = parent.create_element("svg")?;

        el.set_attribute("width", canvas.width.to_string().as_str())?;

        el.set_attribute("height", canvas.height.to_string().as_str())?;

        if let Some(viewbox) = canvas.viewbox {
            el.set_attribute(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    viewbox.x, viewbox.y, viewbox.width, viewbox.height
                )
                .as_str(),
            )?;

            if let Some(aspect) = viewbox.aspect {
                el.set_attribute("preserveAspectRatio", aspect.to_string().as_str())?;
            }
        }

        self.scoped_instructions.push(ScopeInstructon::Canvas(el));

        Ok(())
    }

    fn push_path(&mut self) -> Result<()> {
        self.scoped_instructions.push(ScopeInstructon::Path);
        Ok(())
    }

    fn push_transform(&mut self, transform: crate::Transform) -> Result<()> {
        let parent = self.get_parent();

        let mut el = parent.create_element("g")?;

        el.set_attribute("transform", transform.to_string().as_str())?;

        self.scoped_instructions
            .push(ScopeInstructon::Transform(el));

        Ok(())
    }

    fn push_fill(&mut self, color: crate::Rgba) -> Result<()> {
        let parent = self.get_parent();

        let mut el = parent.create_element("g")?;

        el.set_attribute("color", color.to_string().as_str())?;

        self.scoped_instructions.push(ScopeInstructon::Fill(el));

        Ok(())
    }

    fn push_stroke(&mut self, color: crate::Rgba, width: crate::Length) -> Result<()> {
        let parent = self.get_parent();

        let mut el = parent.create_element("g")?;

        el.set_attribute("stroke", color.to_string().as_str())?;
        el.set_attribute("stroke-width", width.to_string().as_str())?;

        self.scoped_instructions.push(ScopeInstructon::Stroke(el));

        Ok(())
    }

    fn push_label(&mut self, _label: &str) -> Result<()> {
        Ok(())
    }

    fn entity_ref(&mut self, id: &str) -> Result<()> {
        let parent = self.get_parent();

        let entity_ref = parent.create_entity_reference(id)?;

        parent.append_child(entity_ref);

        Ok(())
    }

    fn move_to(&mut self, to: crate::Point) -> Result<()> {
        todo!()
    }

    fn line(&mut self, from: Option<crate::Point>, to: crate::Point) -> Result<()> {
        todo!()
    }

    fn quadratic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl: crate::Point,
        to: crate::Point,
    ) -> Result<()> {
        todo!()
    }

    fn cubic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl1: crate::Point,
        ctrl2: crate::Point,
        to: crate::Point,
    ) -> Result<()> {
        todo!()
    }

    fn arc(
        &mut self,
        center: Option<crate::Point>,
        raddii: (crate::Length, crate::Length),
        start_angle: crate::Angle,
        sweep_angle: crate::Angle,
        x_rotation: crate::Angle,
    ) -> Result<()> {
        todo!()
    }
}

pub struct SvgRenderer {
    raw: Option<RawSvgRenderer>,
    error: Option<Error>,
    svg: Option<String>,
}

impl SvgRenderer {
    /// Create a new svg renderer.
    pub fn new() -> Result<Self> {
        Ok(Self {
            raw: Some(RawSvgRenderer::new()?),
            error: None,
            svg: None,
        })
    }

    /// Serialize svg into xml string.
    pub fn to_string(self) -> Option<String> {
        self.svg
    }

    fn pcall<F>(&mut self, f: F)
    where
        F: FnOnce(RawSvgRenderer) -> Result<RawSvgRenderer>,
    {
        if self.error.is_none() {
            match f(self.raw.take().unwrap()) {
                Ok(raw) => self.raw = Some(raw),
                Err(err) => self.error = Some(err),
            }
        }
    }
}

#[allow(unused)]
impl Renderer for SvgRenderer {
    type Error = Error;

    fn clear(&mut self) {
        self.svg = None;
        match RawSvgRenderer::new() {
            Ok(raw) => self.raw = Some(raw),
            Err(err) => self.error = Some(err),
        }
    }

    fn pop(&mut self, n: usize) {
        self.pcall(|mut raw| {
            raw.pop(n)?;

            Ok(raw)
        });
    }

    fn push_entity(&mut self, id: &str) {
        self.pcall(|mut raw| {
            raw.push_entity(id)?;

            Ok(raw)
        });
    }

    fn push_canvas(&mut self, canvas: crate::Canvas) {
        self.pcall(|mut raw| {
            raw.push_canvas(canvas)?;

            Ok(raw)
        });
    }

    fn push_path(&mut self) {
        self.pcall(|mut raw| {
            raw.push_path()?;

            Ok(raw)
        });
    }

    fn push_transform(&mut self, transform: crate::Transform) {
        self.pcall(|mut raw| {
            raw.push_transform(transform)?;

            Ok(raw)
        });
    }

    fn push_fill(&mut self, color: crate::Rgba) {
        self.pcall(|mut raw| {
            raw.push_fill(color)?;

            Ok(raw)
        });
    }

    fn push_stroke(&mut self, color: crate::Rgba, width: crate::Length) {
        self.pcall(|mut raw| {
            raw.push_stroke(color, width)?;

            Ok(raw)
        });
    }

    fn push_label(&mut self, label: &str) {
        self.pcall(|mut raw| {
            raw.push_label(label)?;

            Ok(raw)
        });
    }

    fn entity_ref(&mut self, id: &str) {
        self.pcall(|mut raw| {
            raw.entity_ref(id)?;

            Ok(raw)
        });
    }

    fn move_to(&mut self, to: crate::Point) {
        self.pcall(|mut raw| {
            raw.move_to(to)?;

            Ok(raw)
        });
    }

    fn line(&mut self, from: Option<crate::Point>, to: crate::Point) {
        self.pcall(|mut raw| {
            raw.line(from, to)?;

            Ok(raw)
        });
    }

    fn quadratic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl: crate::Point,
        to: crate::Point,
    ) {
        self.pcall(|mut raw| {
            raw.quadratic_bezier(from, ctrl, to)?;

            Ok(raw)
        });
    }

    fn cubic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl1: crate::Point,
        ctrl2: crate::Point,
        to: crate::Point,
    ) {
        self.pcall(|mut raw| {
            raw.cubic_bezier(from, ctrl1, ctrl2, to)?;

            Ok(raw)
        });
    }

    fn arc(
        &mut self,
        center: Option<crate::Point>,
        raddii: (crate::Length, crate::Length),
        start_angle: crate::Angle,
        sweep_angle: crate::Angle,
        x_rotation: crate::Angle,
    ) {
        self.pcall(|mut raw| {
            raw.arc(center, raddii, start_angle, sweep_angle, x_rotation)?;

            Ok(raw)
        });
    }

    fn submit(&mut self) -> std::result::Result<(), Self::Error> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }

        let raw = self.raw.take().unwrap();

        self.svg = Some(raw.document.to_string());

        Ok(())
    }
}
