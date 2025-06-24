use rprime::{pollard_rho, wheel_factorize};

#[test]
fn test_factors_of_prime() {
    assert_eq!(wheel_factorize(13).unwrap(), vec![13]);
}

#[test]
fn test_factors_of_composite() {
    assert_eq!(wheel_factorize(28).unwrap(), vec![2, 2, 7]);
}

#[test]
fn test_factors_of_one() {
    assert_eq!(wheel_factorize(1).unwrap(), vec![]);
}

#[test]
fn test_factors_of_zero() {
    assert!(wheel_factorize(0).is_err());
}

#[test]
fn test_factors_of_large_number() {
    assert_eq!(wheel_factorize(360).unwrap(), vec![2, 2, 2, 3, 3, 5]);
}

#[test]
fn test_factors_of_large_prime() {
    assert_eq!(wheel_factorize(1012345678901).unwrap(), vec![1012345678901])
}

#[test]
fn test_large_prime_factors() {
    assert_eq!(wheel_factorize(1061036293492688417).unwrap(), vec![1028910301, 1031223317])
}

#[test]
fn test_large_prime_factors_pr() {
    assert_eq!(pollard_rho(1061036293492688417).unwrap(), vec![1028910301, 1031223317])
}