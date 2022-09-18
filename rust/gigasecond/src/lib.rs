use time::Duration;
use time::PrimitiveDateTime as DateTime;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(10i64.pow(9))
}
