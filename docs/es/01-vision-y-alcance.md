# DOC-01 — Visión y alcance

| Campo | Valor |
| --- | --- |
| Estado | Aprobado |
| Versión | 0.1.0 |
| Producto objetivo | MVP 0.1.0 |
| Última actualización | 2026-10-16 |

## 1. Propósito

Definir qué problema resuelve Sonvik, para quién y qué queda fuera del primer producto verificable. Los compromisos comprobables están en [DOC-02](02-especificacion-de-requisitos.md).

## 2. Problema

Probar una API suele depender de una GUI, una cuenta cloud o un formato difícil de revisar. Esto reduce la reproducibilidad entre desarrollo y CI y puede encerrar las pruebas en un proveedor.

## 3. Visión

Sonvik es un ejecutor open source, local-first y Git-native para explorar y verificar APIs desde archivos legibles. El mismo motor podrá alimentar otras interfaces cuando aparezcan necesidades reales.

**Promesa del MVP:** una request versionada produce un resultado determinista y automatizable tanto localmente como en CI.

## 4. Usuarios

| Actor | Necesidad |
| --- | --- |
| Desarrollador | Ejecutar y depurar una request sin cuenta ni GUI |
| Equipo de API | Revisar pruebas junto al código |
| Pipeline CI | Obtener resultado y exit code reproducibles |
| Mantenedor | Evolucionar formato y motor sin complejidad accidental |

## 5. Alcance del MVP 0.1.0

- CLI multiplataforma.
- Requests HTTP desde YAML: método, URL, headers y body JSON.
- Environment opcional y overrides no sensibles.
- Sustitución de placeholders simples.
- Assertion de status exacto.
- Timeout finito.
- Salida humana y exit codes estables.
- Pruebas offline contra un servidor local.

## 6. Fuera del alcance

- GUI, MCP, colaboración y sincronización cloud.
- Projects, colecciones, flows, captures y scripting.
- Assertions de body, JSONPath y lenguajes de expresiones.
- Importadores cURL, OpenAPI, Postman o Bruno.
- Reportes JSON/JUnit.
- GraphQL específico, gRPC y WebSocket.
- Retries automáticos y concurrencia.
- Almacén propio de secretos.

## 7. Principios

1. Archivos del usuario antes que cuentas del proveedor.
2. Un vertical completo antes que muchas funciones parciales.
3. Biblioteca independiente de presentación, sin capas especulativas.
4. Comportamiento explícito antes que magia o defaults ambiguos.
5. Los secretos no viven en archivos versionables ni aparecen en output normal.
6. Todo formato persistente declara versión.
7. Una abstracción nace de dos implementaciones reales, no de una posibilidad.

## 8. Indicadores de éxito

- Un usuario nuevo ejecuta el ejemplo local siguiendo el README.
- El mismo archivo pasa o falla igual localmente y en CI.
- Configuración inválida falla antes de enviar tráfico.
- Ningún test necesita Internet.
- La salida por defecto no revela headers ni bodies.

## 9. Futuro no comprometido

Desktop, MCP, flows, importadores y protocolos adicionales siguen en la visión. Cada uno requiere evidencia de uso y un ADR o cambio de alcance antes de incorporarse.

## 10. Historial

| Versión | Fecha | Cambio |
| --- | --- | --- |
| 0.1.0 | 2026-10-16 | Baseline inicial del MVP |
