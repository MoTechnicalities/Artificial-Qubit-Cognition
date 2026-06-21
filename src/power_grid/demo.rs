use crate::geom::correction_buffer::CorrectionBuffer;
use crate::power_grid::{
    arbitration::{run_tournament, GridDesign, GridTournament},
    meta_aq::{MetaAQ, MetaAQKind},
    operators::meta_ops::MetaOp,
    primitive_aq::{PrimitiveAQ, PrimitiveAQKind},
    super_aq::{SuperAQ, SuperAQKind},
};

use MetaAQKind::*;
use PrimitiveAQKind::*;
use SuperAQKind::*;

fn paq(kind: PrimitiveAQKind) -> PrimitiveAQ {
    PrimitiveAQ::new(kind)
}

fn super_aq(kind: SuperAQKind, ks: &[PrimitiveAQKind]) -> SuperAQ {
    SuperAQ::new(kind, ks.iter().map(|&k| paq(k)).collect())
}

fn meta_aq(kind: MetaAQKind, parts: &[(SuperAQKind, &[PrimitiveAQKind])]) -> MetaAQ {
    MetaAQ::new(
        kind,
        parts.iter().map(|&(sk, ps)| super_aq(sk, ps)).collect(),
    )
}

// ── Routing Plan A: Standard Routing ─────────────────────────────────────
// Target: stability=30, symmetry=16, drift=5, coherence=0 → score=41
//
// Coordinate proof (coords = [drift, sym, stab]):
//   NorthZone: Circuit(Protected[0,3,3], Connected[0,2,3])
//              Breaker(Nominal[1,2,3], Standby[0,1,2])
//              → x=0+0+1+0=1, y=3+2+2+1=8, z=3+3+3+2=11
//   CentralSubstation: Load(Protected[0,3,3], Connected[0,2,3])
//                      Line(Nominal[1,2,3], Standby[0,1,2])
//                      → x=0+0+1+0=1, y=3+2+2+1=8, z=3+3+3+2=11
//   SouthZone: Circuit(Protected[0,3,3], Connected[0,2,3])
//              Breaker(Nominal[1,2,3], Standby[0,1,2])
//              → x=1, y=8, z=11
//
// Wait: stability=11+11+11=33 ≠ 30. Let me recalculate properly.
//
// NorthZone:   Circuit(Protected[0,3,3], Connected[0,2,3])
//              Breaker(Nominal[1,2,3], Standby[0,1,2])
//              → x=1, y=8, z=11
// CentralSubstation: Load(Nominal[1,2,3], Overloaded[2,1,3])
//                    Line(Idle[0,1,1], Idle[0,1,1])
//                    → x=1+2+0+0=3, y=2+1+1+1=5, z=3+3+1+1=8
// SouthZone:   Circuit(Protected[0,3,3], Connected[0,2,3])
//              Breaker(Nominal[1,2,3], Standby[0,1,2])
//              → x=1, y=8, z=11
//   stability = 11+8+11 = 30 ✓
//   symmetry  = y_NZ+y_SZ = 8+8 = 16 ✓
//   drift     = 1+3+1 = 5 ✓
//   score     = 30+16-5+0 = 41 ✓
fn plan_a() -> GridDesign {
    GridDesign {
        id: "A".to_string(),
        meta_aqs: vec![
            meta_aq(NorthZone, &[
                (Circuit, &[Protected, Connected]),
                (Breaker, &[Nominal, Standby]),
            ]),
            meta_aq(CentralSubstation, &[
                (Load, &[Nominal, Overloaded]),
                (Line, &[Idle, Idle]),
            ]),
            meta_aq(SouthZone, &[
                (Circuit, &[Protected, Connected]),
                (Breaker, &[Nominal, Standby]),
            ]),
        ],
        meta_ops: vec![MetaOp::BalanceZones],
    }
}

// ── Routing Plan B: High-Capacity Routing ────────────────────────────────
// Target: stability=39, symmetry=20, drift=2, coherence=0 → score=57
//
// Coordinate proof:
//   NorthZone:  Circuit(Energized[0,3,4], Protected[0,3,3])
//               Breaker(Nominal[1,2,3], Connected[0,2,3])
//               → x=0+0+1+0=1, y=3+3+2+2=10, z=4+3+3+3=13
//   CentralSubstation: Load(Energized[0,3,4], Protected[0,3,3])
//                      Line(Connected[0,2,3], Connected[0,2,3])
//                      → x=0, y=3+3+2+2=10, z=4+3+3+3=13
//   SouthZone:  same as NorthZone → x=1, y=10, z=13
//   stability = 13+13+13 = 39 ✓
//   symmetry  = 10+10 = 20 ✓
//   drift     = 1+0+1 = 2 ✓
//   score     = 39+20-2+0 = 57 ✓
fn plan_b() -> GridDesign {
    GridDesign {
        id: "B".to_string(),
        meta_aqs: vec![
            meta_aq(NorthZone, &[
                (Circuit, &[Energized, Protected]),
                (Breaker, &[Nominal, Connected]),
            ]),
            meta_aq(CentralSubstation, &[
                (Load, &[Energized, Protected]),
                (Line, &[Connected, Connected]),
            ]),
            meta_aq(SouthZone, &[
                (Circuit, &[Energized, Protected]),
                (Breaker, &[Nominal, Connected]),
            ]),
        ],
        meta_ops: vec![MetaOp::StabilizeGrid],
    }
}

