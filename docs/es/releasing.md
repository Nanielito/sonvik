# Publicación de Sonvik

`release-plz` prepara versiones desde Conventional Commits. Es propietario del cambio de versión, `CHANGELOG.md`, el tag `v<semver>` y la publicación en crates.io. `cargo-dist` usa ese tag para crear el GitHub Release y adjuntar archivos portables y checksums SHA-256.

## Configuración única

Daniel Ramirez debe completar estos pasos en GitHub y crates.io antes de fusionar el primer PR de release:

1. Iniciar sesión en crates.io con GitHub y verificar el correo electrónico.
2. Crear un token de crates.io de corta duración con permisos `publish-new` y `publish-update`, y guardarlo como el secret de Actions `CARGO_REGISTRY_TOKEN`.
3. Crear un fine-grained personal access token limitado al repositorio `Nanielito/sonvik`, con acceso de lectura y escritura a Contents y Pull requests, y guardarlo como `RELEASE_PLZ_TOKEN`. Los PR creados con este token sí disparan el CI del repositorio.
4. En **Settings → Actions → General**, permitir que GitHub Actions cree pull requests.

El token de crates.io es necesario para la primera publicación porque los crates nuevos no admiten Trusted Publishing. Después de publicar `v0.1.0`, se debe revocar el token, migrar a Trusted Publishing y eliminar `CARGO_REGISTRY_TOKEN` en un cambio separado.

## Flujo de release

1. Un push a `main` ejecuta release-plz, pero no publica commits ordinarios.
2. release-plz abre o actualiza un PR `release-plz-*` con la versión, `Cargo.lock` y changelog propuestos.
3. Se revisa el diff y se espera que el CI pase.
4. Al fusionar ese PR, release-plz publica el crate y crea el tag correspondiente.
5. El tag inicia el workflow de distribución. El GitHub Release se publica solamente cuando todos los builds nativos y jobs de checksums requeridos terminan correctamente.

`release_always = false` limita publicación y creación de tags a commits provenientes del PR de release. Un PR abierto o un push ordinario no publica.

Los archivos de release se construyen para Linux x86_64, macOS Apple Silicon y Windows x86_64. Instaladores, fórmulas de package managers, firma y arquitecturas adicionales no pertenecen al MVP.

## Validación local

Antes de habilitar una publicación:

```bash
cargo package --list
cargo publish --dry-run
dist plan --tag v0.1.0
```

La publicación es permanente: una versión existente no puede sobrescribirse ni eliminarse de crates.io; solamente puede marcarse como retirada.
