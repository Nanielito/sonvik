# Sonvik — Brief del proyecto / handoff para Codex

> Estado: concepto y arquitectura inicial. Este documento resume las decisiones y propuestas discutidas para comenzar la implementación; no implica que ya exista código.

## 1. Visión

**Sonvik** es una herramienta open source para explorar, ejecutar y probar APIs, inspirada en clientes como Postman y Bruno, pero diseñada desde el principio como un **motor de pruebas independiente de la interfaz**.

**Tagline provisional:** *Explore. Execute. Verify.*

**Objetivo:** una alternativa local-first, Git-native y orientada a automatización, con CLI como interfaz principal, una futura aplicación desktop y posible integración MCP para agentes.

**Motivación:** evitar dependencia de cuentas, servicios cloud, límites comerciales y formatos propietarios. El valor diferencial no es solo ejecutar requests, sino permitir **pruebas reproducibles y flujos declarativos** utilizables tanto por personas como por CI/CD y agentes.

## 2. Principios de producto

1. **Your APIs, your files:** proyectos y colecciones almacenados localmente como archivos de texto legibles y versionables con Git; sin login obligatorio.
2. **One engine, multiple interfaces:** CLI, desktop y MCP deben reutilizar exactamente el mismo core.
3. **Testing as code:** requests, assertions, variables, capturas y flows reproducibles en automatización.
4. **Formato abierto y portable:** evitar lock-in; favorecer YAML/HTTP u otro formato legible, con importadores de formatos existentes.
5. **MVP enfocado:** no comenzar con colaboración cloud, sincronización, marketplace, dashboards ni múltiples protocolos.
6. **Seguridad local:** los secretos no deben versionarse ni imprimirse accidentalmente; considerar variables de entorno y almacenes seguros del sistema.

## 3. Usuarios y casos de uso

- Desarrolladores que inspeccionan endpoints manualmente.
- Equipos que versionan pruebas junto al código de una API.
- Pipelines CI/CD que ejecutan colecciones o flujos sin GUI.
- A futuro, agentes de IA que ejecutan pruebas mediante un servidor MCP.

## 4. Arquitectura propuesta

```text
                    +----------------+
                    |  Desktop UI    |  (futuro)
                    +--------+-------+
                             |
+-------------+              |
| Sonvik CLI  +--------------+
+-------------+              |
                             v
                    +----------------+
                    |  Sonvik Core   |
                    |----------------|
                    | requests       |
                    | environments   |
                    | variables      |
                    | assertions     |
                    | captures       |
                    | flows          |
                    | runner         |
                    | secrets        |
                    +--------+-------+
                             |
                    +--------+---------+
                    |                  |
                    v                  v
                   HTTP          otros protocolos
                                (GraphQL/gRPC/etc.)

+----------------+       +----------------+
| MCP interface  +------>|  Sonvik Core   |  (futuro)
+----------------+       +----------------+
```

**Tecnología preferida para explorar:** Rust para core y CLI; Tauri + TypeScript para desktop. Rust se propuso también como oportunidad de aprendizaje. **No es una decisión técnica irrevocable**: validar ergonomía, distribución y coste de mantenimiento.

**Separación de responsabilidades:** el core no debe depender de la UI; las interfaces deben ser adaptadores del motor. Considerar arquitectura modular sin sobrediseñar el MVP.

### Organización inicial sugerida

```text
sonvik/
├── Cargo.toml                 # workspace Rust (propuesta)
├── crates/
│   ├── sonvik-core/
│   └── sonvik-cli/
├── apps/
│   └── desktop/              # futuro
├── examples/
└── docs/
```

Los nombres `sonvik-core`, `sonvik-cli`, `sonvik-desktop` y `sonvik-mcp` son **nombres conceptuales**; la estructura definitiva del repositorio queda abierta.

## 5. Modelo de dominio por definir

Entidades principales propuestas:

| Entidad | Responsabilidad |
| --- | --- |
| `Project` | Raíz, configuración, referencias a requests y environments |
| `Request` | Método, URL, headers, query, body, autenticación |
| `Environment` | Valores específicos de local/staging/production |
| `Variable` | Resolución de placeholders y alcance |
| `Response` | Status, headers, body, duración y metadatos |
| `Assertion` | Validación declarativa de respuestas |
| `Capture` | Extracción de valores para pasos posteriores |
| `Flow` | Secuencia declarativa de requests y dependencias |
| `Runner` | Orquestación, ejecución y reporte |

Cuestiones abiertas: precedencia y scope de variables; tipado de expresiones; gestión de errores; timeouts; retries; autenticación; esquema y versionado de archivos; seguridad de secretos; flujo de datos entre pasos; concurrencia; output y exit codes.

## 6. Formato de proyecto: ejemplos exploratorios

**Estos YAML son bocetos, NO una especificación aprobada.** Hay que diseñar un esquema consistente y versionado antes de implementarlos.

```text
api-tests/
├── project.yaml
├── environments/
│   ├── local.yaml
│   ├── staging.yaml
│   └── production.yaml
├── auth/
│   └── login.yaml
├── users/
│   ├── create-user.yaml
│   ├── get-user.yaml
│   └── delete-user.yaml
└── flows/
    └── user-lifecycle.yaml
```

Ejemplo de request:

```yaml
name: Create user
request:
  method: POST
  url: "{{baseUrl}}/users"
  headers:
    Content-Type: application/json
  body:
    name: Daniel
    email: dan@example.com
expect:
  status: 201
  body:
    id: exists
    name: Daniel
capture:
  userId: $.id
```

Ejemplo de flow:

