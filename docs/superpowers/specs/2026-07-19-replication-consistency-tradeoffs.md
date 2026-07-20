# Especificación: tradeoffs de consistencia en Replicación

> **Issue:** #48
> **Milestone:** 09 Replicación
> **Estado:** tested.

## Propósito

Este paso cierra el capítulo de Replicación documentando los tradeoffs de
consistencia que aparecen al combinar primary/replica, lag y confirmación
síncrona o asíncrona.

## Alcance actual

- `docs/09-replicacion.md` explica confirmación asíncrona y síncrona desde sus
  costos.
- El capítulo compara lecturas desde primary y lecturas desde réplicas.
- El texto nombra explícitamente el riesgo de lecturas stale cuando hay lag.
- README y ROADMAP elevan Replicación a `tested`.
- El checklist del plan marca completo el capítulo 09.

## Decisión de diseño

No se agrega código en este issue porque el alcance es documental. Quorum,
consenso, failover automático, timeouts, redes parciales y elección de leader
pertenecen a cursos o pasos posteriores. Aquí basta con dejar clara la tensión
fundamental: latencia, disponibilidad y frescura de lectura no se maximizan al
mismo tiempo.
