use super::{InlineAsmArch, InlineAsmType};
use crate::spec::{RelocModel, Target};
use rustc_data_structures::fx::FxIndexSet;
use rustc_macros::HashStable_Generic;
use rustc_span::{sym, Symbol};
use std::fmt;

def_reg_class! {
    Xtensa XtensaInlineAsmRegClass {
        reg,
        freg,
        breg,
        qreg,
    }
}

impl XtensaInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I8, I16, I32; },
            Self::breg => types! { bool: I1; },
            Self::freg => types! { fp: F32; dfpaccel: F64; },
            Self::qreg => types! { esp32s3: VecI8(16), VecI16(8), VecI32(4), VecF32(4); },
        }
    }
}

// Xtensa has lots of features - macro to reduce boiler plate
macro_rules! feature {
    ($fnname:ident, $feature:expr) => {
        fn $fnname(
            _arch: InlineAsmArch,
            _reloc_model: RelocModel,
            target_features: &FxIndexSet<Symbol>,
            _target: &Target,
            _is_clobber: bool,
        ) -> Result<(), &'static str> {
            if target_features.contains(&$feature) {
                Ok(())
            } else {
                Err(concat!("target does not support ", stringify!($feature), " registers"))
            }
        }
    };
}

feature!(has_fp, sym::fp);
feature!(has_dfpaccel, sym::dfpaccel);
feature!(has_bool, sym::bool);
feature!(has_xloop, sym::xloop);
feature!(has_extendedl32r, sym::extendedl32r);
feature!(has_s32c1i, sym::s32c1i);
feature!(has_mac16, sym::mac16);
feature!(has_windowed, sym::windowed);
feature!(has_debug, sym::debug);
feature!(has_memctl, sym::memctl);
feature!(has_atomctl, sym::atomctl);
feature!(has_exception, sym::exception);
feature!(has_highpriinterrupts, sym::highpriinterrupts);
feature!(has_coprocessor, sym::coprocessor);
feature!(has_rvector, sym::rvector);
feature!(has_timerint, sym::timerint);
feature!(has_interrupt, sym::interrupt);
feature!(has_prid, sym::prid);
feature!(has_miscsr, sym::miscsr);
feature!(has_threadptr, sym::threadptr);
feature!(has_esp32s3, sym::esp32s3);

fn has_expstate(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    match target.cpu.as_ref() {
        "esp32" => Ok(()),
        _ => Err("target does not support expstate registers"),
    }
}
fn has_gpio_out(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    match target.cpu.as_ref() {
        "esp32-s2" => Ok(()),
        _ => Err("target does not support gpio_out registers"),
    }
}

// FIXME sometimes there isn't a frame pointer at all?
fn frame_pointer_is_a7(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    target_features: &FxIndexSet<Symbol>,
    _target: &Target,
    _is_clobber: bool,
) -> bool {
    target_features.contains(&sym::windowed)
}

fn frame_pointer_a7(
    arch: InlineAsmArch,
    reloc_model: RelocModel,
    target_features: &FxIndexSet<Symbol>,
    target: &Target,
    is_clobber: bool,
) -> Result<(), &'static str> {
    if frame_pointer_is_a7(arch, reloc_model, target_features, target, is_clobber) {
        Err("the frame pointer (a7) cannot be used as an operand for inline asm")
    } else {
        Ok(())
    }
}

fn frame_pointer_a15(
    arch: InlineAsmArch,
    reloc_model: RelocModel,
    target_features: &FxIndexSet<Symbol>,
    target: &Target,
    is_clobber: bool,
) -> Result<(), &'static str> {
    if !frame_pointer_is_a7(arch, reloc_model, target_features, target, is_clobber) {
        Err("the frame pointer (a15) cannot be used as an operand for inline asm")
    } else {
        Ok(())
    }
}

