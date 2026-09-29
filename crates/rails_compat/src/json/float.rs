//! The JSON gem's float writer, `fpconv_dtoa` (json-2.21.2 `ext/json/ext/vendor/fpconv.c`, from
//! night-shift/fpconv), ported line for line. `ActiveSupport::JSON.encode` hands finite floats to
//! it, so it isn't `Float#to_s`: `1e-05.to_s` is `"1.0e-05"` where JSON writes `0.00001`, and
//! Grisu2 sometimes picks a longer digit string than the shortest round trip (`9.64032` is written
//! `9.640319999999999`). `vectors/rails_compat_json.json` pins both.
use std::io::Write;

/// `frac * 2^exp`.
#[derive(Clone, Copy)]
struct Fp {
    frac: u64,
    exp: i32,
}

/// Cached powers of ten, 10^-348 to 10^340 in steps of 8.
#[rustfmt::skip]
const POWERS_TEN: [(u64, i32); 87] = [
    (18054884314459144840, -1220), (13451937075301367670, -1193), (10022474136428063862, -1166), (14934650266808366570, -1140),
    (11127181549972568877, -1113), (16580792590934885855, -1087), (12353653155963782858, -1060), (18408377700990114895, -1034),
    (13715310171984221708, -1007), (10218702384817765436, -980), (15227053142812498563, -954), (11345038669416679861, -927),
    (16905424996341287883, -901), (12595523146049147757, -874), (9384396036005875287, -847), (13983839803942852151, -821),
    (10418772551374772303, -794), (15525180923007089351, -768), (11567161174868858868, -741), (17236413322193710309, -715),
    (12842128665889583758, -688), (9568131466127621947, -661), (14257626930069360058, -635), (10622759856335341974, -608),
    (15829145694278690180, -582), (11793632577567316726, -555), (17573882009934360870, -529), (13093562431584567480, -502),
    (9755464219737475723, -475), (14536774485912137811, -449), (10830740992659433045, -422), (16139061738043178685, -396),
    (12024538023802026127, -369), (17917957937422433684, -343), (13349918974505688015, -316), (9946464728195732843, -289),
    (14821387422376473014, -263), (11042794154864902060, -236), (16455045573212060422, -210), (12259964326927110867, -183),
    (18268770466636286478, -157), (13611294676837538539, -130), (10141204801825835212, -103), (15111572745182864684, -77),
    (11258999068426240000, -50), (16777216000000000000, -24), (12500000000000000000, 3), (9313225746154785156, 30),
    (13877787807814456755, 56), (10339757656912845936, 83), (15407439555097886824, 109), (11479437019748901445, 136),
    (17105694144590052135, 162), (12744735289059618216, 189), (9495567745759798747, 216), (14149498560666738074, 242),
    (10542197943230523224, 269), (15709099088952724970, 295), (11704190886730495818, 322), (17440603504673385349, 348),
    (12994262207056124023, 375), (9681479787123295682, 402), (14426529090290212157, 428), (10748601772107342003, 455),
    (16016664761464807395, 481), (11933345169920330789, 508), (17782069995880619868, 534), (13248674568444952270, 561),
    (9871031767461413346, 588), (14708983551653345445, 614), (10959046745042015199, 641), (16330252207878254650, 667),
    (12166986024289022870, 694), (18130221999122236476, 720), (13508068024458167312, 747), (10064294952495520794, 774),
    (14996968138956309548, 800), (11173611982879273257, 827), (16649979327439178909, 853), (12405201291620119593, 880),
    (9242595204427927429, 907), (13772540099066387757, 933), (10261342003245940623, 960), (15290591125556738113, 986),
    (11392378155556871081, 1013), (16975966327722178521, 1039), (12648080533535911531, 1066),
];
const FIRST_POWER: i32 = -348;
const STEP_POWERS: i32 = 8;
const EXP_MIN: i32 = -60;
const EXP_MAX: i32 = -32;

