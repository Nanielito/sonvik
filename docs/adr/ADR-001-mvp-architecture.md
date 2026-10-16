# ADR-001: Arquitectura y alcance del MVP

- Estado: Aceptado
- Fecha: 2026-10-16

## Contexto

Sonvik busca ejecutar y verificar requests HTTP definidas en archivos locales, tanto por personas como por CI. A futuro podría sumar una aplicación desktop y una interfaz MCP, pero hoy no existe un segundo consumidor del motor ni evidencia que justifique diseñar esas interfaces.

El riesgo principal no es que el MVP tenga pocas capas, sino fijar demasiado pronto un modelo de dominio, un formato o una estructura de repositorio que luego haya que mantener. El primer incremento debe validar una sola promesa: una request versionable produce el mismo resultado localmente y en CI.

## Decisión

### Alcance funcional

El MVP tendrá un único camino vertical:

```text
archivo de request + environment opcional
  -> parseo y validación
  -> resolución de variables
  -> ejecución HTTP
  -> comparación exacta del status esperado
  -> salida humana + exit code
```

Se soportarán:

- métodos HTTP habituales;
- URL, headers y body JSON;
- environments como mapas planos de strings;
- placeholders `{{nombre}}` sin lenguaje de expresiones;
- una única assertion: status HTTP exacto;
- timeout finito y configurable;
- salida concisa para humanos y exit codes estables.

La precedencia de variables será `--var` > variables de entorno del proceso > archivo de environment. Una variable ausente o un placeholder mal formado será un error previo al envío.

Los archivos de environment se consideran configuración, no almacenes de secretos. Los secretos se inyectarán por variables de entorno; `--var` se reservará para valores no sensibles porque los argumentos pueden quedar en el historial del shell o ser visibles para otros procesos. Sonvik no imprimirá headers ni bodies por defecto. No se construirá todavía integración con keychains ni un sistema propio de secretos.

### Forma del programa

Se usará Rust, sujeto a una revisión breve tras completar el vertical slice. El repositorio comenzará como **un solo package Cargo** con:

- `src/lib.rs`: carga, validación, resolución, HTTP y evaluación;
- `src/main.rs`: argumentos, presentación del resultado y exit code.

La biblioteca no dependerá de la CLI. Se dividirá en crates únicamente cuando aparezca un segundo consumidor real o tiempos de compilación/ownership lo justifiquen. Los módulos internos pueden seguir las responsabilidades del flujo sin convertir cada concepto en una entidad pública.

No se aplicará una arquitectura hexagonal formal ni se crearán traits para implementaciones únicas. El límite útil es simple: la lógica retorna datos y errores; `main` decide cómo mostrarlos y terminar el proceso.

### Interfaz CLI

Habrá un solo comando de ejecución:

```bash
sonvik run request.yaml [--env environment.yaml] [--var clave=valor]
```

No se distinguirán todavía `send`, `run` y `test`: con una sola request representan el mismo trabajo. Los exit codes iniciales serán:

- `0`: request ejecutada y assertion satisfecha;
- `1`: request ejecutada pero assertion fallida;
- `2`: uso inválido, archivo/configuración inválida o fallo de ejecución.

La salida estructurada (por ejemplo JSON o JUnit) se decidirá cuando exista el primer consumidor concreto; no se anticipará un sistema de reporters.

### Formato de archivos

Se elegirá YAML para el MVP porque los archivos son escritos por personas y los ejemplos actuales ya usan esa forma. Todo documento tendrá `version: 1`, se validarán campos desconocidos y no se admitirán extensiones arbitrarias. Los archivos tendrán un máximo de 1 MiB.

Esquema conceptual mínimo:

```yaml
version: 1
name: Health check
request:
  method: GET
  url: "{{baseUrl}}/health"
  headers:
    Accept: application/json
expect:
  status: 200
```

```yaml
version: 1
variables:
  baseUrl: http://localhost:8080
```

El formato canónico es el modelo validado, no detalles del parser utilizado. Cualquier evolución incompatible requerirá otra versión y una migración explícita; no se promete compatibilidad con bocetos sin `version`.