def_regs! {
    Xtensa XtensaInlineAsmReg XtensaInlineAsmRegClass {
        a2: reg = ["a2"],
        a3: reg = ["a3"],
        a4: reg = ["a4"],
        a5: reg = ["a5"],
        a6: reg = ["a6"],
        a7: reg = ["a7"] % frame_pointer_a7,
        a8: reg = ["a8"],
        a9: reg = ["a9"],
        a10: reg = ["a10"],
        a11: reg = ["a11"],
        a12: reg = ["a12"],
        a13: reg = ["a13"],
        a14: reg = ["a14"],
        a15: reg = ["a15"] % frame_pointer_a15,
        sar: reg = ["sar"],
        configid0: reg = ["configid0"],
        configid1: reg = ["configid1"],
        lbeg: reg = ["lbeg"] % has_xloop,
        lend: reg = ["lend"] % has_xloop,
        lcount: reg = ["lcount"] % has_xloop,
        litbase: reg = ["litbase"] % has_extendedl32r,
        scompare1: reg = ["scompare1"] % has_s32c1i,
        acclo: reg = ["acclo"] % has_mac16,
        acchi: reg = ["acchi"] % has_mac16,
        m0: reg = ["m0"] % has_mac16,
        m1: reg = ["m1"] % has_mac16,
        m2: reg = ["m2"] % has_mac16,
        m3: reg = ["m3"] % has_mac16,
        windowbase: reg = ["windowbase"] % has_windowed,
        windowstart: reg = ["windowstart"] % has_windowed,
        ddr: reg = ["ddr"] % has_debug,
        ibreakenable: reg = ["ibreakenable"] % has_debug,
        ibreaka0: reg = ["ibreaka0"] % has_debug,
        ibreaka1: reg = ["ibreaka1"] % has_debug,
        dbreaka0: reg = ["dbreaka0"] % has_debug,
        dbreaka1: reg = ["dbreaka1"] % has_debug,
        dbreakc0: reg = ["dbreakc0"] % has_debug,
        dbreakc1: reg = ["dbreakc1"] % has_debug,
        icount: reg = ["icount"] % has_debug,
        icountlevel: reg = ["icountlevel"] % has_debug,
        debugcause: reg = ["debugcause"] % has_debug,
        memctl: reg = ["memctl"] % has_memctl,
        atomctl: reg = ["atomctl"] % has_atomctl,
        ps: reg = ["ps"] % has_exception,
        epc1: reg = ["epc1"] % has_exception,
        epc2: reg = ["epc2"] % has_highpriinterrupts,
        epc3: reg = ["epc3"] % has_highpriinterrupts,
        epc4: reg = ["epc4"] % has_highpriinterrupts,
        epc5: reg = ["epc5"] % has_highpriinterrupts,
        epc6: reg = ["epc6"] % has_highpriinterrupts,
        epc7: reg = ["epc7"] % has_highpriinterrupts,
        depc: reg = ["depc"] % has_exception,
        eps2: reg = ["eps2"] % has_highpriinterrupts,
        eps3: reg = ["eps3"] % has_highpriinterrupts,
        eps4: reg = ["eps4"] % has_highpriinterrupts,
        eps5: reg = ["eps5"] % has_highpriinterrupts,
        eps6: reg = ["eps6"] % has_highpriinterrupts,
        eps7: reg = ["eps7"] % has_highpriinterrupts,
        excsave1: reg = ["excsave1"] % has_exception,
        excsave2: reg = ["excsave2"] % has_highpriinterrupts,
        excsave3: reg = ["excsave3"] % has_highpriinterrupts,
        excsave4: reg = ["excsave4"] % has_highpriinterrupts,
        excsave5: reg = ["excsave5"] % has_highpriinterrupts,
        excsave6: reg = ["excsave6"] % has_highpriinterrupts,
        excsave7: reg = ["excsave7"] % has_highpriinterrupts,
        exccause: reg = ["exccause"] % has_exception,
        excvaddr: reg = ["excvaddr"] % has_exception,
        cpenable: reg = ["cpenable"] % has_coprocessor,
        vecbase: reg = ["vecbase"] % has_rvector,
        interrupt: reg = ["interrupt"] % has_interrupt,
        intclear: reg = ["intclear"] % has_interrupt,
        intenable: reg = ["intenable"] % has_interrupt,
        prid: reg = ["prid"] % has_prid,
        ccount: reg = ["ccount"] % has_timerint,
        ccompare0: reg = ["ccompare0"] % has_timerint,
        ccompare1: reg = ["ccompare1"] % has_timerint,
        ccompare2: reg = ["ccompare2"] % has_timerint,
        misc0: reg = ["misc0"] % has_miscsr,
        misc1: reg = ["misc1"] % has_miscsr,
        misc2: reg = ["misc2"] % has_miscsr,
        misc3: reg = ["misc3"] % has_miscsr,
        threadptr: reg = ["threadptr"] % has_threadptr,
        fcr: reg = ["fcr"] % has_dfpaccel,
        fsr: reg = ["fsr"] % has_dfpaccel,
        f64r_lo: reg = ["f64r_lo"] % has_dfpaccel,
        f64r_hi: reg = ["f64r_hi"] % has_dfpaccel,
        f64s: reg = ["f64s"] % has_dfpaccel,
        f0: freg = ["f0"] % has_fp,
        f1: freg = ["f1"] % has_fp,
        f2: freg = ["f2"] % has_fp,
        f3: freg = ["f3"] % has_fp,
        f4: freg = ["f4"] % has_fp,
        f5: freg = ["f5"] % has_fp,
        f6: freg = ["f6"] % has_fp,
        f7: freg = ["f7"] % has_fp,
        f8: freg = ["f8"] % has_fp,
        f9: freg = ["f9"] % has_fp,
        f10: freg = ["f10"] % has_fp,
        f11: freg = ["f11"] % has_fp,
        f12: freg = ["f12"] % has_fp,
        f13: freg = ["f13"] % has_fp,
        f14: freg = ["f14"] % has_fp,
        f15: freg = ["f15"] % has_fp,
        br: reg = ["br"] % has_bool,
        b0: breg = ["b0"] % has_bool,
        b1: breg = ["b1"] % has_bool,
        b2: breg = ["b2"] % has_bool,
        b3: breg = ["b3"] % has_bool,
        b4: breg = ["b4"] % has_bool,
        b5: breg = ["b5"] % has_bool,
        b6: breg = ["b6"] % has_bool,
        b7: breg = ["b7"] % has_bool,
        b8: breg = ["b8"] % has_bool,
        b9: breg = ["b9"] % has_bool,
        b10: breg = ["b10"] % has_bool,
        b11: breg = ["b11"] % has_bool,
        b12: breg = ["b12"] % has_bool,
        b13: breg = ["b13"] % has_bool,
        b14: breg = ["b14"] % has_bool,
        b15: breg = ["b15"] % has_bool,

        // Custom TIE extensions - https://en.wikipedia.org/wiki/Tensilica_Instruction_Extension
        gpio_out: reg = ["gpio_out"] % has_gpio_out,
        expstate: reg = ["expstate"] % has_expstate,

        // ESP32S3 specific TIE extensions
        q0: qreg = ["q0"] % has_esp32s3,
        q1: qreg = ["q1"] % has_esp32s3,
        q2: qreg = ["q2"] % has_esp32s3,
        q3: qreg = ["q3"] % has_esp32s3,
        q4: qreg = ["q4"] % has_esp32s3,
        q5: qreg = ["q5"] % has_esp32s3,
        q6: qreg = ["q6"] % has_esp32s3,
        q7: qreg = ["q7"] % has_esp32s3,

        accx_0: reg = ["accx_0"] % has_esp32s3,
        accx_1: reg = ["accx_1"] % has_esp32s3,
        qacc_h_0: reg = ["qacc_h_0"] % has_esp32s3,
        qacc_h_1: reg = ["qacc_h_1"] % has_esp32s3,
        qacc_h_2: reg = ["qacc_h_2"] % has_esp32s3,
        qacc_h_3: reg = ["qacc_h_3"] % has_esp32s3,
        qacc_h_4: reg = ["qacc_h_4"] % has_esp32s3,
        qacc_l_0: reg = ["qacc_l_0"] % has_esp32s3,
        qacc_l_1: reg = ["qacc_l_1"] % has_esp32s3,
        qacc_l_2: reg = ["qacc_l_2"] % has_esp32s3,
        qacc_l_3: reg = ["qacc_l_3"] % has_esp32s3,
        qacc_l_4: reg = ["qacc_l_4"] % has_esp32s3,
        fft_bit_width: reg = ["fft_bit_width"] % has_esp32s3,
        sar_byte: reg = ["sar_byte"] % has_esp32s3,
        ua_state_0: reg = ["ua_state_0"] % has_esp32s3,
        ua_state_1: reg = ["ua_state_1"] % has_esp32s3,
        ua_state_2: reg = ["ua_state_2"] % has_esp32s3,
        ua_state_3: reg = ["ua_state_3"] % has_esp32s3,

        #error = ["a0"] => "a0 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["sp", "a1"] => "sp is used internally by LLVM and cannot be used as an operand for inline asm",
    }
}

impl XtensaInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
