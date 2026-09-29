# Tabler Icons for Slint

Use the [Tabler Icons](https://github.com/tabler/tabler-icons) outline and filled SVG icon sets in [Slint](https://slint.dev) applications. SVGs are preconverted into Slint `Path` elements, so the library does not parse SVG at runtime.

## Install with Cargo

Add the crate as a build dependency:

```toml
[build-dependencies]
tabler-slint = "3.48.0"
slint-build = "1"
```

In `build.rs`, register the library and compile the Slint file:

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let libraries = HashMap::from([(
        "tabler".to_string(),
        PathBuf::from(tabler_slint::lib()),
    )]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(libraries);

    slint_build::compile_with_config("ui/main.slint", config)
        .expect("Slint build failed");
}
```

## Install with CMake

Make the package directory available in your project, then register its library path:

```cmake
include(path/to/tabler-slint/CMakeLists.txt)

slint_target_sources(my_application
    ui/main.slint
    LIBRARY_PATHS ${TablerSlint_LIBRARY_PATHS}
)
```

For other build systems, copy `tabler-slint/tabler.slint` into your project and map the import name `tabler` to that file.

## Use an icon

Import `IconDisplay`, `IconSet`, and `FilledIconSet` from `@tabler`:

```slint
import { IconDisplay, IconSet, FilledIconSet } from "@tabler";

export component Example {
    VerticalLayout {
        IconDisplay {
            icon: IconSet.Home;
            color: #e5e7eb;
        }

        IconDisplay {
            icon: FilledIconSet.Home;
            color: #60a5fa;
        }
    }
}
```

`IconSet` contains the outline SVGs, and `FilledIconSet` contains the filled SVGs. The sets are independent because Tabler uses the same icon names in both variants.

## Icon display properties

| Property | Type | Default | Description |
| --- | --- | --- | --- |
| `icon` | `Icon` | unset | The icon from either set |
| `size` | `length` | `24px` | Icon width and height |
| `color` | `brush` | `white` | Stroke color for outline icons and fill color for filled icons |
| `stroke-width` | `float` | `2` | Stroke width for outline icons |
| `absolute-stroke-width` | `bool` | `false` | Keep stroke width independent from icon size |

## Regenerate

Requirements: Rust toolchain and Node.js/npm.

```sh
npm install
npm run generate
```

The generator reads the official `@tabler/icons` SVG package and writes `tabler-slint/tabler.slint`. Update the npm dependency and matching crate version when syncing to a new Tabler release.

Tabler icon artwork is MIT licensed. See [LICENSE-TABLER](LICENSE-TABLER); library and generator code is MIT licensed as well.
