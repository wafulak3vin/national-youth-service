#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// NYS Unit struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct NYSUnit {
    id: u64,
    name: String,
    location: String,
    contact: String,
    capacity: u64, // Maximum recruits it can accommodate
    created_at: u64,
}

// Recruit struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Recruit {
    id: u64,
    nys_unit_id: u64,
    name: String,
    age: u64,
    gender: String,
    skills: Vec<String>,
    training_status: String, // "Enrolled", "In Progress", "Completed"
    created_at: u64,
}

// Project struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Project {
    id: u64,
    nys_unit_id: u64,
    name: String,
    description: String,
    budget: f64,
    start_date: u64,
    end_date: u64,
    status: String, // "Planned", "Ongoing", "Completed"
}

// Vehicle struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vehicle {
    id: u64,
    nys_unit_id: u64,
    registration_number: String,
    model: String,
    capacity: u64,
    status: String, // "Available", "In Use", "Under Maintenance"
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    nys_unit_id: u64,
    category: String,
    amount: f64,
    description: String,
    date: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateNYSUnitPayload {
    name: String,
    location: String,
    contact: String,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterRecruitPayload {
    nys_unit_id: u64,
    name: String,
    age: u64,
    gender: String,
    skills: Vec<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateProjectPayload {
    nys_unit_id: u64,
    name: String,
    description: String,
    budget: f64,
    start_date: u64,
    end_date: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterVehiclePayload {
    nys_unit_id: u64,
    registration_number: String,
    model: String,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    nys_unit_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for NYSUnit
impl Storable for NYSUnit {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for NYSUnit {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Recruit
impl Storable for Recruit {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Recruit {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Project
impl Storable for Project {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Project {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Vehicle
impl Storable for Vehicle {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Vehicle {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static NYS_UNITS: RefCell<StableBTreeMap<u64, NYSUnit, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static RECRUITS: RefCell<StableBTreeMap<u64, Recruit, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static PROJECTS: RefCell<StableBTreeMap<u64, Project, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static VEHICLES: RefCell<StableBTreeMap<u64, Vehicle, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));
}

// Functions

// Create NYS Unit
#[ic_cdk::update]
fn create_nys_unit(payload: CreateNYSUnitPayload) -> Result<NYSUnit, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let nys_unit_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let nys_unit = NYSUnit {
        id: nys_unit_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        capacity: payload.capacity,
        created_at: time(),
    };

    NYS_UNITS.with(|units| {
        units.borrow_mut().insert(nys_unit_id, nys_unit.clone());
    });

    Ok(nys_unit)
}

// Register Recruit
#[ic_cdk::update]
fn register_recruit(payload: RegisterRecruitPayload) -> Result<Recruit, Message> {
    if payload.name.is_empty() || payload.gender.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let unit_exists = NYS_UNITS.with(|units| units.borrow().contains_key(&payload.nys_unit_id));
    if !unit_exists {
        return Err(Message::NotFound("NYS Unit not found".to_string()));
    }

    let recruit_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let recruit = Recruit {
        id: recruit_id,
        nys_unit_id: payload.nys_unit_id,
        name: payload.name,
        age: payload.age,
        gender: payload.gender,
        skills: payload.skills,
        training_status: "Enrolled".to_string(),
        created_at: time(),
    };

    RECRUITS.with(|recruits| {
        recruits.borrow_mut().insert(recruit_id, recruit.clone());
    });

    Ok(recruit)
}

// Create Project
#[ic_cdk::update]
fn create_project(payload: CreateProjectPayload) -> Result<Project, Message> {
    if payload.name.is_empty() || payload.budget <= 0.0 {
        return Err(Message::InvalidPayload("Invalid project data".to_string()));
    }

    let unit_exists = NYS_UNITS.with(|units| units.borrow().contains_key(&payload.nys_unit_id));
    if !unit_exists {
        return Err(Message::NotFound("NYS Unit not found".to_string()));
    }

    let project_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let project = Project {
        id: project_id,
        nys_unit_id: payload.nys_unit_id,
        name: payload.name,
        description: payload.description,
        budget: payload.budget,
        start_date: payload.start_date,
        end_date: payload.end_date,
        status: "Planned".to_string(),
    };

    PROJECTS.with(|projects| {
        projects.borrow_mut().insert(project_id, project.clone());
    });

    Ok(project)
}

// Register Vehicle
#[ic_cdk::update]
fn register_vehicle(payload: RegisterVehiclePayload) -> Result<Vehicle, Message> {
    if payload.registration_number.is_empty() || payload.model.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let unit_exists = NYS_UNITS.with(|units| units.borrow().contains_key(&payload.nys_unit_id));
    if !unit_exists {
        return Err(Message::NotFound("NYS Unit not found".to_string()));
    }

    let vehicle_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let vehicle = Vehicle {
        id: vehicle_id,
        nys_unit_id: payload.nys_unit_id,
        registration_number: payload.registration_number,
        model: payload.model,
        capacity: payload.capacity,
        status: "Available".to_string(),
    };

    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(vehicle_id, vehicle.clone());
    });

    Ok(vehicle)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload("Invalid expense amount".to_string()));
    }

    let unit_exists = NYS_UNITS.with(|units| units.borrow().contains_key(&payload.nys_unit_id));
    if !unit_exists {
        return Err(Message::NotFound("NYS Unit not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        nys_unit_id: payload.nys_unit_id,
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
        date: time(),
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(nys_unit_id: u64) -> Result<f64, Message> {
    let unit_exists = NYS_UNITS.with(|units| units.borrow().contains_key(&nys_unit_id));
    if !unit_exists {
        return Err(Message::NotFound("NYS Unit not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.nys_unit_id == nys_unit_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

// Exporting the candid interface
ic_cdk::export_candid!();
