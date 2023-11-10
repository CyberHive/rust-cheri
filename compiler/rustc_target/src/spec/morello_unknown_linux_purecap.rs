use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_musl_base::opts();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-march=morello+c64", "-mabi=purecap"]);
    base.max_atomic_width = Some(128);
    base.has_thread_local = false;

    Target {
        llvm_target: "aarch64-unknown-linux-musl_purecap".into(),
        pointer_width: 64,
        data_layout: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".into(),
        arch: "morello+c64".into(),

        options: TargetOptions {
            pointer_type_width: Some(128),
            linker: Some("lld".into()),
            llvm_abiname: "purecap".into(),
            features: "+v8.2a,+morello,+c64".into(),
            mcount: "\u{1}_mcount".into(),
            ..base
        },
    }
}
