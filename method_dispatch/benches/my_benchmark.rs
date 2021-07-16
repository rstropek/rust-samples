use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait Animal {
    fn make_sound(&self) -> &'static str;
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> &'static str {
        "Miau"
    }
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> &'static str {
        "Wuff"
    }
}

fn check_if_barks(animal: &impl Animal) -> bool {
    animal.make_sound() == "Wuff"
}


fn check_if_barks_dyn(animal: &dyn Animal) -> bool {
    animal.make_sound() == "Wuff"
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("impl", |b| b.iter(|| {
        check_if_barks(black_box(&Cat {}));
        check_if_barks(black_box(&Dog {}));
    }));
    c.bench_function("dyn", |b| b.iter(|| {
        check_if_barks_dyn(black_box(&Cat {}));
        check_if_barks_dyn(black_box(&Dog {}));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