const FRAC_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
const EXP_MASK: u64 = 0x7FF0_0000_0000_0000;
const HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;
const EXP_BIAS: i32 = 1023 + 52;

/// `fpconv_dtoa` for a finite `value`, written into `dest` (the C version's `char dest[32]`).
pub(super) fn dtoa(value: f64, dest: &mut [u8; 32]) -> &str {
    let mut out = &mut dest[..];
    if value.is_sign_negative() {
        push(&mut out, b"-");
    }
    if value == 0.0 {
        push(&mut out, b"0.0");
    } else {
        let mut digits = [0; 18];
        let (ndigits, k) = grisu2(value, &mut digits);
        emit_digits(&digits[..ndigits], k, value.is_sign_negative(), &mut out);
    }
    let len = 32 - out.len();
    std::str::from_utf8(&dest[..len]).expect("ASCII")
}

fn push(out: &mut &mut [u8], bytes: &[u8]) {
    out.write_all(bytes).expect("fpconv_dtoa writes at most 32 bytes");
}

fn push_zeros(out: &mut &mut [u8], count: usize) {
    push(out, &[b'0'; 32][..count]);
}

/// Writes `digits * 10^k`: plainly below 1e15 (with `.0` when integral) and down to 1e-9, in `e`
/// notation otherwise.
fn emit_digits(digits: &[u8], k: i32, negative: bool, out: &mut &mut [u8]) {
    let ndigits = digits.len() as i32;
    let exp = (k + ndigits - 1).abs();

    if k >= 0 && exp < 15 {
        push(out, digits);
        push_zeros(out, k as usize);
        push(out, b".0");
        return;
    }

    if k < 0 && (k > -7 || exp < 10) {
        let offset = ndigits + k;
        if offset <= 0 {
            push(out, b"0.");
            push_zeros(out, (-offset) as usize);
            push(out, digits);
        } else {
            let (whole, fraction) = digits.split_at(offset as usize);
            push(out, whole);
            push(out, b".");
            push(out, fraction);
        }
        return;
    }

    let digits = &digits[..digits.len().min(18 - usize::from(negative))];
    push(out, &digits[..1]);
    if digits.len() > 1 {
        push(out, b".");
        push(out, &digits[1..]);
    }
    push(out, if k + digits.len() as i32 - 1 < 0 { b"e-" } else { b"e+" });
    write!(out, "{exp}").expect("fpconv_dtoa writes at most 32 bytes");
}

/// The digits of `value` (ASCII, into `digits`) and their count, with `value = digits * 10^k`.
fn grisu2(value: f64, digits: &mut [u8; 18]) -> (usize, i32) {
    let w = build_fp(value);
    let (lower, upper) = normalized_boundaries(w);
    let w = normalize(w);

    let (cached, k) = cached_pow10(upper.exp);
    let w = multiply(w, cached);
    let mut upper = multiply(upper, cached);
    let mut lower = multiply(lower, cached);
    lower.frac += 1;
    upper.frac -= 1;

    generate_digits(w, upper, lower, digits, -k)
}

fn build_fp(value: f64) -> Fp {
    let bits = value.to_bits();
    let frac = bits & FRAC_MASK;
    match ((bits & EXP_MASK) >> 52) as i32 {
        0 => Fp { frac, exp: 1 - EXP_BIAS },
        exp => Fp {
            frac: frac + HIDDEN_BIT,
            exp: exp - EXP_BIAS,
        },
    }
}

fn normalize(mut fp: Fp) -> Fp {
    while fp.frac & HIDDEN_BIT == 0 {
        fp.frac <<= 1;
        fp.exp -= 1;
    }
    let shift = 64 - 52 - 1;
    Fp {
        frac: fp.frac << shift,
        exp: fp.exp - shift,
    }
}

