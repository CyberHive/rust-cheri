use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".into(),
        llvm_target: "aarch64-unknown-none-elf".into(),
        pointer_width: 64,
        arch: "morello+c64".into(),

        options: TargetOptions {
            pointer_type_width: Some(128),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("lld".into()),
            llvm_abiname: "purecap".into(),
            cpu: "generic".into(),
            atomic_cas: true,
            features: "+v8.2a,+morello,+c64".into(),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
