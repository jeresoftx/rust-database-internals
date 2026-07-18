use rust_database_internals::btree::{BTree, Key, RecordPointer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Customer {
    id: u64,
    name: &'static str,
}

fn main() {
    let page = [
        Customer {
            id: 1001,
            name: "Ada",
        },
        Customer {
            id: 1002,
            name: "Grace",
        },
        Customer {
            id: 1003,
            name: "Edsger",
        },
        Customer {
            id: 1004,
            name: "Barbara",
        },
    ];
    let mut primary_key_index =
        BTree::new(3).expect("max_keys_per_node=3 debe crear un B-Tree válido");

    for (slot_id, customer) in page.iter().enumerate() {
        primary_key_index
            .insert(
                Key::new(customer.id),
                RecordPointer {
                    page_id: 0,
                    slot_id: slot_id as u16,
                },
            )
            .expect("cada customer.id debe ser una primary key única");
    }

    let pointer = primary_key_index
        .search(Key::new(1002))
        .expect("buscar en el índice no debe fallar")
        .expect("customer_id=1002 debe existir");
    let customer = page[pointer.slot_id as usize];

    assert_eq!(customer.name, "Grace");
    assert_eq!(primary_key_index.height(), 2);

    println!(
        "customer_id=1002 -> page_id={} slot_id={} name={}",
        pointer.page_id, pointer.slot_id, customer.name
    );
}
