use crate::spec::{CodeModel, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::linux_ohos_base::opts();
    base.max_atomic_width = Some(64);

    Target {
        llvm_target: "riscv64-unknown-linux-ohos".into(),
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "riscv64".into(),
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: "generic-rv64".into(),
            features: "+m,+a,+f,+d,+c".into(),
            llvm_abiname: "lp64d".into(),
            ..base
        },
    }
}
