fn main() {
    use com::{
        interfaces::iclass_factory::IClassFactory,
        interfaces::iunknown::IUnknown,
        runtime::{get_class_object, init_apartment, ApartmentType},
    };
    use unzip_com::{CLSID_CLASS, IUnzip};
    // Initialize the COM apartment
    init_apartment(ApartmentType::SingleThreaded)
        .unwrap_or_else(|hr| panic!("Failed to initialize COM Library{:x}", hr));
    println!("Initialized apartment");

    // Get a `BritishShortHairCat` class factory
    let factory = get_class_object::<IClassFactory>(&CLSID_CLASS)
        .unwrap_or_else(|hr| panic!("Failed to get cat class object 0x{:x}", hr));
    println!("Got cat class object");

    // Get an instance of a `BritishShortHairCat` as the `IUnknown` interface
    let unknown = factory
        .create_instance::<IUnknown>()
        .expect("Failed to get IUnknown");
    println!("Got IUnknown");

    // Now get a handle to the `IAnimal` interface
    let unzip = unknown
        .query_interface::<IUnzip>()
        .expect("Failed to get IUnzip");
    unsafe {
        unzip.Unzip("6_WNR.gz\0".as_ptr() as *const libc::c_char, "6_WNR.bmp\0".as_ptr() as *const libc::c_char);
    }
}
