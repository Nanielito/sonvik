# Publicación de Sonvik

`release-plz` prepara versiones desde Conventional Commits. Es propietario del cambio de versión, `CHANGELOG.md`, el tag `v<semver>` y la publicación en crates.io. No crea el GitHub Release ni publica binarios.

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

`release_always = false` limita publicación y creación de tags a commits provenientes del PR de release. Un PR abierto o un push ordinario no publica.

No fusionar el primer PR de release hasta que el issue de distribución de binarios esté implementado. Así el tag `v0.1.0` podrá activar el workflow que creará el GitHub Release y adjuntará sus artefactos.

## Validación local

Antes de habilitar una publicación:

```bash
cargo package --list
cargo publish --dry-run
```

La publicación es permanente: una versión existente no puede sobrescribirse ni eliminarse de crates.io; solamente puede marcarse como retirada.