```yaml
name: user-lifecycle
steps:
  - run: auth/login
    capture:
      token: $.access_token
  - run: users/create
    capture:
      userId: $.id
  - run: users/get
    with:
      id: "{{userId}}"
  - run: users/delete
    with:
      id: "{{userId}}"
```

**Nota de diseño:** definir explícitamente cómo funcionan las capturas, si son propias de cada request o del flow, y qué ocurre si falla un paso. La sintaxis `exists` y `$.id` también necesita especificación.

## 7. Experiencia CLI deseada

Comandos ilustrativos (nombres y argumentos sujetos a diseño):

```bash
sonvik init
sonvik send users/create
sonvik run user-lifecycle
sonvik test --env staging
sonvik import openapi.yaml
sonvik run --env staging --report junit
```

Propiedades esperadas:

- Mismo comportamiento en local y CI.
- Salida legible para humanos y formatos de reporte procesables.
- Códigos de salida adecuados para automatización.
- Capacidad de ejecutar una request, una colección o un flow.
- Nunca exponer tokens/secrets por defecto en logs o reportes.

## 8. Roadmap tentativo

| Fase | Alcance |
| --- | --- |
| **v0.1** | HTTP (GET/POST/PUT/PATCH/DELETE), headers, query, JSON/text body, environments, variables, assertions de status/JSON, captures, CLI; importación cURL/OpenAPI como objetivo inicial a validar |
| **v0.2** | Flows declarativos |
| **v0.3** | Importación de colecciones Postman |
| **v0.4** | Desktop UI |
| **v0.5** | GraphQL |
| **v0.6** | gRPC y WebSocket |
| **v0.7** | MCP server para agentes |

Las versiones son **orientativas**, no compromisos. Se recomienda reducir aún más el primer milestone si resulta necesario: por ejemplo, empezar por HTTP + CLI + status assertions antes de importar OpenAPI.

## 9. Interoperabilidad y formatos

Fuentes de importación propuestas:

- cURL
- OpenAPI
- Postman Collections
- Bruno Collections

El formato canónico de Sonvik debe poder guardarse en Git, ser legible y mantenerse independiente de servicios propietarios. Evaluar YAML frente a archivos `.http` u otros formatos antes de cerrar la decisión.

## 10. MCP y agentes (visión futura)

Sonvik no solo podría **consumir** servicios MCP: también podría **exponer su propio motor** como herramientas para agentes.

Ejemplos conceptuales:

```text
run_flow("user-lifecycle", env="staging")
run_request("users/create", variables={...})
```

Los permisos, secretos, protección contra solicitudes destructivas y aislamiento deben diseñarse antes de habilitar ejecución por agentes.

## 11. Nombre e identidad

**Nombre elegido:** Sonvik.

**Origen conceptual:** la metáfora de una sonda/sonar: enviar una señal, observar la respuesta y comprender el comportamiento de un sistema. El nombre evolucionó tras investigar y descartar Sonda, Kova, Runa, Vexa, Fluxa, Sondr, Pondr, Ekon y otros por conflictos o proximidad con productos existentes.

**Tagline provisional:** *Explore. Execute. Verify.*

**Importante:** Sonvik fue seleccionado como nombre de trabajo, pero **NO se ha certificado disponibilidad** de dominio, organización GitHub, crates.io, npm, PyPI ni registro de marca. Comprobar antes de publicación o branding definitivo, incluyendo búsquedas de marcas pertinentes (por ejemplo, INPI Argentina y USPTO, clases 9 y 42).

## 12. Primeras tareas sugeridas para Codex

1. Proponer un **ADR-001** que defina alcance del MVP, decisiones y alternativas.
2. Diseñar el **modelo de dominio** y los límites entre core, CLI, transporte HTTP y persistencia.
3. Proponer el **formato mínimo versionado** para requests, environments y assertions; añadir ejemplos válidos/inválidos.
4. Crear workspace Rust con `sonvik-core` y `sonvik-cli`, pruebas y CI básico.
5. Implementar el primer camino vertical: cargar una request desde archivo → resolver variables → enviar HTTP → evaluar status → emitir resultado y exit code.
6. Añadir tests unitarios y de integración con servidor HTTP local/mock, sin depender de servicios externos.
7. Documentar decisiones pendientes antes de implementar flows, importadores o UI.

### Criterios de aceptación del primer vertical slice

- Un usuario puede definir y ejecutar una request HTTP desde un archivo versionable.
- Puede seleccionar un environment y resolver `{{baseUrl}}`.
- Puede validar un código HTTP esperado.
- La CLI indica éxito o fallo con exit code coherente.
- Los tests funcionan offline usando un servidor de prueba local.
- No se requiere cuenta ni servicio cloud de Sonvik.

## 13. Prompt sugerido para iniciar con Codex

> Estamos construyendo **Sonvik**, una herramienta open source de API testing local-first y Git-native, con core/CLI en Rust y futura UI Tauri y servidor MCP. Lee `docs/es/project-brief.md` completo. Primero analiza las decisiones abiertas y propón un ADR-001, una arquitectura modular pragmática y un esquema mínimo de request/environment/assertion. No implementes todavía flows, GUI ni MCP. Después de acordar el diseño, crea un workspace Rust con `sonvik-core` y `sonvik-cli` y el primer vertical slice: ejecutar una request HTTP declarativa, resolver variables, verificar status y devolver exit codes correctos. Incluye pruebas automatizadas locales, documentación y CI básico. Prioriza sencillez, portabilidad y cero dependencia cloud.

---

*Documento de transferencia preparado a partir de la conversación de diseño de Sonvik. Los ejemplos y el roadmap son propuestas iniciales, no una API pública establecida.*
