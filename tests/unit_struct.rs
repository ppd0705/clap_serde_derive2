use clap_serde_derive2::ClapSerde;

#[derive(ClapSerde)]
struct Unit;

#[test]
fn unit_struct() {
    Unit::from_clap();
    Unit::from(<Unit as ClapSerde>::Opt::default()).merge_clap();
}
