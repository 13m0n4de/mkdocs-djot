use jotdown::Render;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet};

struct TocToken {
    id: String,
    name: String,
    level: u16,
    child_indices: Vec<usize>,
}

impl TocToken {
    fn new(id: String, level: u16) -> Self {
        Self {
            id,
            name: String::new(),
            level,
            child_indices: Vec::new(),
        }
    }

    fn to_py_dict<'py>(&self, py: Python<'py>, arena: &[TocToken]) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("id", &self.id)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("level", self.level)?;

        let children = PyList::empty(py);
        for &child_idx in &self.child_indices {
            children.append(arena[child_idx].to_py_dict(py, arena)?)?;
        }
        dict.set_item("children", children)?;

        Ok(dict)
    }
}

struct TocBuilder {
    arena: Vec<TocToken>,
    stack: Vec<usize>,
    root_indices: Vec<usize>,
}

impl TocBuilder {
    fn new() -> Self {
        Self {
            arena: Vec::new(),
            stack: Vec::with_capacity(6),
            root_indices: Vec::new(),
        }
    }

    fn add_token(&mut self, id: String, level: u16) -> usize {
        let idx = self.arena.len();
        let token = TocToken::new(id, level);
        self.arena.push(token);

        self.stack
            .retain(|&parent_idx| self.arena[parent_idx].level < level);

        if let Some(&parent_idx) = self.stack.last() {
            self.arena[parent_idx].child_indices.push(idx);
        } else {
            self.root_indices.push(idx);
        }

        self.stack.push(idx);
        idx
    }

    fn set_name(&mut self, idx: usize, name: String) {
        self.arena[idx].name = name;
    }

    fn to_py_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty(py);
        for &idx in &self.root_indices {
            list.append(self.arena[idx].to_py_dict(py, &self.arena)?)?;
        }
        Ok(list)
    }
}

struct PageMetadata {
    title: Option<String>,
    toc_builder: TocBuilder,
    anchors: Vec<String>,
}

impl PageMetadata {
    fn new() -> Self {
        Self {
            title: None,
            toc_builder: TocBuilder::new(),
            anchors: Vec::new(),
        }
    }

    fn to_py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let anchors = PySet::empty(py)?;
        for anchor in &self.anchors {
            anchors.add(anchor)?;
        }

        let result = PyDict::new(py);
        result.set_item("title", &self.title)?;
        result.set_item("toc_tokens", self.toc_builder.to_py_list(py)?)?;
        result.set_item("anchors", anchors)?;

        Ok(result)
    }
}

struct HeadingContext {
    active_heading_level: Option<u16>,
    active_token_idx: Option<usize>,
    heading_text: String,
    found_title: bool,
}

impl HeadingContext {
    fn new() -> Self {
        Self {
            active_heading_level: None,
            active_token_idx: None,
            heading_text: String::with_capacity(64),
            found_title: false,
        }
    }

    fn start_heading(&mut self, metadata: &mut PageMetadata, level: u16, id: &str) {
        if !id.is_empty() {
            metadata.anchors.push(id.to_string());
        }

        let idx = metadata.toc_builder.add_token(id.to_string(), level);
        self.active_heading_level = Some(level);
        self.active_token_idx = Some(idx);
        self.heading_text.clear();
    }

    fn append_text(&mut self, text: &str) {
        self.heading_text.push_str(text);
    }

    fn end_heading(&mut self, metadata: &mut PageMetadata) -> Option<String> {
        let level = self.active_heading_level.take()?;
        let idx = self.active_token_idx.take()?;

        metadata
            .toc_builder
            .set_name(idx, self.heading_text.clone());

        if !self.found_title && level == 1 && !self.heading_text.is_empty() {
            self.found_title = true;
            Some(self.heading_text.clone())
        } else {
            None
        }
    }
}

#[pyfunction]
fn extract_metadata<'py>(py: Python<'py>, djot_text: &str) -> PyResult<Bound<'py, PyDict>> {
    let mut metadata = PageMetadata::new();
    let mut heading_ctx = HeadingContext::new();

    for event in jotdown::Parser::new(djot_text) {
        match event {
            jotdown::Event::Start(jotdown::Container::Heading { level, id, .. }, _) => {
                heading_ctx.start_heading(&mut metadata, level, &id);
            }
            jotdown::Event::Str(text) if heading_ctx.active_heading_level.is_some() => {
                heading_ctx.append_text(&text);
            }
            jotdown::Event::End(jotdown::Container::Heading { .. })
                if heading_ctx.active_heading_level.is_some() =>
            {
                if let Some(title) = heading_ctx.end_heading(&mut metadata) {
                    metadata.title = Some(title);
                }
            }

            _ => {}
        }
    }

    metadata.to_py_dict(py)
}

#[pyfunction]
fn render_to_html(djot_text: &str) -> PyResult<String> {
    let mut html = String::new();
    let events = jotdown::Parser::new(djot_text);
    jotdown::html::Renderer::default()
        .push(events, &mut html)
        .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("HTML rendering failed: {e}")))?;

    Ok(html)
}

#[pymodule]
fn jotdown_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(render_to_html, m)?)?;

    Ok(())
}
