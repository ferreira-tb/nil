use nil_core::{Coord, World};

#[test]
fn cell() {
  each_coord(|world, coord| {
    assert!(world.cell(coord).is_ok());
  });
}

#[test]
fn index_to_coord() {
  each_coord(|world, coord| {
    let index = world.index(coord);
    assert_eq!(coord, world.coord(index).unwrap());
  });
}

fn each_coord<F>(f: F)
where
  F: Fn(&mut World, Coord),
{
  let mut world = World::default();
  (0..100).into_iter().for_each(|x| {
    (0..100).into_iter().for_each(|y| {
      f(&mut world, Coord::new(x, y));
    });
  })
}
