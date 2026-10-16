# DOC-02 — Especificación de requisitos

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.0 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Convenciones

**DEBE**, **NO DEBE**, **DEBERÍA** y **PUEDE** expresan obligatoriedad. Los IDs son permanentes.

## 2. Requisitos funcionales

| ID | Requisito | Verificación |
| --- | --- | --- |
| RF-001 | La CLI DEBE ejecutar `sonvik run <request>` con environment opcional. | E2E |
| RF-002 | DEBE aceptar GET, POST, PUT, PATCH, DELETE, HEAD y OPTIONS. | Integración |
| RF-003 | DEBE admitir URL, headers y `body.json`. | Integración |
| RF-004 | DEBE rechazar documentos inválidos y campos desconocidos antes de enviar. | Unit/E2E |
| RF-010 | DEBE cargar variables desde un environment YAML opcional. | Unit |
| RF-011 | DEBE resolver placeholders `{{nombre}}` en valores textuales soportados. | Unit |
| RF-012 | La precedencia DEBE ser `--var` > entorno del proceso > archivo. | Unit |
| RF-013 | Una variable ausente o placeholder inválido DEBE impedir el envío. | Unit/E2E |
| RF-014 | `--var` DEBE advertirse/documentarse como no apto para secretos. | Revisión |
| RF-020 | DEBE comparar el status recibido con `expect.status`. | Integración |
| RF-021 | DEBE retornar `0` si la assertion pasa y `1` si falla. | E2E |
| RF-022 | DEBE retornar `2` ante uso, configuración o ejecución inválidos. | E2E |
| RF-023 | DEBE distinguir visualmente éxito, assertion fallida y error de ejecución. | E2E |
| RF-030 | DEBE aplicar un timeout finito con override explícito. | Integración |
| RF-031 | NO DEBE reintentar automáticamente. | Integración |
| RF-032 | DEBE seguir hasta 5 redirects y rechazar downgrade HTTPS→HTTP. | Integración |
| RF-040 | NO DEBE imprimir headers ni bodies por defecto. | Seguridad |
| RF-041 | DEBE aceptar secretos mediante variables de entorno sin persistirlos. | Seguridad |
| RF-042 | Los errores DEBEN evitar repetir valores resueltos sensibles. | Seguridad |

## 3. Requisitos no funcionales

| ID | Categoría | Requisito | Verificación |
| --- | --- | --- | --- |
| RNF-001 | Portabilidad | Core y CLI DEBEN compilar en macOS, Linux y Windows. | CI |
| RNF-002 | Reproducibilidad | Tests DEBEN funcionar sin red externa. | CI |
| RNF-003 | Formato | Todo documento persistente DEBE declarar `version: 1`. | Unit |
| RNF-004 | Mantenibilidad | La lógica DEBE ser utilizable sin depender de la CLI. | Arquitectura/test |
| RNF-005 | Seguridad | Secretos NO DEBEN aparecer en logs, errores o fixtures. | Auditoría/test |
| RNF-006 | Rendimiento | Una ejecución sin contar latencia remota DEBERÍA añadir menos de 100 ms. | Benchmark posterior |
| RNF-007 | Compatibilidad | Cambios incompatibles de schema DEBEN cambiar versión y documentar migración. | Release gate |
| RNF-008 | Distribución | El binario publicado NO DEBE requerir runtime externo. | Smoke test |

## 4. Reglas de trazabilidad

- Cada caso de uso referencia sus `RF-*`.
- Cada componente de DOC-04 indica responsabilidades, no duplica requisitos.
- Cada dato persistente aparece en DOC-05.
- Cada milestone de DOC-06 tiene un gate observable.

## 5. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Primera especificación verificable |
