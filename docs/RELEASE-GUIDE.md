# Release Guide

## Crear release

### 1. Verificar que todo compila

```bash
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### 2. Commit de los cambios

```bash
git add .github/workflows/release.yml CLI.md Cargo.toml README.md cli/README.md cli/src/commands/install.rs docs/INSTALLATION.md
git commit -m "fix: correct binary names, align deps, and remove hardcoded URLs"
```

### 3. Crear tag y push

```bash
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

> El push del tag dispara automáticamente `.github/workflows/release.yml`.

### 4. Verificar el release

1. Ir a GitHub Actions y verificar que el workflow `Release` complete sin errores
2. Ir a la pestaña Releases y verificar que existan estos assets:
   - `rhinolabs-ai-linux-x64`
   - `rhinolabs-ai-linux-arm64`
   - `rhinolabs-ai-darwin-x64`
   - `rhinolabs-ai-darwin-arm64`
   - `rhinolabs-ai-windows-x64.exe`
   - `rlai-linux-x64`
   - `rlai-linux-arm64`
   - `rlai-darwin-x64`
   - `rlai-darwin-arm64`
   - `rlai-windows-x64.exe`
   - `rhinolabs-claude-0.1.0.zip`
   - `SHA256SUMS.txt`
   - GUI artifacts (AppImage, .deb, .dmg, .msi)

### 5. Verificar el binario

```bash
# Descargar y probar (ejemplo macOS ARM)
curl -fsSL <releases-url>/download/v0.1.0/rhinolabs-ai-darwin-arm64 -o rhinolabs-ai
chmod +x rhinolabs-ai
./rhinolabs-ai status
```

---

## Paso 5: Crear el Homebrew tap

### 1. Crear el repo `<owner>/homebrew-tap` en GitHub

### 2. Obtener SHA256 de los binarios

```bash
# Descargar el archivo de checksums del release
curl -fsSL <releases-url>/download/v0.1.0/SHA256SUMS.txt
```

### 3. Crear la formula

```bash
mkdir -p Formula
```

Crear `Formula/rhinolabs-ai.rb` con el siguiente contenido (reemplazar `<sha256-*>` con los hashes reales del paso anterior):

```ruby
class RhinolabsAi < Formula
  desc "Skill, profile, and configuration management for AI coding assistants"
  homepage "<repo-url>"
  version "0.1.0"
  license "PROPRIETARY"

  on_macos do
    if Hardware::CPU.arm?
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-darwin-arm64"
      sha256 "<sha256-darwin-arm64>"
    else
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-darwin-x64"
      sha256 "<sha256-darwin-x64>"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-linux-arm64"
      sha256 "<sha256-linux-arm64>"
    else
      url "<releases-url>/download/v0.1.0/rhinolabs-ai-linux-x64"
      sha256 "<sha256-linux-x64>"
    end
  end

  def install
    bin.install Dir["rhinolabs-ai*"].first => "rhinolabs-ai"
    bin.install_symlink "rhinolabs-ai" => "rlai"
  end

  test do
    system "#{bin}/rhinolabs-ai", "status"
  end
end
```

### 4. Push y verificar

```bash
git add Formula/rhinolabs-ai.rb
git commit -m "feat: add rhinolabs-ai formula v0.1.0"
git push origin main
```

### 5. Probar la instalación

```bash
brew tap <owner>/tap
brew install rhinolabs-ai
rhinolabs-ai status
rlai status
```

---

## Paso 6: Actualizar README con Homebrew funcional

Una vez que el Paso 5 esté verificado, actualizar el Quick Start en `README.md` para que Homebrew sea la opción principal con el owner real.

---

## Releases posteriores

Para releases futuros, repetir desde el paso 3 con la nueva versión:

```bash
# Actualizar version en Cargo.toml si corresponde
git tag v<nueva-version>
git push origin main
git push origin v<nueva-version>
```

Después actualizar la formula en homebrew-tap con las nuevas URLs y SHA256.

---

**Last Updated**: 2026-02-05