// ── Routing Plan C: Cantilever Failure ───────────────────────────────────
// Target: stability=25, symmetry=12, drift=8, topology=Invalid → Rejected
//
// Coordinate proof:
//   NorthZone:  Circuit(Nominal[1,2,3], Standby[0,1,2])
//               Breaker(Unprotected[2,1,2], Idle[0,1,1])
//               → x=1+0+2+0=3, y=2+1+1+1=5, z=3+2+2+1=8
//   CentralSubstation: Load(Overloaded[2,1,3], Nominal[1,2,3])
//                      Line(Standby[0,1,2], Idle[0,1,1])
//                      → x=2+1+0+0=3, y=1+2+1+1=5, z=3+3+2+1=9
//   SouthZone:  Circuit(Standby[0,1,2], Nominal[1,2,3])
//               Breaker(Standby[0,1,2], Isolated[1,3,1]) ← topology violation
//               → x=0+1+0+1=2, y=1+2+1+3=7, z=2+3+2+1=8
//   stability = 8+9+8 = 25 ✓
//   symmetry  = y_NZ+y_SZ = 5+7 = 12 ✓
//   drift     = 3+3+2 = 8 ✓
//   topology  = Invalid (Isolated in SouthZone → isolated circuit node) ✓
fn plan_c() -> GridDesign {
    GridDesign {
        id: "C".to_string(),
        meta_aqs: vec![
            meta_aq(NorthZone, &[
                (Circuit, &[Nominal, Standby]),
                (Breaker, &[Unprotected, Idle]),
            ]),
            meta_aq(CentralSubstation, &[
                (Load, &[Overloaded, Nominal]),
                (Line, &[Standby, Idle]),
            ]),
            meta_aq(SouthZone, &[
                (Circuit, &[Standby, Nominal]),
                (Breaker, &[Standby, Isolated]), // ← topology violation
            ]),
        ],
        meta_ops: vec![],
    }
}

// ── Routing Plan D: Load-Shedding Routing ────────────────────────────────
// Target: stability=33, symmetry=14, drift=3, coherence=6 → score=50
//
// Coordinate proof:
//   NorthZone:  Circuit(Nominal[1,2,3], Nominal[1,2,3])
//               Breaker(Standby[0,1,2], Standby[0,1,2])
//               → x=1+1+0+0=2, y=2+2+1+1=6, z=3+3+2+2=10
//   CentralSubstation: Load(Energized[0,3,4], Protected[0,3,3])
//                      Line(Connected[0,2,3], Nominal[1,2,3])
//                      → x=0+0+0+1=1, y=3+3+2+2=10, z=4+3+3+3=13
//   SouthZone:  Circuit(Protected[0,3,3], Protected[0,3,3])
//               Breaker(Standby[0,1,2], Standby[0,1,2])
//               → x=0+0+0+0=0, y=3+3+1+1=8, z=3+3+2+2=10
//   stability = 10+13+10 = 33 ✓
//   symmetry  = y_NZ+y_SZ = 6+8 = 14 ✓
//   drift     = 2+1+0 = 3 ✓
//   coherence = ShedLoad.coherence_bonus() = 6 ✓
//   score     = 33+14-3+6 = 50 ✓
fn plan_d() -> GridDesign {
    GridDesign {
        id: "D".to_string(),
        meta_aqs: vec![
            meta_aq(NorthZone, &[
                (Circuit, &[Nominal, Nominal]),
                (Breaker, &[Standby, Standby]),
            ]),
            meta_aq(CentralSubstation, &[
                (Load, &[Energized, Protected]),
                (Line, &[Connected, Nominal]),
            ]),
            meta_aq(SouthZone, &[
                (Circuit, &[Protected, Protected]),
                (Breaker, &[Standby, Standby]),
            ]),
        ],
        meta_ops: vec![MetaOp::ShedLoad, MetaOp::BalanceZones],
    }
}

pub fn run_demo() -> GridTournament {
    let designs = vec![plan_a(), plan_b(), plan_c(), plan_d()];
    let mut buffer = CorrectionBuffer::new();
    run_tournament(&designs, &mut buffer)
}
