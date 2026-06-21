use crate::bridge_builder::{
    arbitration::{run_tournament, BridgeDesign, Tournament},
    correction_buffer::CorrectionBuffer,
    meta_aq::{MetaAQ, MetaAQKind},
    operators::meta_ops::MetaOp,
    primitive_aq::{PrimitiveAQ, PrimitiveAQKind},
    super_aq::{SuperAQ, SuperAQKind},
};

use MetaAQKind::*;
use PrimitiveAQKind::*;
use SuperAQKind::*;

// ── helpers ─────────────────────────────────────────────────────────────────

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

// ── Design A: Standard Truss ──────────────────────────────────────────────
// Target: stability=32, symmetry=18, drift=4, coherence=0 → score=46
//
// Coordinate proof (coords = [drift, sym, stab]):
//   LeftSpan:       Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]+NonLoadBearing[0,1,2]
//                   → x=0, y=9, z=12
//   CentralSupport: Misaligned[2,1,2]+NonLoadBearing[0,1,2]+Misaligned[2,1,2]+NonLoadBearing[0,1,2]
//                   → x=4, y=4, z=8
//   RightSpan:      Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]+NonLoadBearing[0,1,2]
//                   → x=0, y=9, z=12
//   stability = z_LS+z_CS+z_RS = 12+8+12 = 32 ✓
//   symmetry  = y_LS+y_RS      = 9+9     = 18 ✓
//   drift     = x_LS+x_CS+x_RS = 0+4+0   = 4  ✓
//   score     = 32+18-4+0      = 46 ✓
fn design_a() -> BridgeDesign {
    BridgeDesign {
        id: "A".to_string(),
        meta_aqs: vec![
            meta_aq(LeftSpan, &[
                (Beam,  &[Stable, Aligned]),
                (Joint, &[Connected, NonLoadBearing]),
            ]),
            meta_aq(CentralSupport, &[
                (Support, &[Misaligned, NonLoadBearing]),
                (Span,    &[Misaligned, NonLoadBearing]),
            ]),
            meta_aq(RightSpan, &[
                (Beam,  &[Stable, Aligned]),
                (Joint, &[Connected, NonLoadBearing]),
            ]),
        ],
        meta_ops: vec![MetaOp::BalanceSpans],
    }
}

// ── Design B: Optimised Arch ──────────────────────────────────────────────
// Target: stability=41, symmetry=22, drift=1, coherence=0 → score=62
//
// Coordinate proof:
//   LeftSpan:       Stable[0,3,4]+Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]
//                   → x=0, y=11, z=14
//   CentralSupport: Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]+LoadBearing[1,2,3]
//                   → x=1, y=10, z=13
//   RightSpan:      Stable[0,3,4]+Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]
//                   → x=0, y=11, z=14
//   stability = 14+13+14 = 41 ✓
//   symmetry  = 11+11    = 22 ✓
//   drift     = 0+1+0    = 1  ✓
//   score     = 41+22-1+0 = 62 ✓
fn design_b() -> BridgeDesign {
    BridgeDesign {
        id: "B".to_string(),
        meta_aqs: vec![
            meta_aq(LeftSpan, &[
                (Beam,  &[Stable, Stable]),
                (Joint, &[Aligned, Connected]),
            ]),
            meta_aq(CentralSupport, &[
                (Support, &[Stable, Aligned]),
                (Span,    &[Connected, LoadBearing]),
            ]),
            meta_aq(RightSpan, &[
                (Beam,  &[Stable, Stable]),
                (Joint, &[Aligned, Connected]),
            ]),
        ],
        meta_ops: vec![MetaOp::StabilizeBridge],
    }
}

// ── Design C: Cantilevered Failure ────────────────────────────────────────
// Target: stability=28, symmetry=10, drift=7, topology=Invalid → Rejected
//
// Coordinate proof:
//   LeftSpan:       NonLoadBearing[0,1,2]+Misaligned[2,1,2]+Misaligned[2,1,2]+NonLoadBearing[0,1,2]
//                   → x=4, y=4, z=8
//   CentralSupport: Stable[0,3,4]+Aligned[0,3,3]+Connected[0,2,3]+Connected[0,2,3]
//                   → x=0, y=10, z=13
//   RightSpan:      NonLoadBearing[0,1,2]+Misaligned[2,1,2]+NonLoadBearing[0,1,2]+Disconnected[1,3,1]
//                   → x=3, y=6, z=7
//   stability = 8+13+7  = 28 ✓
//   symmetry  = 4+6     = 10 ✓
//   drift     = 4+0+3   = 7  ✓
//   topology  = Invalid (Disconnected in RightSpan → unsupported span) ✓
fn design_c() -> BridgeDesign {
    BridgeDesign {
        id: "C".to_string(),
        meta_aqs: vec![
            meta_aq(LeftSpan, &[
                (Beam,  &[NonLoadBearing, Misaligned]),
                (Joint, &[Misaligned, NonLoadBearing]),
            ]),
            meta_aq(CentralSupport, &[
                (Support, &[Stable, Aligned]),
                (Span,    &[Connected, Connected]),
            ]),
            meta_aq(RightSpan, &[
                (Beam,  &[NonLoadBearing, Misaligned]),
                (Joint, &[NonLoadBearing, Disconnected]), // ← topology violation
            ]),
        ],
        meta_ops: vec![],
    }
}

// ── Design D: Load-Distributed Truss ─────────────────────────────────────
// Target: stability=35, symmetry=12, drift=3, coherence=6 → score=50
//
// Coordinate proof:
//   LeftSpan:       Misaligned[2,1,2]+NonLoadBearing[0,1,2]+NonLoadBearing[0,1,2]+NonLoadBearing[0,1,2]
//                   → x=2, y=4, z=8
//   CentralSupport: Stable[0,3,4]+Stable[0,3,4]+Stable[0,3,4]+LoadBearing[1,2,3]
//                   → x=1, y=11, z=15
//   RightSpan:      Stable[0,3,4]+Stable[0,3,4]+NonLoadBearing[0,1,2]+NonLoadBearing[0,1,2]
//                   → x=0, y=8, z=12
//   stability = 8+15+12 = 35 ✓
//   symmetry  = 4+8     = 12 ✓
//   drift     = 2+1+0   = 3  ✓
//   coherence = DistributeLoad.coherence_bonus() = 6 ✓
//   score     = 35+12-3+6 = 50 ✓
fn design_d() -> BridgeDesign {
    BridgeDesign {
        id: "D".to_string(),
        meta_aqs: vec![
            meta_aq(LeftSpan, &[
                (Beam,  &[Misaligned, NonLoadBearing]),
                (Joint, &[NonLoadBearing, NonLoadBearing]),
            ]),
            meta_aq(CentralSupport, &[
                (Support, &[Stable, Stable]),
                (Span,    &[Stable, LoadBearing]),
            ]),
            meta_aq(RightSpan, &[
                (Beam,  &[Stable, Stable]),
                (Joint, &[NonLoadBearing, NonLoadBearing]),
            ]),
        ],
        meta_ops: vec![MetaOp::DistributeLoad, MetaOp::BalanceSpans],
    }
}

// ── Entry point ───────────────────────────────────────────────────────────

pub fn run_demo() -> Tournament {
    let designs = vec![design_a(), design_b(), design_c(), design_d()];
    let mut buffer = CorrectionBuffer::new();
    run_tournament(&designs, &mut buffer)
}
