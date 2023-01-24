use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-pf200:64:64:64:32-p:32:32-i64:64-n32-S128-A200-P200-G200".into(),
        llvm_target: "riscv32-unknown-none-purecap".into(),
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            pointer_type_width: Some(64),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            llvm_abiname: "il32pc64".into(),
            cpu: "generic-rv32".into(),
            atomic_cas: true,
            features: "+m,+a,+c,+xcheri,+cap-mode".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
