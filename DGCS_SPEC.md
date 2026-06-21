# DGCS v0.1 Specification

**Deterministic Geometric Cognition Substrate**

*Revision: v0.1 — 2026-06-21*

---

## Purpose

This document defines the minimum viable substrate that any DGCS-compliant cognitive engine must implement. It is a precision architecture specification, not an implementation guide.

The reference implementation is the **Bridge-Builder Reasoner (BBR)** in `src/bridge_builder/`, using the shared substrate in `src/geom/`.

---

## 1. What Is DGCS?

A **Deterministic Geometric Cognition Substrate** is a layered architecture for building reasoners that are:

- **Deterministic** — identical inputs yield identical outputs across all runs, machines, and compilers
- **Geometric** — cognitive state is encoded in integer vectors in $\mathbb{Z}^3$; operators are integer linear maps
- **Auditable** — every transition carries a canonical signature forming a merkle-like chain
- **Multi-scale** — primitive AQs compose into super-AQs, which compose into meta-AQs

DGCS does **not** require:
- probabilistic inference
- floating-point arithmetic
- neural network parameters
- external runtime dependencies beyond a Rust toolchain

---

## 2. Required Layers

A DGCS-compliant reasoner must implement all six layers:

### Layer 1: Primitive AQs

- Discrete geometric states in $\mathbb{Z}^3$ with canonical coordinates
- Each primitive encodes a domain-specific distinction (e.g. Stable/Unstable, Energized/DeEnergized)
- Coordinates use the shared **[drift, symmetry, stability]** semantic axis layout:
  - `coords[0]` = drift axis — deviation from structural baseline
  - `coords[1]` = symmetry axis — balance contribution (for active-zone meta-AQs)
  - `coords[2]` = stability axis — load/capacity measure
- Each primitive carries a canonical signature: `paq:<kind>|coords:[x,y,z]`

### Layer 2: Super-AQs

- Governed composites of 2–4 primitive AQs
- Gestalt = component-wise sum of all primitive coordinates
- Each super-AQ carries a signature encoding: kind, gestalt, component count

### Layer 3: Meta-AQs

- Governed composites of 2+ super-AQs representing an entire semantic region
- Gestalt = component-wise sum of all super-AQ gestalts
- At least one meta-AQ kind must be designated "active" for symmetry measurement
- Each meta-AQ carries a signature encoding: kind, gestalt, component count

### Layer 4: Operators at Three Scales

| Level      | Operates on      | Examples (BBR)                        |
|------------|------------------|---------------------------------------|
| Primitive  | `PrimitiveAQ.coords` | Stabilize, Align, LoadShift, Connect |
| Super      | `SuperAQ.gestalt`    | ReinforceBeam, TightenJoint          |
| Meta       | Meta-AQ gestalt  | BalanceSpans, DistributeLoad(+6)      |

All operators are integer $3 \times 3$ matrices. Non-commuting operators are mandatory for semantic structure.

Meta-operators must expose a `coherence_bonus() -> i32` method. Coherence bonuses are deterministic and governance-specified (not learned).

### Layer 5: Topological Invariant Gate

- Checks structural invariants before accepting any trajectory
- Returns `geom::topology_gate::TopologyStatus::Valid` or `TopologyStatus::Invalid(reason)`
- A rejected design activates the correction buffer immediately
- Domain-specific implementations determine what constitutes a violation
- Common patterns:
  - Disconnected/Isolated primitive in an active zone
  - 2+ destabilising primitives in a single super-AQ

### Layer 6: Resonance-Modulated Arbitration

#### 6a. Resonance Field

- State in $\mathbb{Z}^3$: `[stability_weight, symmetry_weight, drift_penalty]`
- Default field: `[1, 1, 1]` (unit weights)
- Field is itself a cognitive object (recursive resonance): it can be transformed and stabilised

#### 6b. Resonance Score

Uses `geom::resonance_field::ResonanceScore`:
```
score = stability + symmetry - drift + structural_coherence
```
where each component is computed by the domain-specific `evaluate_resonance()` function using the field weights.

#### 6c. Arbitration

- Evaluates all candidates in original declaration order (for deterministic signature)
- Rejects topology-invalid candidates
- Sorts valid candidates by total score (descending)
- Selects `winner = argmax score`
- Archives all valid results to the correction buffer

#### 6d. Correction Buffer

Uses `geom::correction_buffer::CorrectionBuffer`:
- Archives `(plan_id, signature)` pairs for all valid designs
- Provides instant fallback when the primary trajectory drifts or fails topology
- No recomputation required at fallback time

---

## 3. Shared Substrate (geom/)

These types are shared across all DGCS reasoners and must not be domain-specialised:

| Module                    | Type(s)                         | Role |
|---------------------------|---------------------------------|------|
| `geom::topology_gate`     | `TopologyStatus`                | Shared outcome for all topology gates |
| `geom::resonance_field`   | `ResonanceField`, `ResonanceScore` | Shared field state and scoring struct |
| `geom::correction_buffer` | `CorrectionBuffer`              | Shared fallback archive |

Domain-specific evaluation functions (`check_topology`, `evaluate_resonance`) live in `<domain>/evaluation.rs` and return these shared types.

---

## 4. Tournament Signature Format

Every DGCS tournament must emit a canonical, replay-stable signature:

```
<domain>:metaAQ|winner:<id>|<id1>:<score1>|<id2>:<score2>|...|<idN>:invalid
```

Examples:
```
bridge:metaAQ|winner:B|A:46|B:62|C:invalid|D:50
grid:metaAQ|winner:B|A:41|B:57|C:invalid|D:50
```

Requirements:
- Domain prefix identifies the reasoner
- Plans listed in original declaration order (not score order)
- Rejected plans use the literal token `invalid`
- Signature must be identical across any number of replays

---

## 5. Reference Implementations

### Bridge-Builder Reasoner (v0.1 Reference)

Path: `src/bridge_builder/`
Binary: `cargo run --bin bridge_builder`

Demonstrates: primitive AQs, super-AQs (Beam/Joint/Support/Span), meta-AQs (LeftSpan/CentralSupport/RightSpan), three operator levels, topology gate (unsupported span, unstable joint), resonance arbitration with DistributeLoad coherence bonus, correction buffer.

### Power Grid Reasoner (v0.1 Validation)

Path: `src/power_grid/`
Binary: `cargo run --bin power_grid_reasoner`

Demonstrates: **same substrate, different ontology**. Uses identical `geom::` types with a power-grid-specific primitive set (Energized/DeEnergized/Overloaded/...) and topology rules (isolated circuit node, overload cascade). Confirms that DGCS is domain-independent.

---

## 6. Invariant Contract

All DGCS engines must uphold the four invariants:

| Invariant    | Requirement |
|--------------|-------------|
| Determinism  | `f(x) = f(x)` — identical inputs always yield identical outputs |
| Consistency  | All transformations preserve governed unit set membership |
| Closure      | No operator may produce an out-of-domain state |
| Auditability | Every cognitive object carries a canonical, replay-stable signature |

---

## 7. Versioning

| Version | Description |
|---------|-------------|
| v0.1    | Primitive AQs, super-AQs, meta-AQs, three operator levels, topology gate, resonance field, correction buffer, two reference reasoners (BBR, Power Grid) |

Future versions will add: operator discovery, multi-tiered resonance memory, formal topological verification, STIF cross-system handshakes, and multi-agent tournament arbitration (see Section 12 of README).
