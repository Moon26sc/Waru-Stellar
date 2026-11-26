# Waru — Water savings, verifiable incentives

**Waru convierte ahorros hídricos medidos en campo en incentivos verificables para pequeños y medianos agricultores.**

Inspirado en los *Waru Waru*, sistemas andinos de manejo agrícola e hídrico, Waru conecta medición, validación y pagos programables sobre Stellar.

> Estado honesto: esta entrega es una fundación de producto, arquitectura y scaffold. No incluye sensores desplegados, un oráculo productivo, un contrato desplegado ni pagos USDC ejecutados. No publicar esos hitos como hechos hasta adjuntar evidencia verificable.

## El problema

En valles agrícolas con estrés hídrico, medir el ahorro de agua y distribuir incentivos a pequeños productores puede costar demasiado. Los programas de sostenibilidad necesitan una forma auditable de convertir resultados medibles en pagos pequeños, sin depender de procesos manuales costosos.

## La solución

1. Un dispositivo, gateway o simulador controlado genera lecturas de riego y humedad.
2. Un servicio de validación calcula el ahorro contra una línea base acordada y aplica reglas agronómicas.
3. Un oráculo autorizado firma una atestación compacta; los datos personales y telemetría detallada permanecen fuera de la cadena.
4. Un contrato Soroban verifica autorización, unicidad, límites y fondos disponibles.
5. El contrato libera el incentivo en un activo Stellar compatible (USDC de Testnet para el entorno de demostración).
6. Un recibo permite verificar el evento y la transacción en Stellar.

## Qué prueba cada capa

| Capa | Qué demuestra | Qué NO demuestra por sí sola |
|---|---|---|
| Sensor / simulador | Que se generó una lectura | Que el sensor está calibrado o que la lectura es cierta |
| Validador | Que una regla computable aceptó una observación | Que el modelo agronómico sea correcto |
| Firma del oráculo | Que una clave autorizada atestó un payload | Que la fuente física no fue manipulada |
| Soroban | Que el contrato aplicó sus reglas y emitió/ejecutó una transferencia | Que el ahorro físico ocurrió |
| Stellar RPC / explorer | Que una transacción y estado existen en la red consultada | La verdad agronómica fuera de cadena |

## Alcance del MVP

**Incluido**
- Un programa patrocinador.
- Registro de agricultores mediante direcciones Stellar.
- Un oráculo autorizado.
- Línea base y observación de ahorro expresadas en unidades explícitas.
- Evidencia resumida por hash.
- Fondo en un activo Stellar de prueba.
- Pago único, limitado y auditable.
- Panel web de lectura y flujo de wallet para operaciones firmadas.

**Fuera del MVP**
- Hardware IoT propio y despliegue de sensores a escala.
- Modelo universal de ahorro para todos los cultivos.
- Token propio, NFT, DAO, marketplace, crédito agrícola o scoring financiero.
- Mainnet, custodia de fondos reales y promesas de rendimiento.
- Identidad civil/KYC almacenada on-chain.
- Automatización de pagos sin una fuente de datos confiable y un patrocinador financiado.

## Arquitectura

```text
Sensor / Gateway / Simulator
          |
          v
Ingestion API ----> Calibration & Validation
                           |
                           v
                    Evidence Manifest
                    (canonical JSON + SHA-256)
                           |
                           v
                    Oracle signer
                           |
                           v
Web app / Wallet ---> Soroban contract <---- Sponsor funding
                           |
                           v
                  Stellar Asset / USDC
                           |
                           v
                 Farmer wallet + receipt
```

## Estructura del repositorio

- `docs/TECHNICAL_SPEC.md` — componentes, datos, reglas y state machine.
- `docs/PROTOCOL.md` — semántica de evidencia y límites de las pruebas.
- `docs/SECURITY.md` — amenazas y controles.
- `docs/BRANDING.md` — identidad verbal y visual.
- `docs/SHIP_IT_PLAN.md` — entregables y criterios del buildathon.
- `docs/DEMO_SCRIPT.md` — guion verificable de demo.
- `docs/ROADMAP.md` — fases y gates.
- `docs/API.md` — interfaz propuesta.
- `docs/adr/0001-architecture.md` — decisiones iniciales.
- `contracts/waru/` — scaffold Soroban.
- `apps/web/` — boundary de frontend.
- `packages/stellar/` — boundary de SDK, wallet y transacciones.
- `.github/workflows/ci.yml` — CI inicial.
- `.env.example` — variables sin secretos.

## Desarrollo local

Requisitos: Rust 1.93+, Stellar CLI compatible con Soroban SDK 30, y target de contrato administrado por Stellar CLI.

```bash
stellar --version
rustc --version
cd contracts/waru
cargo test
stellar contract build
```

El build correcto del contrato se hace con `stellar contract build`, no con `cargo build --target wasm32-unknown-unknown`.

## Configuración de Testnet

No fijar direcciones inventadas en el repositorio. Configurar después de crear/verificar cada recurso:

- RPC de Stellar Testnet.
- ID del contrato desplegado.
- Dirección del asset de prueba elegido.
- Dirección del administrador y del oráculo.
- Cuenta patrocinadora con fondos de prueba.

Ver `.env.example`. Nunca guardar seeds, secret keys ni tokens de acceso.

## Definition of Done — MVP verificable

- [ ] Build reproducible del contrato.
- [ ] Tests de reglas, autorización, replay, límites y balance.
- [ ] Deploy en Stellar Testnet; guardar Contract ID y hash de deploy.
- [ ] Fondeo del contrato desde una cuenta patrocinadora; guardar hash.
- [ ] Flujo firmado por wallet, no por un script que suplante al usuario.
- [ ] Una observación válida produce exactamente un pago.
- [ ] Repetir el mismo identificador no produce un segundo pago.
- [ ] Lectura del evento, estado y transferencia vía RPC/explorer.
- [ ] UI muestra estados reales y errores; no usa saldos ficticios.
- [ ] Video de demo y manifiesto de evidencia reproducible.

## Estado actual y transparencia

El repositorio original contiene una descripción de producto y posiciona Waru para Stellar Codember Perú 2026. Esta fundación añade una propuesta de arquitectura y scaffold; no certifica despliegue, integración de sensores, calibración ni pagos reales. Mantener el registro de evidencia en `docs/PROTOCOL.md` actualizado con hashes y enlaces reales.

## Licencia

Pendiente de elección explícita por el equipo. No asumir que el código previo o de terceros puede relicenciarse.
