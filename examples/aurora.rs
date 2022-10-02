use glam::Vec2;
use polyanya::Mesh;

macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        let val = $x;
        if !((val.length - $y).abs() < 0.001) {
            assert_eq!(val.length, $y);
        }
    };
}

fn main() {
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
    )
    .expect("set up the subscriber");

    let mesh = Mesh::from_file("meshes/aurora.mesh".into());

    // for i in 0..1000 {
    //     // print!("does it work? {}", i);
    //     mesh.path(Vec2::new(i  as f32, i as f32), Vec2::new(100.0, 100.0));
    //     // println!("{}", i);
    // }

    // assert_delta!(
    //     mesh.path(Vec2::new(993.0, 290.0), Vec2::new(34.0, 622.0))
    //         .unwrap(),
    //     1123.2226
    // );

    assert_delta!(
        mesh.path(Vec2::new(116.0, 422.0), Vec2::new(733.0, 267.0))
            .unwrap(),
        763.463
    );
}
