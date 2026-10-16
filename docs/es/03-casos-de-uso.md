# DOC-03 — Casos de uso

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.0 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Actores

- **Usuario:** crea archivos y ejecuta la CLI.
- **CI:** invoca la CLI y consume exit codes.
- **API objetivo:** recibe requests y devuelve responses; no es confiable.
- **Sistema operativo:** proporciona archivos, variables de entorno, red y reloj.

## 2. Catálogo

| ID | Caso | Requisitos |
| --- | --- | --- |
| CU-001 | Ejecutar una request | RF-001–004, RF-020–023, RF-030–032 |
| CU-002 | Ejecutar con environment | RF-010–014 |
| CU-003 | Verificar una API en CI | RF-021–023, RNF-001–002 |
| CU-004 | Diagnosticar configuración inválida | RF-004, RF-013, RF-023 |
| CU-005 | Usar un secreto sin persistirlo | RF-040–042, RNF-005 |

## 3. CU-001 — Ejecutar una request

**Precondición:** existe un archivo `version: 1` válido.  
**Disparador:** `sonvik run request.yaml`.

1. Sonvik carga y valida el documento.
2. Resuelve variables disponibles.
3. Construye la request y aplica timeout.
4. Envía una sola vez.
5. Compara el status.
6. Muestra resumen y termina con `0` o `1`.

**Alternativas:** archivo o método inválido → no envía y retorna `2`; timeout, DNS, TLS o conexión fallida → retorna `2`; status inesperado → retorna `1`.

## 4. CU-002 — Ejecutar con environment

1. El usuario pasa `--env environment.yaml` y, opcionalmente, `--var clave=valor`.
2. Sonvik valida ambos documentos/argumentos.
3. Combina fuentes con precedencia definida.
4. Sustituye cada placeholder o falla antes del envío.
5. Continúa CU-001.

**Alternativas:** clave duplicada en distintas fuentes → gana la de mayor precedencia; variable ausente → error `2` sin tráfico.

## 5. CU-003 — Verificar en CI

1. El pipeline obtiene el repositorio e inyecta secretos mediante su entorno.
2. Ejecuta el mismo comando usado localmente.
3. Usa el exit code para aprobar o fallar el job.

**Postcondición:** Sonvik no crea cuenta, estado remoto ni artefactos implícitos.

## 6. CU-004 — Diagnosticar configuración inválida

1. Sonvik señala archivo, campo y causa cuando estén disponibles.
2. No incluye valores resueltos sensibles.
3. No envía tráfico parcial.

## 7. CU-005 — Usar un secreto sin persistirlo

1. El usuario o CI define una variable de entorno.
2. El archivo referencia su nombre mediante placeholder.
3. Sonvik usa el valor solo durante la ejecución.
4. El output normal informa resultado sin headers ni bodies.

## 8. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Casos de uso del primer vertical |
