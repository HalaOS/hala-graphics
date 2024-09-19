use std::hint::black_box;

use divan::Bencher;
use ecsrs::{ComponentType, World};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn ecs_entity_new(bencher: Bencher) {
    let mut world = World::new(&[ComponentType::new(1)]);

    bencher.bench_local(|| black_box(world.new_entity()));
}

#[divan::bench]
fn ecs_entity_attach_component(bencher: Bencher) {
    let c1 = ComponentType::new(1);
    let mut world = World::new(&[c1]);

    let entities = [
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
    ]
    .to_vec();

    bencher.bench_local(|| black_box(world.new_component_with(&c1, 1, &entities)));
}

#[divan::bench]
fn ecs_entity_get_component(bencher: Bencher) {
    let c1 = ComponentType::new(1);
    let mut world = World::new(&[c1]);

    let entities = [
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
    ]
    .to_vec();

    let (component_id, _) = world.new_component_with(&c1, 1usize, &entities);

    bencher.bench_local(|| black_box(world.component_ref_unchecked::<usize>(&component_id)));
}

#[divan::bench]
fn ecs_entity_component_iter(bencher: Bencher) {
    let c1 = ComponentType::new(1);
    let mut world = World::new(&[c1]);

    let entities = [
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
        world.new_entity(),
    ]
    .to_vec();

    let (_, _) = world.new_component_with(&c1, 1usize, &entities);

    bencher.bench_local(|| black_box(world.component_iter_with::<usize>(&c1).next()));
}
