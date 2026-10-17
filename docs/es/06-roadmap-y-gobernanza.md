# DOC-06 — Roadmap, calidad y gobernanza

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.1 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Estrategia

Entregar verticales ejecutables. El roadmap expresa orden y gates, no fechas ni promesas de versiones futuras.

## 2. Milestones

### M0 — Baseline documental

- Aprobar o corregir DOC-01 a DOC-06 y ADR-001.
- Cerrar decisiones pendientes del schema mínimo.

**Gate:** no hay contradicciones abiertas sobre alcance, CLI, formato ni exit codes.

### M1 — Primer vertical

- Package Rust con lib + bin.
- Carga/validación, variables, HTTP, status assertion y exit codes.
- Test E2E con servidor local.

**Gate:** CU-001, CU-002 y CU-004 pasan offline.

### M2 — Portabilidad y release 0.1.0

- CI en macOS, Linux y Windows.
- CU-003/CU-005, documentación de uso y ejemplos.
- Archivos portables y checksums SHA-256 para Linux x86_64, macOS Apple Silicon y Windows x86_64.

**Gate:** todos los Must del MVP tienen evidencia, los jobs nativos de release producen los tres archivos y checksums requeridos, y no hay secretos en fixtures/output.

### Después de 0.1.0

El siguiente problema se elige con uso real. Candidatos independientes: assertions JSON, collections/flows, salida estructurada, importación o desktop. No se asignan versiones todavía.

## 3. Definición de terminado

Una capacidad está terminada cuando:

- implementa flujo principal y errores relevantes;
- tiene la prueba mínima proporcional al riesgo;
- no filtra secretos;
- funciona offline cuando no necesita una API externa;
- actualiza requisitos, casos, datos y ADRs afectados;
- documenta sus límites visibles.

## 4. Estrategia de pruebas

- Unitarias para validación, precedencia y placeholders.
- Integración para métodos, bodies, status, redirects y timeout.
- Un E2E CLI contra servidor local efímero.
- Casos de seguridad para redacción y ausencia de envío tras error local.
- Smoke builds por sistema operativo.
- Un plan de distribución que valida el tag de release, la matriz objetivo, los archivos y checksums antes de publicar.

No se fija cobertura porcentual: se exige evidencia sobre ramas con comportamiento distinto.

## 5. Riesgos

| ID | Riesgo | Impacto | Tratamiento |
| --- | --- | ---: | --- |
| RSK-001 | Rust ralentiza el primer vertical | Medio | Revisar tras M1; cambiar solo con evidencia |
| RSK-002 | YAML ambiguo o difícil de evolucionar | Alto | Schema estricto, versión y ejemplos inválidos |
| RSK-003 | Fuga de secretos | Alto | Entorno, output mínimo y tests de redacción |
| RSK-004 | Scope crece hacia un clon de Postman | Alto | DOC-01 y gates como filtro |
| RSK-005 | Diferencias HTTP entre plataformas | Medio | Tests de integración y CI nativa |
| RSK-006 | Abstracciones para futuros consumidores | Medio | Extraer solo ante segunda implementación |

## 6. Decisiones resueltas

| ID | Decisión | Resolución |
| --- | --- | --- |
| OQ-006 | Matriz mínima de SO/arquitectura | Linux x86_64, macOS Apple Silicon y Windows x86_64 para releases del MVP |

## 7. Versionado

- Producto y documentos usan SemVer independiente.
- `0.x` permite cambios incompatibles documentados.
- Cada archivo canónico declara versión de schema.
- `CHANGELOG.md` se crea con el primer cambio de producto, no antes.

## 8. ADRs

Ruta: `docs/adr/ADR-NNN-titulo.md`. Estados: Propuesto, Aceptado, Rechazado, Reemplazado, Retirado.

Un ADR incluye contexto, opciones, decisión, consecuencias, validación y condición de revisión. Una decisión aceptada no se reescribe sustancialmente: una nueva la reemplaza.

## 9. Gobernanza documental

1. Español es la fuente canónica; inglés se sincroniza en el mismo cambio.
2. Toda modificación actualiza fecha, versión e historial del documento afectado.
3. IDs retirados no se reutilizan.
4. Un cambio normativo revisa trazabilidad e impacto.
5. Una baseline aprobada se etiqueta junto con el producto correspondiente.
6. Los IDs y el significado normativo deben coincidir entre idiomas.

## 10. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.1 | 2026-10-16 | Resolución de la matriz y el gate de distribución del MVP |
| 0.1.0 | 2026-10-16 | Roadmap y gobernanza iniciales |