/// The lower and upper ends of the rounding interval around `fp`, sharing the upper's exponent.
fn normalized_boundaries(fp: Fp) -> (Fp, Fp) {
    let mut upper = Fp {
        frac: (fp.frac << 1) + 1,
        exp: fp.exp - 1,
    };
    while upper.frac & (HIDDEN_BIT << 1) == 0 {
        upper.frac <<= 1;
        upper.exp -= 1;
    }
    let u_shift = 64 - 52 - 2;
    upper.frac <<= u_shift;
    upper.exp -= u_shift;

    let l_shift = if fp.frac == HIDDEN_BIT { 2 } else { 1 };
    let lower = Fp {
        frac: ((fp.frac << l_shift) - 1) << (fp.exp - l_shift - upper.exp),
        exp: upper.exp,
    };
    (lower, upper)
}

/// The high 64 bits of the product, rounded half up (the C version's four 32-bit multiplies).
fn multiply(a: Fp, b: Fp) -> Fp {
    let product = u128::from(a.frac) * u128::from(b.frac) + (1 << 63);
    Fp {
        frac: (product >> 64) as u64,
        exp: a.exp + b.exp + 64,
    }
}

/// The cached power that brings a number with binary exponent `exp` into [-60, -32], and its
/// decimal exponent.
fn cached_pow10(exp: i32) -> (Fp, i32) {
    const ONE_LOG_TEN: f64 = 0.301_029_995_663_981_14;
    let approx = (f64::from(-(exp + POWERS_TEN.len() as i32)) * ONE_LOG_TEN) as i32;
    let mut idx = (approx - FIRST_POWER) / STEP_POWERS;
    loop {
        let (frac, power_exp) = POWERS_TEN[idx as usize];
        let current = exp + power_exp + 64;
        if current < EXP_MIN {
            idx += 1;
        } else if current > EXP_MAX {
            idx -= 1;
        } else {
            return (Fp { frac, exp: power_exp }, FIRST_POWER + idx * STEP_POWERS);
        }
    }
}

fn generate_digits(w: Fp, upper: Fp, lower: Fp, digits: &mut [u8; 18], mut k: i32) -> (usize, i32) {
    let wfrac = upper.frac - w.frac;
    let mut delta = upper.frac - lower.frac;
    let shift = -upper.exp;
    let one = 1u64 << shift;
    let mut part1 = upper.frac >> shift;
    let mut part2 = upper.frac & (one - 1);
    let mut idx = 0;
    let mut kappa = 10;

    // The integral part, from 10^9 down.
    while kappa > 0 {
        let div = 10u64.pow(kappa as u32 - 1);
        let digit = part1 / div;
        if digit != 0 || idx != 0 {
            digits[idx] = b'0' + digit as u8;
            idx += 1;
        }
        part1 -= digit * div;
        kappa -= 1;

        let rem = (part1 << shift) + part2;
        if rem <= delta {
            k += kappa;
            round_digit(&mut digits[..idx], delta, rem, div << shift, wfrac);
            return (idx, k);
        }
    }

    // The fractional part.
    let mut unit = 10;
    loop {
        part2 *= 10;
        delta *= 10;
        kappa -= 1;

        let digit = part2 >> shift;
        if digit != 0 || idx != 0 {
            digits[idx] = b'0' + digit as u8;
            idx += 1;
        }
        part2 &= one - 1;
        if part2 < delta {
            k += kappa;
            round_digit(&mut digits[..idx], delta, part2, one, wfrac * unit);
            return (idx, k);
        }
        unit *= 10;
    }
}

/// Moves the last digit down while that brings the number closer to the true value and stays in
/// the interval.
fn round_digit(digits: &mut [u8], delta: u64, mut rem: u64, kappa: u64, frac: u64) {
    let last = digits.last_mut().expect("at least one digit");
    while rem < frac && delta - rem >= kappa && (rem + kappa < frac || frac - rem > rem + kappa - frac) {
        *last -= 1;
        rem += kappa;
    }
}
