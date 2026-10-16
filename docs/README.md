# Sonvik — Documentación

**Versión documental:** 0.1.0  
**Producto objetivo:** MVP 0.1.0  
**Idioma canónico:** Español; traducción inglesa sincronizada  
**Última sincronización:** 2026-10-16

Este directorio contiene la documentación viva y canónica de Sonvik. El brief original ([ES](es/project-brief.md) · [EN](en/project-brief.md)) conserva la exploración inicial; estos documentos convierten esa visión en alcance, requisitos y decisiones trazables.

## Mapa documental

| ID | Español | English | Propósito |
| --- | --- | --- | --- |
| BRIEF | [Brief del proyecto](es/project-brief.md) | [Project brief](en/project-brief.md) | Contexto y propuestas iniciales no normativas |
| DOC-01 | [Visión y alcance](es/01-vision-y-alcance.md) | [Vision and scope](en/01-vision-and-scope.md) | Problema, propuesta de valor y límites |
| DOC-02 | [Especificación de requisitos](es/02-especificacion-de-requisitos.md) | [Software requirements](en/02-software-requirements-specification.md) | Comportamiento verificable del MVP |
| DOC-03 | [Casos de uso](es/03-casos-de-uso.md) | [Use cases](en/03-use-cases.md) | Interacciones y flujos alternativos |
| DOC-04 | [Arquitectura de software](es/04-arquitectura-de-software.md) | [Software architecture](en/04-software-architecture.md) | Límites, flujo, despliegue y seguridad |
| DOC-05 | [Diccionario de datos](es/05-diccionario-de-datos.md) | [Data dictionary](en/05-data-dictionary.md) | Modelo canónico mínimo y reglas de validación |
| DOC-06 | [Roadmap y gobernanza](es/06-roadmap-y-gobernanza.md) | [Roadmap and governance](en/06-roadmap-and-governance.md) | Entrega, calidad, riesgos y cambios |
| ADR-001 | [Arquitectura y alcance del MVP](adr/ADR-001-mvp-architecture.md) | [MVP architecture and scope](adr/ADR-001-mvp-architecture-en.md) | Primera decisión arquitectónica |

## Estados

`Borrador` → `En revisión` → `Aprobado` → `Reemplazado` o `Retirado`.

## Reglas

1. Español es la fuente canónica; la versión inglesa se sincroniza en el mismo cambio.
2. Los IDs no se reutilizan; un elemento eliminado cambia de estado.
3. Un cambio de comportamiento actualiza requisitos, casos de uso y criterios afectados.
4. Las decisiones arquitectónicas significativas se registran en `docs/adr/`.
5. Markdown es la fuente de verdad; otros formatos son artefactos generados.
