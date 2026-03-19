### INCORRECTO (cosas que están mal en nuestros docs)

| # | Problema | Docs afectados |
|---|---------|---------------|
| 1 | **`#[source] source: ScriptObjectRef` no se menciona como requerido** — AGENTS.md dice que TODOS los structs con `#[derive(Script)]` lo necesitan. Nuestro migration-guide dice incorrectamente que structs con `#[deref] view: View` "no necesitan agregarlo" | migration-guide, handbook, language-ref, patterns |
| 2 | **Separadores de argumentos en shaders** — nuestros docs muestran `sdf.box(0. 0. self.rect_size.x self.rect_size.y 4.0)` (espacios), pero AGENTS.md muestra `sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0)` (comas) | migration-guide, language-ref |
| 3 | **`vec2(a, b)` → `vec2(a b)` presentado como regla universal** — pero AGENTS.md muestra `vec2(800, 600)` con comas en DSL. La remoción de comas es solo en shaders, no en todo el DSL | handbook |
| 4 | **PortalList borrow pattern** — nuestro doc usa `.as_portal_list().borrow_mut()` pero AGENTS.md usa `.borrow_mut::<PortalList>()` | patterns |
| 5 | **Hex color rule incompleta** — decimos que `#eee` es seguro porque la `e` no sigue a un dígito, pero `#4466ee`, `#7799ee` también fallan y no están listados | migration-guide |

### FALTANTE (cosas en AGENTS.md que nuestros docs no cubren)

| # | Info faltante | Importancia |
|---|--------------|------------|
| 1 | **`#[repr(C)]` draw shader field ordering** — campos no-instance ANTES de `#[deref]`, instance fields DESPUÉS. Violaciones corrompen el GPU buffer | CRÍTICA |
| 2 | **`script_shader` para custom draw types** y `script_component` para non-widgets vs `register_widget` | ALTA |
| 3 | **`use crate.module.*` NO funciona** en script_mod | ALTA |
| 4 | **`let` bindings no pueden usarse como property values** — hay que instanciarlos con `{}` | ALTA |
| 5 | **No poner comentarios/líneas en blanco antes del primer código** en `script_mod!` | MEDIA |
| 6 | **Debug logging con `~expression`** | MEDIA |
| 7 | **`pub` keyword inválido** en script_mod | MEDIA |
| 8 | **Shader `modf(a,b)` no `mod(a,b)`**, `atan2(y,x)` no `atan(y,x)` | MEDIA |
| 9 | **Texture declarations**: `texture_2d(float)` no `texture2d` | MEDIA |
| 10 | **Enums no expuestos a script** — algunos enums Rust no disponibles, usar defaults | MEDIA |
| 11 | **Script object `map` vs `vec` storage** y template collection pattern con `vm.vec_with()` | MEDIA |
| 12 | **Prelude alias syntax** (`name:mod.path`) | BAJA |
| 13 | **Platform-specific init** en `App::run` (`vm.cx().start_stdin_service()`) | BAJA |
| 14 | **Shader enums** — preferir `match` con `_ =>` | BAJA |

### CORRECTO (lo que sigue bien)

La gran mayoría del contenido está bien:
- `live_design!` → `script_mod!`, preludes, imports
- `Live`/`LiveHook` → `Script`/`ScriptHook`, `#[apply_default]`
- `{{Struct}}` → `#(Struct::register_widget(vm))`
- Theme refs, `+:` merge, `:=` named instances, `@off`/`@on`
- `instance()`/`uniform()`, `pixel: fn()` shader syntax
- `crate_resource("self://...")`, cursor syntax
- `DefaultNone` → `Default` + `#[default]`
- `script_apply_eval!` con `#(expr)`
- Multi-module registration order
- App bootstrapping pattern, `load_all_resources()`
- Cross-module sharing via `mod.*`
