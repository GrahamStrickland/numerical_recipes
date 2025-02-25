/// This routine calculates the phase of the moon. Given an integer `n` and
/// a code `nph` for the phase desired (`nph = 0` for new moon, `1` for first quarter, `2` for
/// full, `3` for last quarter), the routine returns the Julian Day Number `julian_date`, and
/// the fractional part of a day `frac` to be added to it, of the `n`th such phase since January,
/// 1900. Greenwich Mean Time is assumed.
pub fn flmoon(n: i32, nph: i32) -> (i64, f64) {
    let c = f64::from(n + nph) / 4.0;
    let t = c / 1236.85;
    let t2 = t * t;
    let sun_mean_anomaly = 359.2242 + 29.105356 * c;
    let moon_mean_anomaly = 306.0253 + 385.816918 * c + 0.010730 * t2;

    let mut julian_date = 2415020 + 28 * i64::from(n) + 7 * i64::from(nph);
    let mut xtra = 0.75933 + 1.53058868 * c + ((1.178e-4) - (1.55e-7) * t) * t2;

    if nph == 0 || nph == 2 {
        xtra += (0.1734 - 3.93e-4 * t) * f64::sin(sun_mean_anomaly.to_radians())
            - 0.4068 * f64::sin(moon_mean_anomaly.to_radians());
    } else if nph == 1 || nph == 3 {
        xtra += (0.1721 - 4.0e-4 * t) * f64::sin(sun_mean_anomaly.to_radians())
            - 0.6280 * f64::sin(moon_mean_anomaly.to_radians());
    } else {
        panic!("nph is unknown in flmoon");
    }

    let i = match xtra >= 0.0 {
        true => xtra.floor() as i64,
        false => f64::from(xtra - 1.0).ceil() as i64,
    };

    julian_date += i;
    let frac = xtra - i as f64;

    (julian_date, frac)
}

const IGREG: i64 = 15 + 31 * (10 + 12 * 1582);

/// In this routine `julday` returns the Julian Day Number that begins at noon of the calendar date
/// specified by month `mm`, day `id`, and year `iyyy`, all integer variables. Positive year
/// signifies A.D.; negative, B.C. Remember that the year after 1 B.C. was 1 A.D.
pub fn julday(mm: i32, id: i32, iyyy: i32) -> i64 {
    let mut jul;
    let ja;
    let jm;
    let mut jy = iyyy;

    if jy == 0 {
        panic!("julday: there is no year zero.");
    }

    if jy < 0 {
        jy += 1;
    }

    if mm > 2 {
        jm = mm + 1;
    } else {
        jy -= 1;
        jm = mm + 13;
    }

    jul = f64::floor(365.25 * f64::from(jy))
        + f64::floor(30.6001 * f64::from(jm))
        + f64::from(id)
        + 1720995.0;

    if i64::from(id + 31 * (mm + 12 * iyyy)) >= IGREG {
        ja = (0.01 * f64::from(jy)) as i32;
        jul += 2.0 - f64::from(ja) + f64::floor(0.25 * f64::from(ja));
    }

    jul as i64
}

#[cfg(test)]
mod tests {
    use super::{flmoon, julday};

    #[test]
    fn test_flmoon() {
        let (julian_date, frac) = flmoon(42, 2);

        assert_eq!(julian_date, 2416227);
        assert_eq!(frac, 0.7946036184973124);

        let (julian_date, frac) = flmoon(21, 1);

        assert_eq!(julian_date, 2415624);
        assert_eq!(frac, 0.8660142820887664);
    }

    #[test]
    #[should_panic]
    fn test_flmoon_panic() {
        let (_, _) = flmoon(21, 5);
    }

    #[test]
    fn test_julday() {
        let mut res = julday(1, 28, 1991);

        assert_eq!(res, 2448285);

        res = julday(2, 25, 2025);

        assert_eq!(res, 2460732);
    }

    #[test]
    #[should_panic]
    fn test_julday_panic() {
        let _ = julday(0, 0, 0);
    }
}
