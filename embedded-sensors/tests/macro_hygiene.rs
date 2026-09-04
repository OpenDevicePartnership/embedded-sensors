use embedded_sensors_hal::decl_threshold_traits;
use embedded_sensors_hal::sensor::ErrorType;

struct TestSensor;

impl ErrorType for TestSensor {
    type Error = core::convert::Infallible;
}

trait TestSensorTrait: ErrorType<Error = core::convert::Infallible> {}

impl TestSensorTrait for TestSensor {}

impl<T: TestSensorTrait + ?Sized> TestSensorTrait for &mut T {}

decl_threshold_traits!(blocking, TestSensor, TestSensorTrait, f32, "units");

#[test]
fn exported_macro_works_without_a_call_site_paste_dependency() {
    let _ = core::mem::size_of::<TestSensor>();
}
