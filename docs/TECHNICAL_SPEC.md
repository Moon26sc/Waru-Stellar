# Technical specification

## 1. Objetivo

Convertir una observación agrícola elegible en un pago limitado y no duplicable. La cadena aplica reglas financieras y de autorización; la medición física y la interpretación agronómica permanecen fuera de cadena.

## 2. Unidades y definición de ahorro

El MVP debe escoger **una unidad canónica** y usarla en todo el pipeline. Propuesta: litros por periodo de medición (`L`). Para un periodo:

- `baseline_liters`: consumo de referencia ajustado por cultivo, superficie, duración y condiciones acordadas.
- `observed_liters`: consumo observado durante el periodo.
- `saved_liters = max(baseline_liters - observed_liters, 0)`.

La comparación solo es válida si baseline y observación comparten periodo, superficie, cultivo, método y condiciones de normalización. No comparar directamente humedad del suelo con volumen de agua ni inferir ahorro únicamente desde una lectura puntual de humedad.

## 3. Componentes

### Edge / simulator
- Emite lecturas con `device_id`, `farm_ref` seudónimo, timestamp, unidad, valor, firmware/source y secuencia.
- Simulator debe marcar cada payload como `source=simulator`; jamás presentarlo como telemetría física.
- Los datos crudos no se escriben en el contrato.

### Ingestion & validation service
- Valida esquema, rango, timestamp, secuencia, calibración vigente y ventana de medición.
- Calcula baseline y ahorro con una versión identificada del método.
- Rechaza gaps, duplicados, lecturas fuera de rango y dispositivos revocados.
- Produce un manifiesto canónico y su SHA-256.

### Oracle signer
- Firma una atestación estructurada con una clave dedicada.
- La clave no debe vivir en frontend ni en variables expuestas al navegador.
- La rotación/revocación debe estar documentada.
- Un único oráculo es aceptable para demo, pero es un punto de confianza explícito; no venderlo como descentralización de datos.

### Soroban
- Guarda administrador, oráculo, asset, tasa de recompensa y límite por liquidación.
- Recibe fondos del patrocinador.
- Acepta una atestación autenticada, comprueba hash/nonce, ahorro positivo y límites.
- Transfiere el activo desde el balance del contrato a la wallet del agricultor.
- Registra un evento con referencia, hash, unidad, ahorro y monto.

### Web / wallet
- Conecta wallet Stellar compatible.
- Muestra mediciones, estado de validación, cálculo, activo, monto y recibo.
- El usuario firma las transacciones que requieren su autorización.
- Consulta estado de red y contrato; no sustituye respuestas fallidas por datos mock.

## 4. State machine

`INGESTED -> VALIDATED -> ATTESTED -> SETTLED`

Estados terminales adicionales: `REJECTED`, `EXPIRED`.

- `INGESTED`: payload recibido, aún no elegible.
- `VALIDATED`: reglas de calidad y baseline aprobadas.
- `ATTESTED`: oráculo firmó un manifiesto con ID único.
- `SETTLED`: contrato aceptó el ID una sola vez y ejecutó el pago.
- `REJECTED`: fallo de regla, fuente revocada o ahorro no positivo.
- `EXPIRED`: ventana superada antes de liquidar.

No permitir transición de `REJECTED` o `EXPIRED` a `SETTLED` con el mismo ID. Una corrección genera un nuevo manifiesto/ID y mantiene vínculo de auditoría.

## 5. Fórmula de recompensa

Para el prototipo:

`reward_units = min(saved_liters * reward_per_liter, max_reward_per_claim)`

Requisitos:
- Aritmética entera en unidades mínimas del token; nada de `float`.
- La tasa y el cap los define el patrocinador y se fijan antes del periodo.
- El contrato valida overflow, ahorro positivo, monto > 0 y fondos suficientes.
- El contrato no decide si el baseline agronómico es científicamente válido; eso pertenece a la política del programa y al oráculo.

La conversión de litros a recompensa es una política de incentivo, no una medición de valor monetario del agua.

## 6. Evidence manifest

Campos recomendados:
- `schema_version`
- `claim_id` (nonce/ID único)
- `farmer_wallet`
- `farm_ref` seudónimo
- `period_start`, `period_end`
- `crop_code`, `area_m2`
- `baseline_liters`, `observed_liters`, `saved_liters`
- `unit`
- `method_id`, `method_version`
- `device_or_simulator_ref`
- `calibration_ref`
- `source_kind` (`sensor`, `gateway`, `simulator`)
- `issued_at`, `expires_at`
- `payload_hash`
- `oracle_key_id`

Canonicalizar JSON (orden estable, UTF-8, sin campos ambiguos) antes de SHA-256. No incluir nombres, teléfonos, coordenadas precisas ni documentos personales en cadena.

## 7. Datos on-chain / off-chain

On-chain: parámetros de programa, dirección de asset, direcciones de roles, nonce/claim ID, hash de evidencia, ahorro resumido, importe, destinatario y eventos de liquidación.

Off-chain: telemetría cruda, ubicación precisa, identidad civil, detalles de parcela, calibración completa y documentación agronómica. Proteger con controles de acceso, retención y consentimiento.

## 8. Riesgos de medición

- Sensores mal calibrados o físicamente desplazados.
- Manipulación de gateway o replay de lecturas.
- Baseline injusto por clima, cultivo, suelo, superficie o calendario.
- Ahorro aparente por menor producción o estrés del cultivo.
- Doble conteo de periodos o parcelas.
- Colusión productor/oráculo.
- Pérdida de conectividad y lecturas faltantes.

Mitigaciones: baseline versionado, reglas por cultivo, ventanas completas, calibración trazable, límites, auditoría por muestreo, anomalías, y un mecanismo de disputa fuera de cadena. El MVP debe describir el alcance real de la validación.

## 9. Entornos

- Local: tests de contrato y simulador claramente etiquetado.
- Testnet: asset de prueba, contrato y wallets de prueba; evidencia pública.
- Mainnet: fuera del MVP; requiere revisión legal, operacional, agronómica, custodia y seguridad.
