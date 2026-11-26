# Security model

## Assets
- Saldo del patrocinador en el contrato.
- Clave administrativa.
- Clave del oráculo.
- Integridad de claim IDs y hashes.
- Wallets y privacidad de agricultores.
- Disponibilidad y consistencia de la telemetría.

## Trust boundaries
1. Sensor/device -> gateway.
2. Gateway -> ingestion/validator.
3. Validator -> oracle signer.
4. Oracle signer -> Soroban.
5. Sponsor wallet -> contract vault.
6. Contract -> token contract.
7. Browser -> wallet -> Stellar network.

## Threats and controls

| Amenaza | Control MVP |
|---|---|
| Oráculo comprometido | Clave separada, mínimo privilegio, rotación/revocación, límites por claim y programa |
| Replay / doble pago | `claim_id` único y storage de claims procesados |
| Recompensa inflada | Cap por claim, tasa fijada por admin, validación de overflow y ahorro positivo |
| Fondo insuficiente | Transferencia atómica; fallo revierte el settlement |
| Sponsor no autorizado | `require_auth` sobre depósito y operaciones administrativas |
| Lectura manipulada | Validación de esquema, secuencia, calibración y anomalías; auditoría física fuera de cadena |
| Manipulación de baseline | Método versionado, periodo comparable, política publicada |
| Robo de secret key | Nunca en frontend, `.env` local ignorado, secretos CI gestionados por GitHub |
| Exposición de datos | PII y telemetría granular fuera de cadena; IDs seudónimos |
| Asset equivocado | Configurar explícitamente el asset de Testnet y verificar issuer/contract |
| Front-running / claim substitution | Vincular wallet, hash, claim ID y expiración a la atestación |
| Admin malicioso | límites, eventos, separación de roles y revisión multi-persona antes de mainnet |

## Security invariants
- Un claim se liquida como máximo una vez.
- No se paga ahorro <= 0.
- No se paga importe <= 0.
- Un claim expirado no se liquida.
- Solo el oráculo configurado puede atestar.
- Solo el administrador puede cambiar configuración.
- Los cambios de configuración deben ser visibles/auditables.
- Ninguna transferencia puede exceder el cap por claim ni el balance disponible.
- Los fallos de token transfer no dejan el claim marcado como pagado.

## Operational controls
- Separar claves de admin, oracle y sponsor.
- Usar cuentas de Testnet sin valor real.
- No pegar seeds en issues, logs, screenshots ni chat.
- Registrar versión del contrato y del método de cálculo.
- Alertar sobre fallos de RPC, liquidaciones y cambios de configuración.
- Antes de mainnet: auditoría independiente, revisión agronómica, pruebas de carga y plan de incidentes.

## Known limitation

El scaffold inicial usa una dirección de oráculo autorizada y por tanto es un modelo de confianza centralizado. No es un oracle network ni un sensor attestation hardware-rooted.
