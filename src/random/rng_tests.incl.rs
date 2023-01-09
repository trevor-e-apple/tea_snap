use std::collections::HashSet;

/// a basic test to make sure that the sequences aren't the same
#[test]
fn sequence_diff() {
    let mut generator_one = instance(0);
    let mut generator_two = instance(1);

    let mut sequence_one: Vec<u32> = Vec::new();
    for _ in 0..10 {
        sequence_one.push(generator_one.rand());
    }

    let mut sequence_two: Vec<u32> = Vec::new();
    for _ in 0..10 {
        sequence_two.push(generator_two.rand());
    }

    assert!(sequence_one != sequence_two);
}

/// test that having the same seed produces the same sequence
#[test]
fn same_seed() {
    let mut generator_one = instance(0);
    let mut generator_two = instance(0);

    let mut sequence_one: Vec<u32> = Vec::new();
    for _ in 0..100 {
        sequence_one.push(generator_one.rand());
    }

    let mut sequence_two: Vec<u32> = Vec::new();
    for _ in 0..100 {
        sequence_two.push(generator_two.rand());
    }

    assert!(sequence_one == sequence_two);
}

/// test that we can generate the whole range of random values
#[test]
fn rand_u8_full_range() {
    const MAX_ATTEMPTS: u32 = 20000;
    let mut generator = instance(0);

    let mut attempts = 0;
    let mut set = HashSet::new();
    while attempts < MAX_ATTEMPTS {
        let value = generator.rand_u8(0, u8::MAX);
        set.insert(value);
        if (set.len() as u8) == u8::MAX {
            break;
        }

        attempts += 1;
    }

    assert!(attempts < MAX_ATTEMPTS);
}

/// test that we can generate the partial range of random values
#[test]
fn rand_u8_partial_range() {
    const MAX_ATTEMPTS: u32 = 20000;
    const START: u8 = u8::MAX / 4;
    const END: u8 = 3 * (u8::MAX / 4);
    let mut generator = instance(0);

    let mut attempts = 0;
    let mut set = HashSet::new();
    while attempts < MAX_ATTEMPTS {
        let value = generator.rand_u8(START, END);
        assert!(value >= START);
        assert!(value < END);

        set.insert(value);
        if (set.len() as u8) == (END - START) {
            break;
        }

        attempts += 1;
    }

    assert!(attempts < MAX_ATTEMPTS);
}

/// test to see if 
#[test]
fn sample_discrete() {
    let mut generator_one = instance(0);
    let mut generator_two = instance(1);

    let distribution: Vec<f32> = vec![0.1, 0.4, 0.3, 0.2];

    let mut sequence_one: Vec<usize> = Vec::new();
    for _ in 0..10 {
        sequence_one.push(generator_one.rand_sample_discrete(&distribution));
    }

    let mut sequence_two: Vec<usize> = Vec::new();
    for _ in 0..10 {
        sequence_two.push(generator_two.rand_sample_discrete(&distribution));
    }

    assert!(sequence_one != sequence_two);
}