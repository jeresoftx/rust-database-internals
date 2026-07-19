#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MaintenanceCase {
    operation: &'static str,
    touched_indexes: &'static [&'static str],
    duplicate_check: bool,
}

fn main() {
    let cases = [
        MaintenanceCase {
            operation: "insert customer",
            touched_indexes: &[
                "pk_customers",
                "uq_customers_email",
                "idx_customers_country",
            ],
            duplicate_check: true,
        },
        MaintenanceCase {
            operation: "update email",
            touched_indexes: &["uq_customers_email"],
            duplicate_check: true,
        },
        MaintenanceCase {
            operation: "update country",
            touched_indexes: &["idx_customers_country"],
            duplicate_check: false,
        },
    ];

    assert_eq!(cases[0].touched_indexes.len(), 3);
    assert!(cases[1].duplicate_check);
    assert!(!cases[2].duplicate_check);

    for case in cases {
        println!(
            "{} -> {:?}, duplicate_check={}",
            case.operation, case.touched_indexes, case.duplicate_check
        );
    }
}
