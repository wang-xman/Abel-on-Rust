//! unittest for crate complex
use super::*;

mod test_complex_constructor {
    use super::*;

    #[test]
    fn test_constructor() {
        let test_value = Complex::new(0.5, 1.0);
        assert_eq!(test_value.imag, 1.0);
        assert_eq!(test_value.real, 0.5);
        assert_eq!( test_value.value(), (0.5, 1.0) );
    }
}

mod test_complex_partial_eq {
    use super::*;

    #[test]
    fn test_partialeq() {
        let test_complex = Complex::new(0.5, 0.0);
        let c2 = Complex::new(0.5, 1.0);

        let test_double = Double::new(0.5);
        let test_int = Integer::new(1);
        // Complex == Complex
        assert_ne!(test_complex, c2);
        assert!(!(test_complex == c2));
        // Complex == Double
        assert_eq!(test_complex, test_double);
        // Complex == f64
        assert_eq!(test_complex, 0.5);
        // Complex == Integer
        assert!(!(test_complex == test_int));
        // Complex == i32
        assert!(!(test_complex == 1));

        // Double == Complex
        assert_eq!(test_double, test_complex);
        // f64 == Complex
        assert_eq!(0.5, test_complex);
        // Integer == Complex
        assert!(!(test_int == test_complex));
        // i32 == Complex
        assert!(!(100 == test_complex));
    }
}

mod test_complex_verifier {
    use super::*;

    #[test]
    fn test_is_real() {
        let creal = Complex::new(0.5, 0.0);
        let cimag = Complex::new(0.0, 1.0);
        let czero = Complex::new(0.0, 0.0);
        assert!(creal.is_real());
        assert!(!cimag.is_real());
        assert!(czero.is_real());
    }

    #[test]
    fn test_is_imag() {
        let creal = Complex::new(0.5, 0.0);
        let cimag = Complex::new(0.0, 1.0);
        let czero = Complex::new(0.0, 0.0);
        assert!(!creal.is_imag());
        assert!(cimag.is_imag());
        assert!(!czero.is_imag());
    }

    #[test]
    fn test_is_zero() {
        let creal = Complex::new(0.5, 0.0);
        let cimag = Complex::new(0.0, 1.0);
        let czero = Complex::new(0.0, 0.0);
        assert!(!creal.is_zero());
        assert!(!cimag.is_zero());
        assert!(czero.is_zero());
    }
}