### Ejecución y pruebas

La ejecución será secuencial y sin retries automáticos. Los retries pueden repetir operaciones no idempotentes y ocultar fallos; se añadirán solo con semántica explícita.

El timeout predeterminado será de 30 segundos. Se seguirán hasta cinco redirects, rechazando cualquier downgrade de HTTPS a HTTP. Los nombres de variable usarán `[A-Za-z_][A-Za-z0-9_]*`; se sustituirán valores de URL, headers y strings dentro de `body.json`, nunca claves.

Una prueba de integración levantará un servidor HTTP local efímero y recorrerá el camino completo. Las pruebas no dependerán de red externa, cuentas ni servicios cloud.

## Decisiones cuestionadas

| Propuesta del brief | Decisión para el MVP | Motivo |
| --- | --- | --- |
| Workspace con `sonvik-core` y `sonvik-cli` | Un package con lib + bin | Mantiene el límite sin coste de workspace ni APIs entre crates. |
| `Project` como raíz | No existe aún | Ejecutar un archivo no requiere manifiesto ni modelo agregado. |
| Requests, collections y flows | Solo una request | Una colección puede ser después una lista de paths; flows requieren semántica propia. |
| Assertions, captures y expresiones | Solo status exacto | JSONPath, tipado y scopes multiplican decisiones antes de validar el runner. |
| Importación cURL/OpenAPI en v0.1 | Diferida | Importar exige primero estabilizar el formato destino. |
| Capa de transporte extensible | HTTP directo | GraphQL usa HTTP; gRPC/WebSocket justificarán una abstracción cuando lleguen. |
| Desktop y MCP condicionan el core | Solo límite lib/bin | Se conserva reutilización sin diseñar APIs para consumidores hipotéticos. |
| Gestión segura propia de secretos | Inyección externa y logs mínimos | Evita almacenar secretos y delega su custodia al entorno existente. |
| Reporters múltiples | Texto y exit codes | Es lo mínimo necesario para uso local y CI. |

## Alternativas consideradas

### Workspace desde el inicio

Hace visible la separación entre core y CLI, pero añade manifiestos, versionado y una frontera pública que todavía no aporta aislamiento real. Se rechaza por ahora.

### TypeScript/Node como implementación única

Reduce la distancia con una futura UI, pero introduce un runtime en la distribución del CLI. Sigue siendo una alternativa válida si el vertical slice demuestra que el coste de desarrollo o el ecosistema HTTP/YAML de Rust frena el proyecto. Rust es una elección reversible, no un principio de producto.

### Archivos `.http`

Son excelentes para requests manuales, pero no tienen una representación estándar de environments y assertions. Se difieren como formato importable, no canónico.

### JSON o TOML

JSON es inequívoco pero menos cómodo para edición manual; TOML funciona bien para configuración, pero representa con menos naturalidad bodies y estructuras HTTP anidadas. Ambos pueden reconsiderarse antes de declarar estable `version: 1`; no se soportarán varios formatos en el MVP.

## Consecuencias

Positivas:

- el primer resultado útil requiere pocas piezas y una sola frontera arquitectónica;
- core y CLI permanecen desacoplados sin estructura especulativa;
- el comportamiento puede verificarse completamente offline;
- versionar desde el primer archivo evita compatibilidad accidental.

Costes y límites aceptados:

- todavía no hay flows, captures, assertions de body, importadores ni GUI;
- la CLI no ejecuta colecciones;
- Rust puede reevaluarse después del vertical slice;
- YAML pasa a ser un compromiso solo cuando se implemente y documente el esquema `version: 1`.

## Criterios para revisar esta decisión

Revisar este ADR cuando ocurra alguno de estos hechos:

- aparece un segundo consumidor del motor: extraer `sonvik-core` puede reducir acoplamiento;
- se necesitan varias requests encadenadas: definir primero semántica de fallo, scope y captures;
- un consumidor de CI exige salida estructurada: añadir el formato concreto que necesite;
- se implementa otro transporte: extraer una abstracción desde dos implementaciones reales;
- el vertical slice en Rust revela fricción de distribución o mantenimiento superior a su beneficio.
