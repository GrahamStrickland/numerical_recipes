pub fn flmoon<'a, 'b>(n: i32, nph: i32, mut jd: &'a mut i64, frac: &'b mut f64) {
    let c = f64::from(n + nph) / 4.0;
    let t = c / 1236.85;
    let t2 = t * t;
    let r#as = 359.2242 + 29.105356 * c;
    let am = 306.0253 + 385.816918 * c + 0.010730 * t2;

    *jd = 2415020 + 28 * i64::from(n) + 7 * i64::from(nph);
    let mut xtra = 0.75933 + 1.53058868 * c + ((1.178e-4) - (1.55e-7) * t) * t2;

    if nph == 0 || nph == 2 {
        xtra += (0.1734 - 3.93e-4 * t) * f64::sin(r#as.to_radians())
            - 0.4068 * f64::sin(am.to_radians());
    } else {
        xtra += (0.1721 - 4.0e-4 * t) * f64::sin(r#as.to_radians())
            - 0.6280 * f64::sin(am.to_radians());
    } // else throw error "nph is unknown in flmoon"

    let i = if xtra >= 0.0 {
        xtra.floor() as i64;
    } else {
        f64::from(xtra - 1.0).ceil() as i64;
    };

    *jd = jd + i;
    *frac = xtra - i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flmoon() {
        let result = flmoon(0, 0, 0.0, 0.0);
        assert_eq!(result, 4);
    }
}
