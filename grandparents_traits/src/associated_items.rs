//! Associated types, associated constants, and generic trait parameters.

#[derive(Clone, Debug)]
struct WorkItem {
    id: u32,
    title: &'static str,
}

trait Repository {
    type Item;

    fn get(&self, id: u32) -> Option<Self::Item>;
}

struct WorkItemRepository {
    items: Vec<WorkItem>,
}

impl Repository for WorkItemRepository {
    type Item = WorkItem;

    fn get(&self, id: u32) -> Option<Self::Item> {
        self.items.iter().find(|item| item.id == id).cloned()
    }
}

trait TaxPolicy {
    const RATE: f32;

    fn tax_for(amount: f32) -> f32 {
        amount * Self::RATE
    }
}

struct StandardTax;

impl TaxPolicy for StandardTax {
    const RATE: f32 = 0.20;
}

trait ConvertTo<T> {
    fn convert(self) -> T;
}

#[derive(Clone, Copy)]
struct Hours(f32);

#[derive(Clone, Copy)]
struct Days(f32);

impl ConvertTo<Days> for Hours {
    fn convert(self) -> Days {
        Days(self.0 / 8.0)
    }
}

fn print_lookup<R>(repository: &R, id: u32)
where
    R: Repository<Item = WorkItem>,
{
    match repository.get(id) {
        Some(item) => println!("associated type lookup: #{} {}", item.id, item.title),
        None => println!("associated type lookup: #{id} not found"),
    }
}

pub fn run() {
    println!("\n== Associated items and generic traits ==");

    let repository = WorkItemRepository {
        items: vec![
            WorkItem {
                id: 1,
                title: "define trait shape",
            },
            WorkItem {
                id: 2,
                title: "implement trait",
            },
        ],
    };
    print_lookup(&repository, 2);

    let tax = <StandardTax as TaxPolicy>::tax_for(1_000.0);
    println!("associated const and default method: tax = ${tax:.2}");

    let effort = Hours(16.0);
    let duration: Days = effort.convert();
    println!(
        "generic trait parameter: {:.1} hours = {:.1} days",
        effort.0, duration.0
    );
}
