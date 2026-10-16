# DOC-05 — Diccionario de datos

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.0 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Convenciones

- Los campos desconocidos se rechazan.
- Los documentos declaran `version: 1`.
- Los mapas de variables contienen strings.
- Un campo omitido usa un default documentado; `null` no equivale automáticamente a omitido.
- El modelo interno puede cambiar sin migración mientras preserve el formato y comportamiento públicos.

## 2. DAT-001 — RequestDocument

| Campo | Tipo | Obligatorio | Regla |
| --- | --- | ---: | --- |
| `version` | integer | Sí | Exactamente `1` |
| `name` | string | No | No vacío si existe |
| `request` | Request | Sí | Request válida |
| `expect` | Expectation | Sí | MVP exige status |

## 3. DAT-002 — Request

| Campo | Tipo | Obligatorio | Regla |
| --- | --- | ---: | --- |
| `method` | enum/string | Sí | GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS |
| `url` | string | Sí | URL HTTP(S) luego de resolver |
| `headers` | map<string,string> | No | Sin nombres inválidos |
| `body.json` | YAML/JSON value | No | Serializado como JSON |
| `timeout_ms` | positive integer | No | Default 30000; rango 1–300000 |

`body.json` admite únicamente valores compatibles con JSON. Sonvik aplica `Content-Type: application/json` si el usuario no lo definió. No se infiere autenticación.

## 4. DAT-003 — Expectation

| Campo | Tipo | Obligatorio | Regla |
| --- | --- | ---: | --- |
| `status` | integer | Sí | 100–599 |

## 5. DAT-004 — EnvironmentDocument

| Campo | Tipo | Obligatorio | Regla |
| --- | --- | ---: | --- |
| `version` | integer | Sí | Exactamente `1` |
| `variables` | map<string,string> | Sí | Nombres válidos y únicos |

Un environment no es un secret store. El repositorio DEBERÍA ignorar archivos locales que el usuario decida crear, pero el formato no marcará valores como secretos en el MVP.

## 6. DAT-005 — RunResult

Dato de runtime, no persistido automáticamente:

| Campo | Tipo | Descripción |
| --- | --- | --- |
| `outcome` | enum | passed, assertion_failed, error |
| `status` | integer? | Status si hubo response |
| `duration` | duration? | Tiempo de transporte medido |
| `error_kind` | enum? | Config, variables, transport o timeout |
| `message` | string | Resumen redactado |

## 7. Ejemplo válido

```yaml
version: 1
name: Health check
request:
  method: GET
  url: "{{baseUrl}}/health"
  headers:
    Accept: application/json
  timeout_ms: 5000
expect:
  status: 200
```

## 8. Ejemplo inválido

```yaml
# Sin versión
request: { method: GET, url: https://example.test }
expect: { status: 200 }
```

## 9. Decisiones pendientes antes de congelar v1

- Tratamiento de múltiples valores de un mismo header.

## 10. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Modelo mínimo inicial |
