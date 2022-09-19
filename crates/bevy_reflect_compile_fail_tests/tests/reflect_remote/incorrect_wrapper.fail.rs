use bevy_reflect::Reflect;

mod external_crate {
    pub struct TheirFoo {
        pub value: u32,
    }
}

#[repr(transparent)]
#[derive(Reflect)]
#[reflect(from_reflect = false)]
struct MyFoo(#[reflect(ignore)] pub external_crate::TheirFoo);

#[derive(Reflect)]
#[reflect(from_reflect = false)]
struct MyStruct {
    // Reason: `MyFoo` does not implement `ReflectRemote` (from `#[reflect_remote]` attribute)
    #[reflect(remote = "MyFoo")]
    foo: external_crate::TheirFoo,
}

fn main() {}
