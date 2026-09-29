use std::{
    collections::HashSet,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;
use tera::{Context, Tera};
use usvg::{Node, Options, Tree, tiny_skia_path::PathSegment};

const OUTLINE_SOURCE: &str = "./node_modules/@tabler/icons/icons/outline";
const FILLED_SOURCE: &str = "./node_modules/@tabler/icons/icons/filled";
const OUTPUT: &str = "./tabler-slint/tabler.slint";

#[derive(Serialize)]
struct Icon {
    name_pascal: String,
    paths: Vec<IconPath>,
}

#[derive(Serialize)]
struct IconPath {
    viewbox_x: f32,
    viewbox_y: f32,
    viewbox_width: f32,
    viewbox_height: f32,
    commands: String,
    has_fill: bool,
    has_stroke: bool,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let outline = generate_icons(OUTLINE_SOURCE)?;
    let filled = generate_icons(FILLED_SOURCE)?;

    let mut template = Tera::default();
    template.add_raw_template("tabler", include_str!("../tabler.template"))?;

    let mut context = Context::new();
    context.insert("outline_icons", &outline);
    context.insert("filled_icons", &filled);
    let slint_file = template.render("tabler", &context)?;

    let output_path = Path::new(OUTPUT);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, slint_file)?;

    println!(
        "Generated {} outline icons and {} filled icons in {}",
        outline.len(),
        filled.len(),
        output_path.display()
    );

    Ok(())
}

fn generate_icons(source: &str) -> Result<Vec<Icon>, Box<dyn std::error::Error>> {
    let mut files: Vec<PathBuf> = fs::read_dir(source)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "svg"))
        .collect();
    files.sort();

    let mut names = HashSet::new();
    let mut icons = Vec::with_capacity(files.len());

    for file in files {
        let stem = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("Invalid SVG filename: {}", file.display()))?;
        let name_pascal = to_pascal_case(stem);

        if !names.insert(name_pascal.clone()) {
            return Err(
                format!("Icon name collision after PascalCase conversion: {name_pascal}").into(),
            );
        }

        let content = fs::read_to_string(&file)?;
        let tree = Tree::from_str(&content, &Options::default())
            .map_err(|error| format!("Could not parse {}: {error}", file.display()))?;
        let paths = read_paths_from_tree(&tree);

        icons.push(Icon { name_pascal, paths });
    }

    Ok(icons)
}

fn to_pascal_case(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut uppercase_next = true;

    for ch in input.chars() {
        if ch.is_whitespace() || ch == '-' || ch == '_' {
            uppercase_next = true;
        } else if uppercase_next {
            output.extend(ch.to_uppercase());
            uppercase_next = false;
        } else {
            output.push(ch);
        }
    }

    if output.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        output.insert_str(0, "Icon");
    }

    output
}

fn read_paths_from_tree(tree: &Tree) -> Vec<IconPath> {
    let size = tree.size();
    let width = size.width();
    let height = size.height();
    let mut paths = Vec::new();

    tree.root().children().iter().for_each(|node| {
        if let Node::Path(path) = node {
            let has_fill = path.fill().is_some();
            let has_stroke = path.stroke().is_some();

            // Tabler SVGs include an empty 24x24 sentinel path with no paint.
            // Keep only paths that actually contribute visible fill or stroke.
            if !has_fill && !has_stroke {
                return;
            }

            let bounds = path.abs_bounding_box();
            let viewbox_x = -bounds.left() / width;
            let viewbox_y = -bounds.top() / height;
            let viewbox_width = width + viewbox_x.abs();
            let viewbox_height = height + viewbox_y.abs();

            paths.push(IconPath {
                viewbox_x,
                viewbox_y,
                viewbox_width,
                viewbox_height,
                commands: path_segments_to_string(path.data()),
                has_fill,
                has_stroke,
            });
        }
    });

    paths
}

fn path_segments_to_string(data: &usvg::tiny_skia_path::Path) -> String {
    let mut output = String::new();

    for segment in data.segments() {
        match segment {
            PathSegment::MoveTo(point) => {
                let _ = write!(output, "M {} {} ", point.x, point.y);
            }
            PathSegment::LineTo(point) => {
                let _ = write!(output, "L {} {} ", point.x, point.y);
            }
            PathSegment::QuadTo(control, point) => {
                let _ = write!(
                    output,
                    "Q {} {} {} {} ",
                    control.x, control.y, point.x, point.y
                );
            }
            PathSegment::CubicTo(control1, control2, point) => {
                let _ = write!(
                    output,
                    "C {} {} {} {} {} {} ",
                    control1.x, control1.y, control2.x, control2.y, point.x, point.y
                );
            }
            PathSegment::Close => output.push_str("Z "),
        }
    }

    output.pop();
    output
}
