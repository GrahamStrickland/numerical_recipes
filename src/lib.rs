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

#[cfg(test)]
mod tests {
    use super::flmoon;

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
}
