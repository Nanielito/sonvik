# DOC-04 — Arquitectura de software

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.0 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Drivers

- Igual comportamiento local y en CI.
- Binario portable sin servicio Sonvik.
- Archivos legibles y versionables.
- Errores seguros antes de efectos cuando sea posible.
- Camino futuro a más interfaces sin diseñarlas hoy.

## 2. Decisión estructural

Un package Cargo contiene una biblioteca y un binario:

```text
sonvik/
├── Cargo.toml
├── src/
│   ├── lib.rs       # pipeline reutilizable
│   └── main.rs      # CLI, presentación y exit code
├── examples/
├── tests/
└── docs/
```

`main` depende de la biblioteca; la biblioteca no conoce argumentos, terminales ni exit codes. No hay traits para servicios con una única implementación ni crates por cada módulo.

## 3. Flujo

```text
CLI -> carga/validación -> variables -> request HTTP -> evaluación -> resultado
```

| Responsabilidad | Entrada | Salida |
| --- | --- | --- |
| Carga | paths | documentos deserializados |
| Validación | documentos | modelo válido o errores localizados |
| Resolución | modelo + fuentes | request resuelta o variable ausente |
| HTTP | request + timeout | response metadata o error de transporte |
| Evaluación | status real/esperado | resultado pass/fail |
| Presentación | resultado/error | texto y exit code |

Estas son responsabilidades, no módulos obligatorios. La implementación usa el menor número de archivos que siga siendo legible.

## 4. Límites de confianza

Son no confiables: YAML, variables de entorno, argumentos, DNS, certificados y toda response. Antes de enviar se validan versión, campos, método, URL, placeholders y timeout. Cada archivo declarativo se limita a 1 MiB.

## 5. Seguridad

- Archivos de environment no contienen secretos por convención y documentación.
- Secretos llegan por entorno del proceso.
- `--var` es solo para valores no sensibles.
- Output normal omite headers y bodies.
- Errores mencionan nombres de variables, no valores resueltos.
- No hay hooks, scripts ni evaluación de expresiones.
- Redirects y TLS usan una política explícita y probada.

## 6. Errores

La biblioteca distingue al menos:

- configuración/validación;
- resolución de variables;
- transporte/timeout;
- assertion fallida.

Solo la última representa una ejecución técnicamente válida con verificación negativa. `main` traduce categorías a exit codes según ADR-001.

## 7. Portabilidad y despliegue

El artefacto es un binario por plataforma. CI ejecutará formato, lint, tests y build en runners nativos cuando exista código. No se promete instalador en el MVP; un release puede publicar binarios y checksums.

## 8. Evolución

- Segundo consumidor real → evaluar extraer crate core.
- Segunda salida estructurada real → interfaz mínima de reporter.
- Segundo transporte real → extraer contrato desde ambos.
- Encadenamiento real → diseñar Flow, scopes y captures en otro ADR.

## 9. Decisiones relacionadas

- [ADR-001](../adr/ADR-001-mvp-architecture.md): alcance, package único, CLI y formato.

## 10. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Arquitectura pragmática inicial |
