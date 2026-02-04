# Plan: Hacer que las herramientas funcionen de verdad

## Contexto

El código compila, los tests pasan, pero las herramientas NO se pueden instalar ni distribuir. El README documenta cosas que no existen. El repo va a ser migrado a otro propietario, por lo que todas las URLs deben ser relativas.

## Problemas encontrados

### CRÍTICO 1: Nombre de binario incorrecto en release.yml
- `cli/Cargo.toml` define el binario como `rhinolabs-ai` (línea 10)
- `.github/workflows/release.yml` compila `--bin rhinolabs` (línea 77)
- **El release workflow VA A FALLAR** porque ese binario no existe

### CRÍTICO 2: Dependencias de sistema inconsistentes
- `test.yml` usa `libwebkit2gtk-4.1-dev` + `libayatana-appindicator3-dev`
- `release.yml` usa `libwebkit2gtk-4.0-dev` + `libappindicator3-dev`
- El build de la GUI en release puede fallar o producir binarios incompatibles

### CRÍTICO 3: No hay releases publicados
- Nunca se pusheó un tag → el workflow nunca se disparó
- No hay binarios descargables → la instalación no funciona

### CRÍTICO 4: Homebrew tap no existe
- `brew tap rhinolabs/tap` falla porque el repo `<owner>/homebrew-tap` no existe
- No hay formula

### CRÍTICO 5: URLs hardcoded al owner incorrecto
- 11 instancias de `github.com/rhinolabs/rhinolabs-ai` en archivos .md
- 1 instancia en `Cargo.toml` workspace (línea 10: `repository`)
- Todas apuntan a un owner que no es el real y se van a romper en la migración

**Archivos afectados:**
- `README.md` (líneas 198, 489)
- `cli/README.md` (líneas 13, 20, 27, 33, 215, 275, 276)
- `docs/INSTALLATION.md` (líneas 29, 62)
- `Cargo.toml` (línea 10)

---

## Plan de implementación

### Paso 1: Arreglar release.yml

**Archivo:** `.github/workflows/release.yml`

1. Cambiar `--bin rhinolabs` → `--bin rhinolabs-ai` en la línea 77
2. Actualizar `artifact_name: rhinolabs` → `artifact_name: rhinolabs-ai` y `rhinolabs.exe` → `rhinolabs-ai.exe`
3. Actualizar los `asset_name` para que coincidan: `rhinolabs-ai-linux-x64`, `rhinolabs-ai-darwin-arm64`, etc.
4. Alinear dependencias de sistema con test.yml:
   - `libwebkit2gtk-4.0-dev` → `libwebkit2gtk-4.1-dev`
   - `libappindicator3-dev` → `libayatana-appindicator3-dev`
5. Verificar que el alias `rlai` también se incluya en los assets (es un binario separado en `cli/src/bin/rlai.rs`)

### Paso 2: Convertir todas las URLs a relativas

**Archivos a modificar:**

| Archivo | Cambio |
|---------|--------|
| `README.md:198` | `[Releases](https://github.com/rhinolabs/rhinolabs-ai/releases)` → `[Releases](../../releases)` |
| `README.md:489` | `[GitHub Issues](https://github.com/rhinolabs/rhinolabs-ai/issues)` → `[Issues](../../issues)` |
| `cli/README.md:13,20,27` | URLs de descarga de releases → links relativos a `../../releases/latest` |
| `cli/README.md:215` | `git clone https://...` → `git clone` con URL relativa o instrucción genérica |
| `cli/README.md:275,276` | URLs de repo e issues → relativas |
| `docs/INSTALLATION.md:29,62` | `git clone https://...` → instrucción genérica |
| `Cargo.toml:10` | `repository = "https://github.com/rhinolabs/rhinolabs-ai"` → eliminar o dejar vacío (se actualiza post-migración) |

### Paso 3: Actualizar README Quick Start

**Archivo:** `README.md`

Reemplazar el Quick Start actual (líneas 169-183) que tiene `brew tap rhinolabs/tap` inexistente.

Nuevo Quick Start con métodos que SÍ funcionan:

```markdown
### For Team Developers

```bash
# Option 1: Download from releases
# Go to the Releases page and download the binary for your platform

