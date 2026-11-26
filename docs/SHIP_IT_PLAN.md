# Ship It / Stellar Codember delivery plan

## Contexto público consultado
La página pública de Stellar Codember Perú describe una buildathon del 14 de noviembre al 4 de diciembre de 2026, repositorio checkpoint obligatorio y entrega en APEX con repositorio, evidencia Testnet y video de máximo 3 minutos. La página también indica que el proyecto puede integrar Stellar mediante contrato Soroban o SDK/RPC/Horizon. Confirmar siempre bases y cambios directamente con la organización.

## Objetivo de entrega
Un único flujo reproducible: sponsor funds vault -> oracle attests eligible claim -> contract validates -> contract pays farmer -> explorer receipt verifies.

## Prioridades

### P0 — fundamento
- Definir unidad, baseline y método de cálculo.
- Definir asset de Testnet y roles.
- Compilar contrato y tests.
- Documentar source_kind (simulator vs sensor).
- Crear registro de evidencia.

### P1 — flujo on-chain
- Deploy contract en Testnet.
- Configurar asset, admin y oracle.
- Fondear vault desde sponsor.
- Ejecutar settlement válido.
- Repetir claim y demostrar rechazo por replay.
- Probar claim inválido, ahorro cero, cap y fondos insuficientes.

### P2 — interfaz y demo
- Wallet connect y firma real.
- Panel de claim con origen de datos visible.
- Recibo con Contract ID y transaction hash.
- Video de máximo 3 minutos.
- Documentar herramientas de IA y código previo conforme a bases.

## Matriz de evaluación interna

| Criterio | Evidencia |
|---|---|
| Problema y usuario | Caso de uso y flujo agricultor/sponsor |
| Stellar real | Contract ID, asset, RPC y transacciones |
| Funcionalidad | Fondeo, settlement y lectura posterior |
| Seguridad | Tests de auth, replay, caps y errores |
| Honestidad | Diferenciar simulación, sensor y auditoría |
| Demo | Video reproducible con enlaces verificables |

## Submission checklist
- [ ] Confirmar reglas vigentes y elegibilidad.
- [ ] Registrar el repositorio en el Hub/APEX según el proceso oficial.
- [ ] Incluir README y documentación.
- [ ] Adjuntar evidencia de Testnet real.
- [ ] Video <= 3 minutos.
- [ ] Declarar IA y código previo según bases.
- [ ] No afirmar hardware productivo si solo se usa simulador.
