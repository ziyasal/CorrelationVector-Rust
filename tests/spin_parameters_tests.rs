use correlation_vector;
use correlation_vector::enums::SpinEntropy;

#[test]
fn it_sets_entropy() {
    let  mut sp = correlation_vector::SpinParameters::default();

    sp.set_entropy(Box::new(SpinEntropy::Four));

    assert_eq!(4, *sp.entropy() as i32);
}