# Option 2: Build from source
git clone <repo-url>
cd rhinolabs-ai/cli
cargo build --release
# Binary at: target/release/rhinolabs-ai

# Option 3: Homebrew (after tap is set up)
brew tap <owner>/tap
brew install rhinolabs-ai
```
```

**Nota:** La sección de homebrew se deja como Option 3 comentando que depende de que el tap esté configurado. Se activa después del Paso 5.

### Paso 4: Crear el primer release

1. Commit de todos los arreglos de los pasos 1-3
2. Tag `v0.1.0`
3. Push del tag → dispara `release.yml`
4. Verificar que el workflow complete exitosamente en GitHub Actions
5. Verificar que los binarios se descarguen y ejecuten correctamente

### Paso 5: Crear el Homebrew tap

1. Crear repo `<owner>/homebrew-tap` en GitHub
2. Crear la formula `Formula/rhinolabs-ai.rb`:

```ruby
class RhinolabsAi < Formula
  desc "Skill, profile, and configuration management for AI coding assistants"
  homepage "<repo-url>"
  version "0.1.0"
  license "PROPRIETARY"

  on_macos do
    if Hardware::CPU.arm?
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-darwin-arm64"
      sha256 "<sha256>"
    else
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-darwin-x64"
      sha256 "<sha256>"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-linux-arm64"
      sha256 "<sha256>"
    else
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-linux-x64"
      sha256 "<sha256>"
    end
  end

  def install
    bin.install "rhinolabs-ai-#{target}" => "rhinolabs-ai"
    # Also create rlai alias
    bin.install_symlink "rhinolabs-ai" => "rlai"
  end

  test do
    system "#{bin}/rhinolabs-ai", "status"
  end
end
```

**Nota:** Las URLs de descarga y SHA256 se llenan DESPUÉS de que el release se publique (Paso 4). Los hashes se obtienen del `SHA256SUMS.txt` que genera el workflow.

3. Verificar: `brew tap <owner>/tap && brew install rhinolabs-ai && rhinolabs-ai status`

### Paso 6: Actualizar README con homebrew funcional

Una vez que el Paso 5 esté verificado, actualizar el Quick Start para que la opción de homebrew sea la principal (sin `<owner>` hardcoded — se pone el owner real post-migración o el actual).

### Paso 7: Actualizar `Last Updated` y limpiar

- `README.md:499` — Actualizar fecha
- Revisar `cli/README.md` para que las instrucciones de descarga coincidan con los nombres de assets reales del release

---

## Orden de ejecución y dependencias

```
Paso 1 (release.yml) ──┐
Paso 2 (URLs)          ├──→ Paso 4 (tag + release) ──→ Paso 5 (homebrew tap) ──→ Paso 6 (README final)
Paso 3 (Quick Start)  ──┘                                                         Paso 7 (cleanup)
```

Pasos 1, 2 y 3 son independientes y se pueden hacer en paralelo.
Paso 4 depende de 1+2+3 (necesita el commit con todos los arreglos).
Paso 5 depende de 4 (necesita los binarios publicados para obtener SHA256).
Pasos 6 y 7 son el cierre.

---

## Archivos que se modifican

| Archivo | Paso |
|---------|------|
| `.github/workflows/release.yml` | 1 |
| `README.md` | 2, 3, 6, 7 |
| `cli/README.md` | 2, 7 |
| `docs/INSTALLATION.md` | 2 |
| `Cargo.toml` | 2 |
| `<owner>/homebrew-tap` (repo nuevo) | 5 |

## Verificación end-to-end

1. `cargo fmt --all -- --check` — formateo limpio
2. `cargo clippy --workspace -- -D warnings` — sin warnings
3. `cargo test --workspace` — todos los tests pasan
4. Push tag `v0.1.0` → verificar que GitHub Actions `release.yml` completa sin errores
5. Descargar binario del release → ejecutar `rhinolabs-ai status` → funciona
6. `brew tap <owner>/tap && brew install rhinolabs-ai` → instalación exitosa
7. `rhinolabs-ai profile list` → comando funciona
8. `rlai status` → alias funciona
9. Verificar que NINGUNA URL en los .md apunte a un owner hardcoded
