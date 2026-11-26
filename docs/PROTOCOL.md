# Waru protocol & evidence semantics

## Principio

**La cadena prueba ejecución y estado; no prueba por sí sola la verdad del mundo físico.**

Un recibo on-chain puede demostrar que una clave autorizada presentó una atestación, que el contrato aceptó sus parámetros y que se ejecutó una transferencia. No demuestra que el volumen de agua fue medido correctamente ni que el agricultor logró un ahorro causal.

## Evidence levels

| Nivel | Evidencia | Lenguaje permitido |
|---|---|---|
| E0 | Datos de simulador | “Escenario simulado” |
| E1 | Lecturas de dispositivo/gateway | “Lecturas recibidas” |
| E2 | Lecturas con controles de calidad y calibración documentada | “Lecturas validadas según método X” |
| E3 | Atestación del oráculo aceptada por Soroban | “Ahorro reportado y atestado; pago ejecutado” |
| E4 | Auditoría independiente del método y muestreo físico | “Resultado auditado bajo alcance y periodo especificados” |

No llamar E3 “ahorro físicamente probado” si no existe una auditoría física.

## Canonical claim

`claim_id` debe ser único por agricultor, parcela seudónima y periodo. Debe derivarse de una fuente de entropía adecuada o asignarse por un servicio con unicidad garantizada. Nunca derivarlo solo del timestamp.

El manifiesto se serializa canónicamente y se hashea. El contrato almacena el hash y los campos mínimos necesarios para liquidación. Publicar un hash no hace que un documento privado sea recuperable ni demuestra su veracidad.

## Oracle attestation

La atestación debe vincular como mínimo:
- `claim_id`
- wallet destinataria
- hash del manifiesto
- ahorro en litros
- monto solicitado o parámetros de cálculo
- versión del método
- fecha de emisión y expiración
- `key_id`

El contrato exige autenticación de la dirección de oráculo configurada. La clave privada se mantiene fuera del navegador y se rota mediante un proceso administrativo. Un oráculo único representa confianza centralizada.

## Settlement receipt

Por cada pago conservar:
- red y passphrase/identificador de red
- Contract ID
- asset contract/address
- `claim_id`
- hash del manifiesto
- dirección del agricultor
- cantidad y unidad del token
- hash de transacción
- ledger/fecha observada
- evento del contrato y resultado de consulta posterior

No fabricar hashes, ledger numbers, Contract IDs ni capturas. Para cada afirmación pública enlazar la transacción real y el código desplegado.

## Replay & idempotency

El contrato registra cada `claim_id` como procesado antes de completar el flujo atómico. Si la transferencia falla, toda la invocación debe revertirse. Una repetición exitosa del mismo ID debe fallar sin pagar de nuevo.

## Privacy

La cadena es pública. Usar identificadores seudónimos y hashes con suficiente entropía. No almacenar datos personales, geolocalización precisa ni telemetría detallada en eventos o storage público. Un hash de datos de baja entropía puede permitir ataques de diccionario; usar manifiestos con nonce y contenido no sensible.
